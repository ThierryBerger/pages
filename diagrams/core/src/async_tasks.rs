//! Async: where the waiting went.
//!
//! The build, on one continuous `t`:
//!
//!   t = 0  one line, two calls. The second starts when the first finishes.
//!   t = 1  each call is really units of work with waiting between them.
//!          Those units are the awaits.
//!   t = 2  lift the functions off the timeline. An async fn does not run —
//!          it describes work.
//!   t = 3  a runtime appears, owning the only timeline there is.
//!   t = 4  the units land on that timeline, interleaved, with the waiting
//!          overlapped away.
//!   t = 5  each unit hands back a Future.
//!   t = 6  the runtime loop polls it, and both enums open up: Poll<T>, and
//!          the state machine the async fn actually compiled to.

use crate::viz::math::{beat, lerp, smoothstep};
use crate::viz::pen::{n, Pen, Pt, FONT, MONO, VISIBLE};
use core::fmt::Write;

const STAGES: usize = 7;

#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Progression, 0..=6.
    pub t: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self { t: 6.0, width: 720.0, height: 600.0 }
    }
}

struct Func {
    label: &'static str,
    /// Position on the stage-0 shared timeline, as a fraction of the track.
    tl_x: f32,
    tl_w: f32,
    /// Position once lifted off the timeline into its own box.
    box_y: f32,
}

const FUNCS: &[Func] = &[
    Func { label: "async fn load_user()",  tl_x: 0.02, tl_w: 0.46, box_y: 88.0 },
    Func { label: "async fn load_posts()", tl_x: 0.50, tl_w: 0.46, box_y: 156.0 },
];

struct Unit {
    label: &'static str,
    func: usize,
    /// Position within its function's block on the timeline, 0..1.
    rel_x: f32,
    rel_w: f32,
    /// Order the runtime happens to run it in — interleaved between functions.
    slot: usize,
}

const UNITS: &[Unit] = &[
    Unit { label: "GET /user",  func: 0, rel_x: 0.06, rel_w: 0.17, slot: 0 },
    Unit { label: "parse json", func: 0, rel_x: 0.62, rel_w: 0.17, slot: 2 },
    Unit { label: "GET /posts", func: 1, rel_x: 0.06, rel_w: 0.17, slot: 1 },
    Unit { label: "parse json", func: 1, rel_x: 0.62, rel_w: 0.17, slot: 3 },
];

const BOX_X: f32 = 44.0;
const BOX_W: f32 = 304.0;
const BOX_H: f32 = 54.0;
const TL_Y: f32 = 150.0;
const TL_H: f32 = 38.0;
const RT_TOP: f32 = 246.0;
const RT_H: f32 = 96.0;
const RT_LINE: f32 = 312.0;

pub fn svg(p: Params) -> String {
    let t = p.t.clamp(0.0, STAGES as f32 - 1.0);

    let a_units = smoothstep(0.0, 1.0, t);
    let a_isolate = smoothstep(1.0, 2.0, t);
    let a_runtime = smoothstep(2.0, 3.0, t);
    let a_sched = smoothstep(3.0, 4.0, t);
    let a_future = smoothstep(4.0, 5.0, t);
    let a_poll = smoothstep(5.0, 6.0, t);

    let x0 = 60.0;
    let x1 = p.width - 40.0;
    let track = x1 - x0;

    let ink = "var(--dg-ink, #0b0b0b)";
    let ink2 = "var(--dg-ink-2, #52514e)";
    let muted = "var(--dg-muted, #898781)";
    let axis = "var(--dg-axis, #c3c2b7)";
    let accent = "var(--dg-accent, #eb6834)";
    let fc = ["var(--dg-a, #2a78d6)", "var(--dg-b, #1baf7a)"];

    let mut pen = Pen::new();
    let _ = write!(
        pen.s,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"100%\" \
         preserveAspectRatio=\"xMidYMid meet\" font-family=\"{}\" role=\"img\" \
         aria-label=\"Two async functions become units of work interleaved on a single runtime timeline, driven by polling futures\">",
        n(p.width), n(p.height), FONT
    );

    // Geometry of a function, morphing from timeline block to isolated box.
    let fbox = |i: usize| -> (f32, f32, f32, f32) {
        let f = &FUNCS[i];
        let tx = x0 + track * f.tl_x;
        let tw = track * f.tl_w;
        (
            lerp(tx, BOX_X, a_isolate),
            lerp(tw, BOX_W, a_isolate),
            lerp(TL_Y - TL_H * 0.5, f.box_y, a_isolate),
            lerp(TL_H, BOX_H, a_isolate),
        )
    };

    // ---- the original single timeline ---------------------------------------
    let tl_op = 1.0 - a_isolate;
    if tl_op > VISIBLE {
        let a = Pt::new(x0, TL_Y + 34.0);
        let b = Pt::new(x1, TL_Y + 34.0);
        pen.line(a, b, axis, 1.6, tl_op, None);
        pen.arrow(a, b, axis, tl_op, 9.0);
        pen.text(x1, TL_Y + 52.0, 11.5, muted, "end", 500, tl_op, "time");
    }

    // ---- functions -----------------------------------------------------------
    for (i, f) in FUNCS.iter().enumerate() {
        let (bx, bw, by, bh) = fbox(i);
        let c = fc[i];
        pen.rect(bx, by, bw, bh, 6.0, c, 0.10);
        pen.stroked_rect(bx, by, bw, bh, 6.0, c, 1.5, 1.0, None);
        // Centred while the block is solid, lifted clear once units fill it,
        // then settled as the box header once the function is isolated.
        let ly = lerp(lerp(by + bh * 0.5 + 4.0, by - 8.0, a_units), by + 17.0, a_isolate);
        pen.open("g", &format!("font-family=\"{MONO}\""));
        pen.text(bx + 12.0, ly, 11.5, ink, "start", 600, 1.0, f.label);
        pen.close("g");
    }

    // ---- waiting: the reason any of this exists ------------------------------
    let wait_op = a_units * (1.0 - a_isolate);
    if wait_op > VISIBLE {
        for (i, _) in FUNCS.iter().enumerate() {
            let (bx, bw, by, bh) = fbox(i);
            let us: Vec<&Unit> = UNITS.iter().filter(|u| u.func == i).collect();
            // The span between the two awaits is dead time on this thread.
            let g0 = bx + bw * (us[0].rel_x + us[0].rel_w);
            let g1 = bx + bw * us[1].rel_x;
            pen.line(
                Pt::new(g0, by + bh * 0.5),
                Pt::new(g1, by + bh * 0.5),
                muted, 1.2, wait_op * 0.8, Some("3 3"),
            );
            if i == 0 {
                pen.text((g0 + g1) * 0.5, by + bh * 0.5 - 7.0, 10.5, muted, "middle", 500, wait_op, "waiting");
            }
        }
    }

    // ---- units, inside their function ---------------------------------------
    for u in UNITS {
        let (bx, bw, by, bh) = fbox(u.func);
        let c = fc[u.func];
        // On the timeline the unit sits where it happens; in the box it becomes
        // one of a list of things the function will ask for.
        let tl_ux = bx + bw * u.rel_x;
        let tl_uw = bw * u.rel_w;
        let idx = if u.rel_x < 0.3 { 0.0 } else { 1.0 };
        let chip_x = BOX_X + 14.0 + idx * 146.0;
        let chip_w = 132.0;
        let ux = lerp(tl_ux, chip_x, a_isolate);
        let uw = lerp(tl_uw, chip_w, a_isolate);
        let uy = lerp(by + bh * 0.5 - 9.0, by + 26.0, a_isolate);

        pen.rect(ux, uy, uw, 20.0, 4.0, c, a_units * 0.20);
        pen.stroked_rect(ux, uy, uw, 20.0, 4.0, c, 1.4, a_units, None);
        pen.open("g", &format!("font-family=\"{MONO}\""));
        pen.text(ux + uw * 0.5, uy + 14.0, 10.0, ink,
                 "middle", 600, a_units * a_isolate, u.label);
        pen.close("g");
        // `.await` marker, only while the units still live on the timeline.
        pen.open("g", &format!("font-family=\"{MONO}\""));
        pen.text(ux + uw * 0.5, uy + 36.0, 9.5, c, "middle", 600,
                 a_units * (1.0 - a_isolate), ".await");
        pen.close("g");
    }

    // ---- the runtime ---------------------------------------------------------
    if a_runtime > VISIBLE {
        pen.stroked_rect(36.0, RT_TOP, p.width - 72.0, RT_H, 8.0, axis, 1.4, a_runtime, Some("5 4"));
        pen.open("g", &format!("font-family=\"{MONO}\""));
        pen.text(48.0, RT_TOP + 20.0, 11.5, accent, "start", 700, a_runtime, "runtime");
        pen.close("g");
        let a = Pt::new(126.0, RT_LINE);
        let b = Pt::new(p.width - 52.0, RT_LINE);
        pen.line(a, b, axis, 1.6, a_runtime, None);
        pen.arrow(a, b, axis, a_runtime, 9.0);
        pen.text(p.width - 52.0, RT_LINE + 22.0, 11.0, muted, "end", 500, a_runtime, "time");
    }

    // ---- units scheduled onto the runtime timeline --------------------------
    let slot_x = |s: usize| 136.0 + s as f32 * 112.0;
    let slot_w = 100.0;
    if a_sched > VISIBLE {
        for u in UNITS {
            let c = fc[u.func];
            let sx = slot_x(u.slot);
            let sy = RT_LINE - 11.0;
            // Connector back to the function that described this unit.
            let (bx, _, by, bh) = fbox(u.func);
            pen.path(
                &format!("M {} {} C {} {}, {} {}, {} {}",
                         n(bx + 60.0), n(by + bh), n(bx + 60.0), n(by + bh + 40.0),
                         n(sx + slot_w * 0.5), n(sy - 46.0), n(sx + slot_w * 0.5), n(sy - 2.0)),
                c, 1.0, a_sched * 0.32, Some("3 3"),
            );
            pen.rect(sx, sy, slot_w, 22.0, 4.0, c, a_sched * 0.20);
            pen.stroked_rect(sx, sy, slot_w, 22.0, 4.0, c, 1.4, a_sched, None);
            pen.open("g", &format!("font-family=\"{MONO}\""));
            pen.text(sx + slot_w * 0.5, sy + 15.0, 10.0, ink,
                     "middle", 600, a_sched, u.label);
            pen.close("g");
        }
    }

    // ---- each unit hands back a Future ---------------------------------------
    if a_future > VISIBLE {
        for u in UNITS {
            let sx = slot_x(u.slot);
            let px = sx + slot_w * 0.5 - 28.0;
            let py = RT_LINE - 48.0;
            pen.stroked_rect(px, py, 56.0, 18.0, 9.0, accent, 1.3, a_future, None);
            pen.open("g", &format!("font-family=\"{MONO}\""));
            pen.text(px + 28.0, py + 13.0, 9.5, accent, "middle", 600, a_future, "Future");
            pen.close("g");
            pen.line(
                Pt::new(px + 28.0, py + 18.0),
                Pt::new(px + 28.0, RT_LINE - 13.0),
                accent, 1.0, a_future * 0.5, Some("2 2"),
            );
        }
    }

    // ---- the poll loop, and what is actually inside ---------------------------
    if a_poll > VISIBLE {
        // A loop glyph on the runtime: poll, and poll again.
        let (cx, cy, r) = (82.0, RT_LINE - 6.0, 15.0);
        pen.path(
            &format!("M {} {} A {} {} 0 1 1 {} {}", n(cx - r), n(cy), n(r), n(r), n(cx), n(cy - r)),
            accent, 1.8, a_poll, None,
        );
        pen.arrow(Pt::new(cx - 4.0, cy - r), Pt::new(cx + 3.0, cy - r + 1.0), accent, a_poll, 7.0);
        pen.open("g", &format!("font-family=\"{MONO}\""));
        pen.text(cx, cy + r + 15.0, 10.0, accent, "middle", 600, a_poll, "poll");
        pen.close("g");

        let py = 372.0;
        panel(&mut pen, 36.0, py, 320.0, 174.0, a_poll, ink2,
              "the async fn compiled to", &[
            ("enum LoadUser {", ink),
            ("    Start,", ink2),
            ("    AwaitingGet(GetFut),", ink2),
            ("    AwaitingJson(JsonFut),", ink2),
            ("    Done,", ink2),
            ("}", ink),
            ("", ink),
            ("one variant per .await", muted),
        ]);

        panel(&mut pen, 372.0, py, 312.0, 174.0, a_poll, ink2,
              "what the runtime asks it", &[
            ("enum Poll<T> {", ink),
            ("    Pending,", ink2),
            ("    Ready(T),", ink2),
            ("}", ink),
            ("", ink),
            ("Pending -> back in the queue", muted),
            ("Ready   -> advance the state", muted),
        ]);
    }

    // ---- captions ------------------------------------------------------------
    let beats: [(&str, &str); STAGES] = [
        ("Two calls, one line", "the second starts when the first finishes"),
        ("Units of work", "each .await is a unit; between them the thread just waits"),
        ("A description, not a schedule", "an async fn doesn't run — it says what to run"),
        ("A runtime", "it owns the only timeline there is"),
        ("Interleaved", "one thread, and the waiting overlaps"),
        ("Futures", "each unit hands back something you can poll"),
        ("The loop, and the enums", "Pending goes back in the queue; the fn was a state machine"),
    ];
    for (i, (title, sub)) in beats.iter().enumerate() {
        let op = beat(t, i, STAGES);
        pen.text(44.0, 28.0, 16.0, ink, "start", 700, op, title);
        pen.text(44.0, 46.0, 12.5, muted, "start", 400, op, sub);
    }

    pen.s.push_str("</svg>");
    pen.s
}

#[allow(clippy::too_many_arguments)]
fn panel(pen: &mut Pen, x: f32, y: f32, w: f32, h: f32, op: f32, title_c: &str,
         title: &str, lines: &[(&str, &str)]) {
    pen.stroked_rect(x, y, w, h, 6.0, "var(--dg-axis, #c3c2b7)", 1.2, op * 0.8, None);
    pen.text(x + 14.0, y + 20.0, 11.0, title_c, "start", 600, op, title);
    pen.open("g", &format!("font-family=\"{MONO}\""));
    for (i, (line, c)) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        pen.text(x + 14.0, y + 42.0 + i as f32 * 15.5, 10.5, c, "start", 500, op, line);
    }
    pen.close("g");
}
