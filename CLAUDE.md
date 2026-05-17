# pages

Thierry Berger's personal site — Zola static site deployed on Cloudflare Pages.

## Stack
- **Zola** (theme: `adidoks`) — `zola serve` to develop locally
  - marimo won't work locally because of COOP/COEP headers..
- **Cloudflare Pages** — build command: `bash build.sh`
- **Jujutsu** (colocated with git) — use `jj` to commit, not `git commit`

## Structure
- `content/` — Markdown pages (blog, games, research)
- `static/` — committed assets; `static/research/company/` is **build-time generated** (gitignored)
- `external/company/` — git submodule: marimo simulation project (builds to `static/research/company/`)
- `themes/adidoks/` — git submodule: Zola theme
- `tools/generate_hero_svg.py` — generates hero SVG cards for games

## Build
See `build.sh` (CI). The company submodule uses its own `python build.py`.
