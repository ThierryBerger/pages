#!/usr/bin/env python3
"""Generate hero SVG cards with embedded base64 images.

Usage:
    python tools/generate_hero_svg.py <config>

Configs are defined at the bottom of this file. Run without arguments to list them.
"""

import base64
import sys
from pathlib import Path

STATIC = Path(__file__).resolve().parent.parent / "static" / "images"


def generate(
    *,
    source_image: str,
    output_svg: str,
    title_lines: list[str],
    tagline: str,
    accent_color: str,
    overlay_color: str = "#2d2d2d",
    title_font_size: int = 52,
    title_y_start: int = 85,
    title_line_spacing: int = 45,
    tagline_y_offset: int = 35,
):
    src = STATIC / source_image
    if not src.exists():
        print(f"Error: {src} not found")
        sys.exit(1)

    suffix = src.suffix.lower()
    mime = {
        ".png": "image/png",
        ".jpg": "image/jpeg",
        ".jpeg": "image/jpeg",
    }.get(suffix)
    if not mime:
        print(f"Error: unsupported image format {suffix}")
        sys.exit(1)

    b64 = base64.b64encode(src.read_bytes()).decode("ascii")

    # Compute vertical positions
    if len(title_lines) == 1:
        title_y = title_y_start
    else:
        title_y = title_y_start - (len(title_lines) - 1) * title_line_spacing // 2

    last_title_y = title_y + (len(title_lines) - 1) * title_line_spacing
    tagline_y = last_title_y + tagline_y_offset

    # Build title text elements
    title_elements = "\n".join(
        f'  <text x="400" y="{title_y + i * title_line_spacing}" text-anchor="middle" '
        f'font-family="system-ui, \'Segoe UI\', Arial, sans-serif" font-weight="900" '
        f'font-size="{title_font_size}" fill="#ffffff" filter="url(#titleGlow)" '
        f'letter-spacing="3">{line}</text>'
        for i, line in enumerate(title_lines)
    )

    svg = f"""\
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 800 400" width="800" height="400">
  <defs>
    <linearGradient id="topFade" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color="{overlay_color}" stop-opacity="0.95"/>
      <stop offset="35%" stop-color="{overlay_color}" stop-opacity="0.7"/>
      <stop offset="65%" stop-color="{overlay_color}" stop-opacity="0.05"/>
      <stop offset="100%" stop-color="{overlay_color}" stop-opacity="0"/>
    </linearGradient>
    <linearGradient id="bottomFade" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color="{overlay_color}" stop-opacity="0"/>
      <stop offset="35%" stop-color="{overlay_color}" stop-opacity="0.05"/>
      <stop offset="65%" stop-color="{overlay_color}" stop-opacity="0.7"/>
      <stop offset="100%" stop-color="{overlay_color}" stop-opacity="0.95"/>
    </linearGradient>
    <radialGradient id="vignette" cx="50%" cy="50%" r="70%">
      <stop offset="0%" stop-color="#000" stop-opacity="0"/>
      <stop offset="100%" stop-color="#000" stop-opacity="0.45"/>
    </radialGradient>
    <filter id="titleGlow">
      <feGaussianBlur stdDeviation="3" result="blur"/>
      <feMerge>
        <feMergeNode in="blur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
    <filter id="btnGlow">
      <feGaussianBlur stdDeviation="2" result="blur"/>
      <feMerge>
        <feMergeNode in="blur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
    <filter id="textShadow">
      <feDropShadow dx="0" dy="2" stdDeviation="4" flood-color="#000" flood-opacity="0.8"/>
    </filter>
  </defs>

  <image href="data:{mime};base64,{b64}" x="0" y="0" width="800" height="400" preserveAspectRatio="xMidYMid slice"/>

  <rect width="800" height="400" fill="url(#vignette)"/>
  <rect width="800" height="200" fill="url(#topFade)"/>
  <rect y="200" width="800" height="200" fill="url(#bottomFade)"/>

{title_elements}
  <text x="400" y="{tagline_y}" text-anchor="middle" font-family="system-ui, 'Segoe UI', Arial, sans-serif" font-weight="600" font-size="15" fill="{accent_color}" letter-spacing="7" filter="url(#textShadow)">{tagline}</text>

  <rect x="335" y="342" width="130" height="40" rx="20" fill="rgba(255,255,255,0.12)" stroke="{accent_color}" stroke-width="2" filter="url(#btnGlow)"/>
  <text x="400" y="368" text-anchor="middle" font-family="system-ui, 'Segoe UI', Arial, sans-serif" font-size="15" fill="#ffffff" font-weight="700" letter-spacing="2">PLAY NOW</text>
</svg>"""

    out = STATIC / output_svg
    out.write_text(svg)
    print(f"Written {out}  ({len(b64)} base64 chars, {out.stat().st_size / 1024:.0f} KB)")


# ── Card configs ─────────────────────────────────────────────────

CONFIGS = {
    "hole3d": dict(
        source_image="hole3d-hero.png",
        output_svg="hole3d-hero.svg",
        title_lines=["HOLE3D"],
        tagline="DEVOUR THE WORLD",
        accent_color="#ffb74d",
        overlay_color="#1a1a2e",
    ),
    "raceText": dict(
        source_image="raceText.png",
        output_svg="raceText-hero.svg",
        title_lines=["RACETEXT"],
        tagline="SPELL YOUR WAY TO VICTORY",
        accent_color="#64b5f6",
        overlay_color="#1b2a1b",
    ),
    "ticTac": dict(
        source_image="ticTac.png",
        output_svg="ticTac-hero.svg",
        title_lines=["TIC TAC TOE"],
        tagline="CHALLENGE A FRIEND",
        accent_color="#5cd6a0",
        overlay_color="#2d2d2d",
    ),
    "sheriff": dict(
        source_image="sheriff-hero.jpg",
        output_svg="sheriff-hero.svg",
        title_lines=["DON'T SHOOT", "THE SHERIFF"],
        tagline="DRAW FAST, SPARE THE LAW",
        accent_color="#d4a843",
        overlay_color="#2a1f0f",
        title_font_size=42,
        title_y_start=80,
        title_line_spacing=40,
    ),
}


if __name__ == "__main__":
    if len(sys.argv) < 2 or sys.argv[1] not in CONFIGS:
        print(f"Usage: {sys.argv[0]} <config>")
        print(f"Available configs: {', '.join(CONFIGS)}")
        print(f"Use 'all' to regenerate every hero SVG.")
        sys.exit(1 if len(sys.argv) > 1 else 0)

    if sys.argv[1] == "all":
        for name, cfg in CONFIGS.items():
            print(f"Generating {name}...")
            generate(**cfg)
    else:
        generate(**CONFIGS[sys.argv[1]])
