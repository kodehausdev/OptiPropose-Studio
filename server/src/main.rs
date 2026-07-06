use axum::{
    extract::State,
    http::{HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
    resend_api_key: Option<String>,
    from_email: String,
    to_email: String,
}

#[derive(Deserialize)]
struct BriefPayload {
    building: String,
    metric: String,
    resources: String,
    timeline: String,
    email: String,
    company: Option<String>,
}

#[derive(Serialize)]
struct BriefResponse {
    ok: bool,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "briefs.db".to_string());
    let conn = Connection::open(&db_path).expect("failed to open sqlite database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS briefs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            building TEXT NOT NULL,
            metric TEXT NOT NULL,
            resources TEXT NOT NULL,
            timeline TEXT NOT NULL,
            email TEXT NOT NULL,
            company TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .expect("failed to create briefs table");

    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        resend_api_key: std::env::var("RESEND_API_KEY").ok(),
        from_email: std::env::var("FROM_EMAIL")
            .unwrap_or_else(|_| "brief@optipropose.com".to_string()),
        to_email: std::env::var("TO_EMAIL")
            .unwrap_or_else(|_| "hello@optipropose.com".to_string()),
    };

    let cors = build_cors_layer();

    let app = Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/brief", post(submit_brief))
        .layer(cors)
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8787".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind port");
    tracing::info!("optipropose-server listening on {addr}");
    axum::serve(listener, app).await.expect("server error");
}

fn build_cors_layer() -> CorsLayer {
    match std::env::var("ALLOWED_ORIGIN") {
        Ok(origins) => {
            let list: Vec<HeaderValue> = origins
                .split(',')
                .filter_map(|o| o.trim().parse().ok())
                .collect();
            CorsLayer::new()
                .allow_origin(list)
                .allow_methods([Method::POST, Method::GET])
                .allow_headers(tower_http::cors::Any)
        }
        Err(_) => CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods([Method::POST, Method::GET])
            .allow_headers(tower_http::cors::Any),
    }
}

async fn submit_brief(
    State(state): State<AppState>,
    Json(payload): Json<BriefPayload>,
) -> (StatusCode, Json<BriefResponse>) {
    if payload.building.trim().is_empty()
        || payload.metric.trim().is_empty()
        || payload.resources.trim().is_empty()
        || payload.timeline.trim().is_empty()
        || payload.email.trim().is_empty()
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(BriefResponse {
                ok: false,
                error: Some("missing required field".to_string()),
            }),
        );
    }

    let insert_result = {
        let conn = state.db.lock().expect("db mutex poisoned");
        conn.execute(
            "INSERT INTO briefs (building, metric, resources, timeline, email, company) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                payload.building,
                payload.metric,
                payload.resources,
                payload.timeline,
                payload.email,
                payload.company,
            ],
        )
    };

    if let Err(e) = insert_result {
        tracing::error!("failed to store brief: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(BriefResponse {
                ok: false,
                error: Some("failed to store brief".to_string()),
            }),
        );
    }

    send_email_notification(&state, &payload).await;

    (
        StatusCode::OK,
        Json(BriefResponse {
            ok: true,
            error: None,
        }),
    )
}

async fn send_email_notification(state: &AppState, payload: &BriefPayload) {
    let Some(api_key) = state.resend_api_key.clone() else {
        tracing::warn!("RESEND_API_KEY not set, skipping email notification");
        return;
    };

    let company = payload.company.clone().unwrap_or_else(|| "—".to_string());
    let subject = if payload.company.is_some() {
        format!("New Brief — {company}")
    } else {
        "New Brief".to_string()
    };
    let text = format!(
        "What we're building: {}\nMetric that moves: {}\nBudget + data: {}\nTimeline: {}\nEmail: {}\nCompany: {}",
        payload.building, payload.metric, payload.resources, payload.timeline, payload.email, company
    );

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "from": state.from_email,
        "to": [state.to_email],
        "subject": subject,
        "text": text,
        "reply_to": payload.email,
    });

    let result = client
        .post("https://api.resend.com/emails")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await;

    match result {
        Ok(resp) if !resp.status().is_success() => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            tracing::error!("resend send failed: {status} {body}");
        }
        Err(e) => tracing::error!("resend request failed: {e}"),
        _ => {}
    }
}
