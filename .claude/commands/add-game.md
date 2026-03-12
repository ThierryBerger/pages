# Add a new game card to the games page

You are helping add a new game entry to this Zola static site's games page.
Gather the following information from the user before proceeding (ask for
anything not already provided):

- **game_id**: short slug used in `static/games/<game_id>/` and the URL
  (e.g. `raceText`, `syllabe`)
- **title**: display name for the card (e.g. "Syllabe")
- **description**: one-sentence marketing hook (will go in the `<p>` desc)
- **features**: 2–3 bullet points (tech stack, audience, key traits)
- **hero_image**: filename of the source screenshot in `static/images/`
  (e.g. `syllabe-hero.png`)
- **accent_color**: hex color for the SVG overlay accent and CTA button
- **overlay_color**: dark hex used for the gradient overlays (default `#2d2d2d`)
- **tagline**: ALL-CAPS short tagline shown in the hero image (e.g.
  `LEARN WORDS, ONE TAP AT A TIME`)
- **title_lines**: list of strings for the hero title — split across lines if
  needed (e.g. `["DON'T SHOOT", "THE SHERIFF"]` for a long title)
- **btn_variant**: CSS class suffix — one of `default` (dark purple),
  `green`, `golden`, `purple` (bright violet), `cyan` (light blue), or
  `custom` (provide full inline style)

---

## Step 1 — Generate the hero SVG

Add the new config to `tools/generate_hero_svg.py` under `CONFIGS`:

```python
"<game_id>": dict(
    source_image="<hero_image>",
    output_svg="<game_id>-hero.svg",
    title_lines=<title_lines>,
    tagline="<tagline>",
    accent_color="<accent_color>",
    overlay_color="<overlay_color>",
    # Only add these if the title needs size/position tweaks:
    # title_font_size=42,
    # title_y_start=80,
    # title_line_spacing=40,
),
```

Then run:

```bash
python3 tools/generate_hero_svg.py <game_id>
```

Confirm the output file was written to `static/images/<game_id>-hero.svg`.

---

## Step 2 — Add the card to `content/games/_index.md`

Insert before the `<p class="games-footer">` line:

```html
<div class="card game-card">
  <a href="/games/<game_id>/index.html" class="game-card-img-link">
    <img src="/images/<game_id>-hero.svg" alt="<title> — <tagline>" class="game-card-img" />
  </a>
  <div class="card-body">
    <h2><title></h2>
    <p class="game-card-desc">
      <description>
    </p>
    <ul class="game-card-features">
      <li><feature 1></li>
      <li><feature 2></li>
      <li><feature 3></li>
    </ul>
    <a href="/games/<game_id>/index.html" class="btn btn-primary btn-game btn-game-<variant>">
      Play <title>
    </a>
  </div>
</div>
```

For `btn_variant = default`, omit the variant suffix class entirely.
For `btn_variant = custom`, use an inline `style=` on the anchor instead.

---

## Step 3 — If using a new button color variant

Add the variant to `sass/games.scss` inside the `.btn-game` rule:

```scss
&.btn-game-<variant> {
    background: linear-gradient(135deg, <dark_shade>, <accent_color>);
}
```

---

## Checklist

- [ ] Config added to `tools/generate_hero_svg.py`
- [ ] `static/images/<game_id>-hero.svg` generated successfully
- [ ] Card HTML added to `content/games/_index.md` before the footer
- [ ] Button variant exists in `sass/games.scss` (or inline style used)
- [ ] `zola serve` shows the card correctly
