# Optipropose Studio

The public site for **Optipropose Studio** (RC 9335843, Abuja, NG) — an independent
engineering studio shipping production-grade systems: logistics pipelines, AI order
structuring, and enterprise HR platforms.

**Live at [optipropose.com](https://optipropose.com).**

## How it's built

Deliberately boring, deliberately fast:

- **Frontend** — a single self-contained page (`Optipropose Studio.dc.html`) in the
  Design Component format: inline styles, no build step, no framework. `support.js`
  is the runtime. Opens directly in a browser.
- **Backend** (`server/`) — a small Rust/axum service that receives contact-form
  briefs, stores them in SQLite, and forwards a notification email via Resend.

## Deploy pipeline

Push to `main` deploys both halves automatically:

| Target | Platform | What |
|---|---|---|
| `optipropose.com` | Vercel | the static site (repo root) |
| brief API | Railway | `server/` via its Dockerfile |

`www.` and `studio.` 308-redirect to the apex. The API's CORS is locked to
`https://optipropose.com`.

## Local dev

```bash
# frontend — any static server
npx http-server -p 5500
# open http://localhost:5500/Optipropose%20Studio.dc.html

# backend
cd server
cp .env.example .env   # fill in values
cargo run
```

Note: the deployed API only accepts browser submissions from the production origin,
so point the form at a local server (via the `briefApiUrl` prop) when testing locally.

## Future plans

- **Case studies / build logs** — deeper write-ups of TCDGO, CordHR and the MedLab
  receptionist than the product cards allow.
- **Stack migration (when earned, not before)** — the single-file format is the
  right tool while this is one page. The trigger to move (likely to Astro) is
  content the format can't do well: a blog, CMS-managed case studies, or separate
  routes with per-page SEO. Until then, no rewrite.
- **Brief pipeline upgrades** — spam protection on the form, plus a small admin
  view over stored briefs instead of email-only.
- **Durable brief storage** — the API's SQLite currently lives on ephemeral
  container storage (email is the durable record); attach a volume or move to
  Postgres if briefs need to be queryable long-term.
