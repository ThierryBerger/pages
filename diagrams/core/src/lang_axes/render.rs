//! Scene -> SVG string.
//!
//! Output is plain SVG elements referencing CSS custom properties, with light
//! -mode hex fallbacks baked in so the standalone file opens correctly on its
//! own. Text is real `<text>`: it inherits the page font, respects dark mode,
//! is selectable, and is readable by a screen reader.

use super::data::Family;
use super::scene::{Placed, Scene};
use crate::viz::math::{smoothstep, Projected, Vec3};
use crate::viz::pen::{esc, n, Pen, FONT, VISIBLE};
use core::fmt::Write;

fn color_var(p: &Placed) -> &'static str {
    if p.lang.subject {
        "var(--la-subject, #eb6834)"
    } else if p.lang.family == Family::Manual {
        "var(--la-manual, #1baf7a)"
    } else {
        "var(--la-managed, #2a78d6)"
    }
}

pub fn render(scene: &Scene) -> String {
    let p = &scene.params;
    let mut pen = Pen::new();

    let _ = write!(
        pen.s,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" \
         width=\"100%\" preserveAspectRatio=\"xMidYMid meet\" \
         font-family=\"{}\" class=\"la-svg\" role=\"img\" \
         aria-label=\"Programming languages positioned by control, safety and cost to wield\">",
        n(p.width), n(p.height), FONT
    );

    let cam = &scene.cam;
    let (ex, ey, ez) = scene.extent();
    let pr = |x: f32, y: f32, z: f32| cam.project(Vec3::new(x, y, z));

    let grid = "var(--la-grid, #e1e0d9)";
    let axis = "var(--la-axis, #c3c2b7)";
    let muted = "var(--la-muted, #898781)";
    let ink = "var(--la-ink, #0b0b0b)";
    let ink2 = "var(--la-ink-2, #52514e)";
    let surface = "var(--la-surface, #ffffff)";

    // ---- floor plane (y = 0), spanning control x cost -------------------
    let g_floor = scene.a_cost * 0.9;
    pen.s.push_str("<g class=\"la-floor\">");
    for i in 0..=4 {
        let f = i as f32 / 4.0;
        pen.line(pr(0.0, 0.0, f * ez), pr(ex, 0.0, f * ez), grid, 1.0, g_floor, None);
        pen.line(pr(f * ex, 0.0, 0.0), pr(f * ex, 0.0, ez), grid, 1.0, g_floor, None);
    }
    pen.s.push_str("</g>");

    // ---- back wall (z = 0), the plain 2D scatter grid of stage 1 ---------
    let g_wall = scene.a_safety * 0.9;
    pen.s.push_str("<g class=\"la-wall\">");
    for i in 1..=4 {
        let f = i as f32 / 4.0;
        pen.line(pr(0.0, f * ey, 0.0), pr(ex, f * ey, 0.0), grid, 1.0, g_wall, None);
        pen.line(pr(f * ex, 0.0, 0.0), pr(f * ex, ey, 0.0), grid, 1.0, g_wall, None);
    }
    pen.s.push_str("</g>");

    // ---- the assumed trade-off: control + safety = 1 ---------------------
    // At stage 1 this is a line. At stage 2 it has to become a *plane*: the folk
    // model makes the same claim at every price. Drawn as a line it would sit on
    // the back wall while the marks moved forward, and "Rust is above the line"
    // would stop being something the reader can actually see.
    if scene.a_tradeoff > VISIBLE {
        let a = pr(0.0, scene.a_safety, 0.0);
        let b = pr(1.0, 0.0, 0.0);
        if scene.a_cost > VISIBLE {
            let a2 = pr(0.0, scene.a_safety, ez);
            let b2 = pr(1.0, 0.0, ez);
            let _ = write!(
                pen.s,
                "<polygon points=\"{},{} {},{} {},{} {},{}\" fill=\"{}\" opacity=\"{}\"/>",
                n(a.x), n(a.y), n(b.x), n(b.y), n(b2.x), n(b2.y), n(a2.x), n(a2.y),
                muted, n(scene.a_cost * scene.a_tradeoff * 0.075)
            );
            pen.line(a2, b2, muted, 1.2, scene.a_tradeoff * scene.a_cost * 0.45, Some("4 5"));
        }
        pen.line(a, b, muted, 1.6, scene.a_tradeoff * 0.85, Some("5 4"));

    }

    // ---- axes ------------------------------------------------------------
    let o = pr(0.0, 0.0, 0.0);
    pen.s.push_str("<g class=\"la-axes\">");
    pen.line(o, pr(ex, 0.0, 0.0), axis, 1.8, 1.0, None);
    pen.line(o, pr(0.0, ey, 0.0), axis, 1.8, scene.a_safety, None);
    pen.line(o, pr(0.0, 0.0, ez), axis, 1.8, scene.a_cost, None);
    pen.arrow(o, pr(ex, 0.0, 0.0), axis, 1.0, 9.0);
    pen.arrow(o, pr(0.0, ey, 0.0), axis, scene.a_safety, 9.0);
    pen.arrow(o, pr(0.0, 0.0, ez), axis, scene.a_cost, 9.0);
    pen.s.push_str("</g>");

    // ---- axis captions ---------------------------------------------------
    // The x axis is relabelled rather than replaced: stage 0's "level" and
    // stage 1's "control" are the same axis, which is the article's point.
    // The two labels share an anchor, so their fades must not overlap — one is
    // fully gone before the other starts, with a beat of nothing in between.
    let t = scene.params.t;
    let naive = 1.0 - smoothstep(0.15, 0.40, t);
    let renamed = smoothstep(0.55, 0.85, t);
    let xe = pr(ex, 0.0, 0.0);
    pen.text(o.x - 10.0, o.y + 20.0, 12.0, muted, "end", 500, naive, "high level");
    pen.text(xe.x + 14.0, xe.y + 5.0, 12.0, muted, "start", 500, naive, "low level");
    pen.text(xe.x + 14.0, xe.y + 5.0, 12.5, ink2, "start", 600, renamed, "control");

    let ye = pr(0.0, ey, 0.0);
    pen.text(ye.x, ye.y - 12.0, 12.5, ink2, "middle", 600, scene.a_safety, "safety");

    let ze = pr(0.0, 0.0, ez);
    pen.text(ze.x - 12.0, ze.y + 15.0, 12.5, ink2, "end", 600, scene.a_cost, "cost to wield");

    // ---- data marks, far to near ----------------------------------------
    pen.s.push_str("<g class=\"la-marks\">");
    for placed in &scene.placed {
        let c = color_var(placed);

        // Drop line: how much of this position is height, i.e. safety.
        pen.line(placed.mark, placed.foot, c, 1.2, scene.a_safety * 0.28, Some("3 3"));
        // Footprint on the floor: where it sits in control x cost alone.
        if scene.a_cost > VISIBLE {
            let _ = write!(
                pen.s,
                "<ellipse cx=\"{}\" cy=\"{}\" rx=\"4.5\" ry=\"2.2\" fill=\"{}\" opacity=\"{}\"/>",
                n(placed.foot.x), n(placed.foot.y), c, n(scene.a_cost * 0.35)
            );
        }

        // 2px surface ring so overlapping marks stay separable.
        let r = if placed.lang.subject { 8.5 } else { 7.0 };
        // <title> gives a native hover tooltip and doubles as the accessible name.
        let _ = write!(
            pen.s,
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\">\
             <title>{}: control {:.2}, safety {:.2}, cost {:.2}. {}</title></circle>",
            n(placed.mark.x), n(placed.mark.y), n(r), c, surface,
            esc(placed.lang.name), placed.lang.control, placed.lang.safety,
            placed.lang.cost, esc(placed.lang.note)
        );

        // Leader from mark to label when decluttering pushed the label away.
        let dy = (placed.label_y - 4.0 - placed.mark.y).abs();
        if dy > 9.0 {
            pen.line(
                placed.mark,
                Projected { x: placed.label_x - 3.0, y: placed.label_y - 4.0, depth: 0.0 },
                axis, 1.0, 0.5, None,
            );
        }

        let weight = if placed.lang.subject { 700 } else { 500 };
        pen.text(placed.label_x, placed.label_y, 13.0, ink, "start", weight, 1.0, placed.lang.name);
    }
    pen.s.push_str("</g>");

    legend(&mut pen, scene, p.width, p.height, ink2);
    captions(&mut pen, scene, ink, muted);

    pen.s.push_str("</svg>");
    pen.s
}

fn legend(pen: &mut Pen, scene: &Scene, width: f32, height: f32, ink2: &str) {
    let y = height - 18.0;
    let mut x = 48.0;
    let _ = width;
    pen.s.push_str("<g class=\"la-legend\">");

    let dots: [(&str, &str); 3] = [
        ("var(--la-manual, #1baf7a)", "manual memory"),
        ("var(--la-managed, #2a78d6)", "garbage collected"),
        ("var(--la-subject, #eb6834)", "Rust"),
    ];
    for (c, label) in dots {
        let _ = write!(
            pen.s,
            "<circle cx=\"{}\" cy=\"{}\" r=\"5.5\" fill=\"{}\"/>",
            n(x), n(y - 4.0), c
        );
        pen.text(x + 12.0, y, 12.0, ink2, "start", 500, 1.0, label);
        x += 12.0 + label.len() as f32 * 6.7 + 22.0;
    }

    // The assumed trade-off gets a swatch rather than an in-plot annotation:
    // it is a line the data sits along, so any label near it lands on a mark.
    if scene.a_tradeoff > VISIBLE {
        let op = scene.a_tradeoff;
        let a = Projected { x, y: y - 4.0, depth: 0.0 };
        let b = Projected { x: x + 18.0, y: y - 4.0, depth: 0.0 };
        pen.line(a, b, "var(--la-muted, #898781)", 1.6, op * 0.9, Some("5 4"));
        pen.text(x + 25.0, y, 12.0, ink2, "start", 500, op, "the trade-off everyone assumes");
    }

    pen.s.push_str("</g>");
}

/// The three beats, cross-faded in place as `t` sweeps.
fn captions(pen: &mut Pen, scene: &Scene, ink: &str, muted: &str) {
    let t = scene.params.t;
    // Disjoint ranges: each beat is fully faded out before the next appears.
    let beats: [(f32, &str, &str); 3] = [
        (
            1.0 - smoothstep(0.20, 0.42, t),
            "One axis",
            "the line everyone draws",
        ),
        (
            smoothstep(0.58, 0.80, t) * (1.0 - smoothstep(1.20, 1.42, t)),
            "Two axes",
            "the one you were drawing was hiding another",
        ),
        (
            smoothstep(1.58, 1.80, t),
            "Three axes",
            "and here is what leaving the line costs",
        ),
    ];
    pen.s.push_str("<g class=\"la-caption\">");
    for (op, title, sub) in beats {
        pen.text(48.0, 24.0, 16.0, ink, "start", 700, op, title);
        pen.text(48.0, 41.0, 12.5, muted, "start", 400, op, sub);
    }
    pen.s.push_str("</g>");
}
