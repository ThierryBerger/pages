//! Concurrency: one timeline becomes two, and the order between them dissolves.
//!
//! The build, on one continuous `t`:
//!
//!   t = 0  one line of time, two functions end to end.
//!   t = 1  a second line. Across the page is still time; *down* the page is
//!          now a CPU.
//!   t = 2  the second line gets work of its own. Both are busy at once.
//!   t = 3  zoom in: each block is a run of instructions.
//!   t = 4  project every instruction onto one shared ruler. They interleave,
//!          and nothing in the program decided how.

use crate::viz::math::{beat, lerp, smoothstep};
use crate::viz::pen::{n, Pen, Pt, FONT, MONO, VISIBLE};
use core::fmt::Write;

const STAGES: usize = 5;

#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Progression, 0..=4.
    pub t: f32,
    /// Which run of the same program. Fractional values interpolate, so
    /// "run it again" slides the blocks instead of cutting.
    pub run: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self { t: 4.0, run: 0.0, width: 720.0, height: 430.0 }
    }
}

/// A function call occupying a span of one lane's time.
struct Block {
    label: &'static str,
    lane: usize,
    ticks: usize,
}

const BLOCKS: &[Block] = &[
    Block { label: "fetch()",  lane: 0, ticks: 4 },
    Block { label: "parse()",  lane: 0, ticks: 3 },
    Block { label: "render()", lane: 1, ticks: 4 },
    Block { label: "write()",  lane: 1, ticks: 3 },
];

pub const RUNS: usize = 3;

/// Start and width of each block, as fractions of the track, per run.
///
/// These are not arbitrary jitter. They were searched offline under three
/// constraints: no two of the fourteen instructions may land closer than
/// ~18px (or they stop reading as two marks), the called-out pair must stay
/// *adjacent* in the merged order (or the bracket means nothing), and the two
/// lanes should alternate as much as possible. Run 1 is the one where the pair
/// comes out in the opposite order — which is the entire point.
const GEOM: [[(f32, f32); 4]; RUNS] = [
    [(0.02, 0.29), (0.39, 0.21), (0.060, 0.29), (0.43, 0.20)],
    [(0.02, 0.29), (0.41, 0.23), (0.125, 0.29), (0.45, 0.24)],
    [(0.02, 0.29), (0.45, 0.23), (0.060, 0.29), (0.49, 0.22)],
];

/// The two instructions the hazard bracket calls out: adjacent on the shared
/// ruler, from different lanes, with nothing in the program ordering them.
const HAZARD: [(usize, usize); 2] = [(0, 3), (2, 2)];

/// Geometry for block `bi` at a possibly fractional run, wrapping at the end so
/// pressing "run again" forever keeps sliding forward.
fn geom(run: f32, bi: usize) -> (f32, f32) {
    let n = RUNS as f32;
    let r = run.rem_euclid(n);
    let i0 = r.floor() as usize % RUNS;
    let i1 = (i0 + 1) % RUNS;
    let f = r - r.floor();
    let a = GEOM[i0][bi];
    let b = GEOM[i1][bi];
    (lerp(a.0, b.0, f), lerp(a.1, b.1, f))
}

fn tick_frac(run: f32, bi: usize, i: usize) -> f32 {
    let (x, w) = geom(run, bi);
    x + w * (i as f32 + 0.5) / BLOCKS[bi].ticks as f32
}

pub fn svg(p: Params) -> String {
    let t = p.t.clamp(0.0, STAGES as f32 - 1.0);
    let run = p.run;

    let a_lane1 = smoothstep(0.0, 1.0, t);
    let a_work1 = smoothstep(1.0, 2.0, t);
    let a_instr = smoothstep(2.0, 3.0, t);
    let a_ruler = smoothstep(3.0, 4.0, t);

    let x0 = 104.0;
    let x1 = p.width - 46.0;
    let track = x1 - x0;
    let lane_y = [138.0f32, 200.0];
    let ruler_y = 322.0;
    let bh = 32.0;

    let ink = "var(--dg-ink, #0b0b0b)";
    let ink2 = "var(--dg-ink-2, #52514e)";
    let muted = "var(--dg-muted, #898781)";
    let axis = "var(--dg-axis, #c3c2b7)";
    let alert = "var(--dg-alert, #d03b3b)";
    let lane_c = ["var(--dg-a, #2a78d6)", "var(--dg-b, #1baf7a)"];

    let mut pen = Pen::new();
    let _ = write!(
        pen.s,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"100%\" \
         preserveAspectRatio=\"xMidYMid meet\" font-family=\"{}\" role=\"img\" \
         aria-label=\"Two CPU timelines whose instructions interleave in an order the program does not fix\">",
        n(p.width), n(p.height), FONT
    );

    let at = |f: f32| x0 + track * f;
    let lane_alpha = |lane: usize| if lane == 0 { 1.0 } else { a_lane1 };

    // ---- the CPU axis: the thing the second line actually adds --------------
    if a_lane1 > VISIBLE {
        let top = Pt::new(40.0, lane_y[0] - 6.0);
        let bot = Pt::new(40.0, lane_y[1] + 8.0);
        pen.line(top, bot, axis, 1.6, a_lane1, None);
        pen.arrow(top, bot, axis, a_lane1, 8.0);
        let my = (lane_y[0] + lane_y[1]) * 0.5;
        pen.open("g", &format!("transform=\"rotate(-90 {} {})\"", n(24.0), n(my)));
        pen.text(24.0, my, 11.5, muted, "middle", 600, a_lane1, "CPU");
        pen.close("g");
    }

    // ---- lanes -------------------------------------------------------------
    for (i, &ly) in lane_y.iter().enumerate() {
        let op = lane_alpha(i);
        if op < VISIBLE {
            continue;
        }
        let a = Pt::new(x0, ly);
        let b = Pt::new(x1, ly);
        pen.line(a, b, axis, 1.6, op, None);
        pen.arrow(a, b, axis, op, 9.0);
        pen.open("g", &format!("font-family=\"{MONO}\""));
        pen.text(92.0, ly + 4.0, 11.5, ink2, "end", 600, op * a_lane1,
                 if i == 0 { "CPU 0" } else { "CPU 1" });
        pen.close("g");
    }
    pen.text(x1, lane_y[0] - 14.0, 11.5, muted, "end", 500, 1.0, "time");

    // ---- blocks ------------------------------------------------------------
    for (bi, b) in BLOCKS.iter().enumerate() {
        let op = if b.lane == 0 { 1.0 } else { a_work1 };
        if op < VISIBLE {
            continue;
        }
        let ly = lane_y[b.lane];
        let (gx, gw) = geom(run, bi);
        let bx = at(gx);
        let bw = track * gw;
        let c = lane_c[b.lane];

        pen.rect(bx, ly - bh * 0.5, bw, bh, 5.0, c, op * 0.14);
        pen.stroked_rect(bx, ly - bh * 0.5, bw, bh, 5.0, c, 1.5, op, None);

        // The label rises out of the block as instructions fill it, so the name
        // stays readable instead of being struck through by ticks.
        let label_y = lerp(ly + 4.0, ly - bh * 0.5 - 8.0, a_instr);
        pen.open("g", &format!("font-family=\"{MONO}\""));
        pen.text(bx + bw * 0.5, label_y, 12.0, ink, "middle", 600, op, b.label);
        pen.close("g");

        // Instructions: the block was never atomic.
        for i in 0..b.ticks {
            let tx = at(tick_frac(run, bi, i));
            pen.line(
                Pt::new(tx, ly - 9.0),
                Pt::new(tx, ly + 9.0),
                c, 2.0, op * a_instr, None,
            );
        }

        // Drop lines onto the shared ruler.
        if a_ruler > VISIBLE {
            for i in 0..b.ticks {
                let tx = at(tick_frac(run, bi, i));
                let hazard = HAZARD.contains(&(bi, i));
                pen.line(
                    Pt::new(tx, ly + 9.0),
                    Pt::new(tx, ruler_y - 9.0),
                    c,
                    if hazard { 1.4 } else { 1.0 },
                    op * a_ruler * if hazard { 0.7 } else { 0.28 },
                    Some("2 3"),
                );
            }
        }
    }

    // ---- the shared ruler: one order, chosen by nobody ----------------------
    if a_ruler > VISIBLE {
        let a = Pt::new(x0, ruler_y);
        let b = Pt::new(x1, ruler_y);
        pen.line(a, b, axis, 1.6, a_ruler, None);
        pen.arrow(a, b, axis, a_ruler, 9.0);
        pen.text(x0, ruler_y + 48.0, 11.5, muted, "start", 500, a_ruler,
                 "what actually happened, in order");

        for (bi, b) in BLOCKS.iter().enumerate() {
            for i in 0..b.ticks {
                let tx = at(tick_frac(run, bi, i));
                let hazard = HAZARD.contains(&(bi, i));
                let h = if hazard { 11.0 } else { 8.0 };
                pen.line(
                    Pt::new(tx, ruler_y - h),
                    Pt::new(tx, ruler_y + h),
                    lane_c[b.lane],
                    if hazard { 3.2 } else { 2.4 },
                    a_ruler,
                    None,
                );
            }
        }

        // Hazard bracket over the adjacent pair from different lanes.
        let hx: Vec<f32> = HAZARD
            .iter()
            .map(|&(bi, i)| at(tick_frac(run, bi, i)))
            .collect();
        let (h0, h1) = (hx[0].min(hx[1]), hx[0].max(hx[1]));
        let by = ruler_y - 20.0;
        pen.path(
            &format!("M {} {} L {} {} L {} {} L {} {}",
                     n(h0), n(by + 6.0), n(h0), n(by), n(h1), n(by), n(h1), n(by + 6.0)),
            alert, 1.6, a_ruler, None,
        );
        pen.text((h0 + h1) * 0.5, by - 7.0, 11.0, alert, "middle", 600, a_ruler, "order undefined");

        // Which of the pair actually landed first, this run.
        let first = if hx[0] <= hx[1] { HAZARD[0].0 } else { HAZARD[1].0 };
        pen.open("g", &format!("font-family=\"{MONO}\""));
        pen.text((h0 + h1) * 0.5, ruler_y + 26.0, 10.0, lane_c[BLOCKS[first].lane],
                 "middle", 600, a_ruler, &format!("this run: {} first", BLOCKS[first].label));
        pen.close("g");
    }

    // ---- captions ----------------------------------------------------------
    let beats: [(&str, &str); STAGES] = [
        ("One line", "two calls, one after the other"),
        ("A second line", "across is still time; down the page is now a CPU"),
        ("Both lines busy", "two functions running at the same instant"),
        ("Zoom in", "a block was never atomic — it is a run of instructions"),
        ("No shared order", "run the same program again and the order changes"),
    ];
    for (i, (title, sub)) in beats.iter().enumerate() {
        let op = beat(t, i, STAGES);
        pen.text(48.0, 28.0, 16.0, ink, "start", 700, op, title);
        pen.text(48.0, 46.0, 12.5, muted, "start", 400, op, sub);
    }

    pen.s.push_str("</svg>");
    pen.s
}
