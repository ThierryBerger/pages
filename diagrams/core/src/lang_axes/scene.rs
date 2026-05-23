//! Turns a progression value `t` into a laid-out, depth-sorted scene.
//!
//! The whole narrative is three beats on one continuous parameter:
//!
//!   t = 0   one axis.    The line everybody draws: low level <-> high level.
//!   t = 1   two axes.    That axis was only ever `control`; `safety` was the
//!                        thing it hid. The assumed trade-off appears as a line.
//!   t = 2   three axes.  `cost` lifts out of the page and explains what the
//!                        languages sitting off that line had to pay.
//!
//! Crucially the x coordinate never moves. Stage 0 is not a different chart —
//! it is the same chart with two coordinates still collapsed to zero, which is
//! exactly the claim the article is making about the folk model.

use super::data::{Lang, LANGS};
use crate::viz::math::{lerp, smoothstep, Camera, Projected, Vec3};

/// How far past 1.0 the axis lines and grid extend.
const AXIS_OVERSHOOT: f32 = 1.08;
/// Camera pose once the third axis is fully revealed.
const YAW_FINAL: f32 = -0.35; // rad, ~ -20 deg
const PITCH_FINAL: f32 = 0.26; // rad, ~  15 deg

#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// Progression, 0..=2.
    pub t: f32,
    /// Extra rotation from the reader dragging, radians.
    pub yaw_offset: f32,
    pub pitch_offset: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            t: 2.0,
            yaw_offset: 0.0,
            pitch_offset: 0.0,
            width: 720.0,
            height: 580.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Placed {
    pub lang: &'static Lang,
    pub world: Vec3,
    /// The mark itself.
    pub mark: Projected,
    /// Directly below the mark on the floor plane: shows how much of the
    /// position is height (safety) versus footprint.
    pub foot: Projected,
    pub label_x: f32,
    pub label_y: f32,
}

pub struct Scene {
    pub params: Params,
    pub cam: Camera,
    /// 0 -> the safety axis is collapsed; 1 -> fully revealed.
    pub a_safety: f32,
    /// 0 -> the cost axis is collapsed; 1 -> fully revealed.
    pub a_cost: f32,
    /// Fades the assumed trade-off line in slightly before stage 1 lands.
    pub a_tradeoff: f32,
    /// Depth-sorted far to near, ready for painter's algorithm.
    pub placed: Vec<Placed>,
    pub plot: Rect,
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl Scene {
    pub fn build(params: Params) -> Scene {
        let t = params.t.clamp(0.0, 2.0);
        let a_safety = smoothstep(0.0, 1.0, t);
        let a_cost = smoothstep(1.0, 2.0, t);
        let a_tradeoff = smoothstep(0.62, 1.0, t);

        // Room for labels on the right, the legend below, captions above.
        let plot = Rect {
            x0: 48.0,
            y0: 76.0,
            x1: params.width - 84.0,
            y1: params.height - 58.0,
        };

        let yaw = lerp(0.0, YAW_FINAL, a_cost) + params.yaw_offset;
        let pitch = lerp(0.0, PITCH_FINAL, a_cost) + params.pitch_offset;

        // Content grows as axes reveal, so keep the framing centred on it.
        let center = Vec3::new(0.5, 0.5 * a_safety, 0.5 * a_cost);

        let mut cam = Camera {
            yaw,
            pitch,
            scale: 1.0,
            center,
            origin: (0.0, 0.0),
        };
        cam.scale = fit_scale(&cam, a_safety, a_cost, &plot);
        cam.origin = fit_origin(&cam, a_safety, a_cost, &plot);

        let mut placed: Vec<Placed> = LANGS
            .iter()
            .map(|lang| {
                let world = Vec3::new(
                    lang.control,
                    lang.safety * a_safety,
                    lang.cost * a_cost,
                );
                let mark = cam.project(world);
                let foot = cam.project(Vec3::new(world.x, 0.0, world.z));
                Placed {
                    lang,
                    world,
                    mark,
                    foot,
                    label_x: mark.x,
                    label_y: mark.y,
                }
            })
            .collect();

        // Painter's algorithm: smaller depth is farther from the camera.
        placed.sort_by(|a, b| a.mark.depth.total_cmp(&b.mark.depth));

        place_labels(&mut placed, a_safety, &plot);

        Scene {
            params,
            cam,
            a_safety,
            a_cost,
            a_tradeoff,
            placed,
            plot,
        }
    }

    /// World-space corners of the content box at the current reveal.
    pub fn extent(&self) -> (f32, f32, f32) {
        content_extent(self.a_safety, self.a_cost)
    }
}

fn content_extent(a_safety: f32, a_cost: f32) -> (f32, f32, f32) {
    (
        AXIS_OVERSHOOT,
        AXIS_OVERSHOOT * a_safety,
        AXIS_OVERSHOOT * a_cost,
    )
}

/// Project the eight corners of the content box and pick the scale that makes
/// the result fill the plot rect. Recomputed per frame, so dragging can never
/// push the diagram outside its box.
fn projected_bounds(cam: &Camera, a_safety: f32, a_cost: f32) -> (f32, f32, f32, f32) {
    let (ex, ey, ez) = content_extent(a_safety, a_cost);
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for &x in &[0.0, ex] {
        for &y in &[0.0, ey] {
            for &z in &[0.0, ez] {
                let p = cam.project(Vec3::new(x, y, z));
                min_x = min_x.min(p.x);
                max_x = max_x.max(p.x);
                min_y = min_y.min(p.y);
                max_y = max_y.max(p.y);
            }
        }
    }
    (min_x, min_y, max_x, max_y)
}

fn fit_scale(cam: &Camera, a_safety: f32, a_cost: f32, plot: &Rect) -> f32 {
    let unit = Camera { scale: 1.0, origin: (0.0, 0.0), ..*cam };
    let (min_x, min_y, max_x, max_y) = projected_bounds(&unit, a_safety, a_cost);
    let w = (max_x - min_x).max(1e-4);
    let h = (max_y - min_y).max(1e-4);
    let sx = (plot.x1 - plot.x0) / w;
    let sy = (plot.y1 - plot.y0) / h;
    // A lone horizontal line (stage 0) would otherwise be scaled up until it
    // overflowed vertically-unbounded; cap so the reveal grows rather than shrinks.
    sx.min(sy).min(420.0)
}

fn fit_origin(cam: &Camera, a_safety: f32, a_cost: f32, plot: &Rect) -> (f32, f32) {
    let (min_x, min_y, max_x, max_y) = projected_bounds(cam, a_safety, a_cost);
    let cx = (plot.x0 + plot.x1) * 0.5 - (min_x + max_x) * 0.5;
    let cy = (plot.y0 + plot.y1) * 0.5 - (min_y + max_y) * 0.5;
    (cx, cy)
}

const LABEL_H: f32 = 15.0;
const CHAR_W: f32 = 7.6;

/// Labels sit to the right of their mark. At stage 0 every point is on one
/// horizontal line, so the offset staggers up/down and relaxes back to a plain
/// right-hand label as the safety axis separates the points on its own.
fn place_labels(placed: &mut [Placed], a_safety: f32, plot: &Rect) {
    let stagger = 1.0 - a_safety;
    for (i, p) in placed.iter_mut().enumerate() {
        let dir = if i % 2 == 0 { -1.0 } else { 1.0 };
        p.label_x = p.mark.x + 12.0;
        p.label_y = p.mark.y + 4.0 + stagger * dir * 21.0;
    }

    // Relax overlaps vertically. Eight labels converge in a handful of passes.
    for _ in 0..80 {
        let mut moved = false;
        for i in 0..placed.len() {
            for j in (i + 1)..placed.len() {
                let wi = placed[i].lang.name.len() as f32 * CHAR_W;
                let wj = placed[j].lang.name.len() as f32 * CHAR_W;
                let dx_gap = (placed[i].label_x - placed[j].label_x).abs();
                if dx_gap > (wi.max(wj) + 6.0) {
                    continue;
                }
                let dy = placed[j].label_y - placed[i].label_y;
                let need = LABEL_H;
                if dy.abs() < need {
                    let push = (need - dy.abs()) * 0.5 + 0.25;
                    let sign = if dy >= 0.0 { 1.0 } else { -1.0 };
                    placed[i].label_y -= push * sign;
                    placed[j].label_y += push * sign;
                    moved = true;
                }
            }
        }
        if !moved {
            break;
        }
    }

    for p in placed.iter_mut() {
        p.label_y = p.label_y.clamp(plot.y0 - 34.0, plot.y1 + 40.0);
    }
}
