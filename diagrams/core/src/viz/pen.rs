//! SVG drawing primitives shared by every diagram.
//!
//! Deliberately a string builder rather than a scene graph: each diagram knows
//! its own draw order, and emitting text directly keeps the wasm small and the
//! output diffable against the build-time render.

use core::fmt::Write;

pub const FONT: &str = "system-ui, -apple-system, 'Segoe UI', sans-serif";
pub const MONO: &str = "ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace";

/// Anything fainter than this is dropped rather than emitted at ~zero alpha.
pub const VISIBLE: f32 = 0.004;

/// Two decimals keeps the payload small without visible snapping.
pub fn n(v: f32) -> String {
    let r = (v * 100.0).round() / 100.0;
    if r == r.trunc() {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// A point in SVG space, plus a depth key for painter's-algorithm sorting.
/// Flat diagrams simply leave `depth` at zero.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pt {
    pub x: f32,
    pub y: f32,
    pub depth: f32,
}

impl Pt {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y, depth: 0.0 }
    }
}

#[derive(Default)]
pub struct Pen {
    pub s: String,
}

impl Pen {
    pub fn new() -> Self {
        Self { s: String::with_capacity(8192) }
    }

    pub fn raw(&mut self, s: &str) {
        self.s.push_str(s);
    }

    /// Opens an arbitrary element, e.g. `open("g", "class=\"lane\"")`.
    pub fn open(&mut self, tag: &str, attrs: &str) {
        let _ = write!(self.s, "<{tag} {attrs}>");
    }

    pub fn close(&mut self, tag: &str) {
        let _ = write!(self.s, "</{tag}>");
    }

    #[allow(clippy::too_many_arguments)]
    pub fn line(&mut self, a: Pt, b: Pt, stroke: &str, w: f32, op: f32, dash: Option<&str>) {
        if op < VISIBLE {
            return;
        }
        let d = dash.map(|d| format!(" stroke-dasharray=\"{d}\"")).unwrap_or_default();
        let _ = write!(
            self.s,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"{}/>",
            n(a.x), n(a.y), n(b.x), n(b.y), stroke, n(w), n(op), d
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn text(&mut self, x: f32, y: f32, size: f32, fill: &str, anchor: &str, weight: u32, op: f32, body: &str) {
        if op < VISIBLE {
            return;
        }
        let _ = write!(
            self.s,
            "<text x=\"{}\" y=\"{}\" font-size=\"{}\" fill=\"{}\" text-anchor=\"{}\" font-weight=\"{}\" opacity=\"{}\">{}</text>",
            n(x), n(y), n(size), fill, anchor, weight, n(op), esc(body)
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, rx: f32, fill: &str, op: f32) {
        if op < VISIBLE || w <= 0.0 {
            return;
        }
        let _ = write!(
            self.s,
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"{}\" opacity=\"{}\"/>",
            n(x), n(y), n(w), n(h), n(rx), fill, n(op)
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stroked_rect(&mut self, x: f32, y: f32, w: f32, h: f32, rx: f32, stroke: &str, sw: f32, op: f32, dash: Option<&str>) {
        if op < VISIBLE || w <= 0.0 {
            return;
        }
        let d = dash.map(|d| format!(" stroke-dasharray=\"{d}\"")).unwrap_or_default();
        let _ = write!(
            self.s,
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"{}/>",
            n(x), n(y), n(w), n(h), n(rx), stroke, n(sw), n(op), d
        );
    }

    pub fn circle(&mut self, x: f32, y: f32, r: f32, fill: &str, op: f32) {
        if op < VISIBLE {
            return;
        }
        let _ = write!(
            self.s,
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" opacity=\"{}\"/>",
            n(x), n(y), n(r), fill, n(op)
        );
    }

    pub fn path(&mut self, d: &str, stroke: &str, w: f32, op: f32, dash: Option<&str>) {
        if op < VISIBLE {
            return;
        }
        let da = dash.map(|d| format!(" stroke-dasharray=\"{d}\"")).unwrap_or_default();
        let _ = write!(
            self.s,
            "<path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{}\"{}/>",
            d, stroke, n(w), n(op), da
        );
    }

    /// Small filled triangle at `to`, oriented along `from -> to`.
    pub fn arrow(&mut self, from: Pt, to: Pt, fill: &str, op: f32, size: f32) {
        if op < VISIBLE {
            return;
        }
        let (dx, dy) = (to.x - from.x, to.y - from.y);
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-3 {
            return;
        }
        let (ux, uy) = (dx / len, dy / len);
        let (px, py) = (-uy, ux);
        let w = size * 0.38;
        let _ = write!(
            self.s,
            "<polygon points=\"{},{} {},{} {},{}\" fill=\"{}\" opacity=\"{}\"/>",
            n(to.x), n(to.y),
            n(to.x - ux * size + px * w), n(to.y - uy * size + py * w),
            n(to.x - ux * size - px * w), n(to.y - uy * size - py * w),
            fill, n(op)
        );
    }
}
