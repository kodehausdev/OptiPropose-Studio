# Optipropose Studio — Marketing Site

The public studio site for **Optipropose Studio** (RC 9335843, Abuja, NG). An
independent engineering studio that ships production-grade systems: logistics
pipelines, AI order structuring, and enterprise HR platforms.

The AI proposal generator product lives separately at **app.optipropose.com**
(not part of this repo). This site will eventually be hosted at
**optipropose.com** once finished.

## Files
- `Optipropose Studio.dc.html` — the live site (primary deliverable).
- `Optipropose Wireframes.dc.html` — early wireframe exploration.
- `support.js` — runtime for the Design Component format. **Do not edit.**
- `assets/`, `uploads/` — images and references.

## Architecture — Design Components (`.dc.html`)
Each page is a single self-contained `.dc.html` that opens directly in a browser.
Structure inside the file:
- `<helmet>` — fonts, `@keyframes`, and the small `<style>` block that holds the
  ONLY class-based CSS allowed (hover states, `::placeholder`, focus rings).
- `<x-dc>` … `</x-dc>` — the markup template. `{{ path }}` holes are dotted
  lookups only (no expressions). Control flow via `<sc-if>` / `<sc-for>`.
- `<script data-dc-script>` — a `class Component extends DCLogic` with a
  `renderVals()` method returning the template's values + handlers.

### Rules
- **Inline styles only** for layout/color. No stylesheets, no utility classes.
- Compute anything dynamic in `renderVals()` and expose it by name — never put
  logic in `{{ }}` holes.
- Tweakable props are declared in the `data-props` JSON on the script tag and
  read via `this.props.x ?? default`.

## Brand system
- **Accent (signature mint-green):** `#34d39a`, referenced as `var(--accent)`.
- **Background:** near-black `#0a0b0a` with a radial mint tint at the top.
- **Fonts:** `Space Grotesk` (headings), `Instrument Sans` (body),
  `JetBrains Mono` (labels, metrics, mono UI).
- **Logo:** mint rounded-square icon (upward-arrow mark) + "**Optipropose**"
  bold white + "Studio" regular zinc-400.
- Tone: precise, operator-focused, no fluff. No "All rights reserved" in footer.

## Contact form
The "Tell us what to ship" section is a vetting brief. On submit it opens the
visitor's mail client (`mailto:hello@optipropose.com`) — no backend, nothing
stored server-side.

## Conventions
- Keep copy tight; the studio positions as selective (only 2 builds at a time).
- Products referenced: **TCDGO** (logistics) and **CordHR** (full HR OS —
  payroll, leave, compliance, web dashboard; do not describe it as just a
  WhatsApp/chat assistant).
