//! Minimal 3D math. Orthographic on purpose: a diagram is read by comparing
//! positions, and perspective foreshortening makes equal distances look unequal.

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn clamp01(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

/// Smooth 0->1 ramp between two edges. Used for every stage transition so the
/// morph eases instead of sliding linearly.
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    if (edge1 - edge0).abs() < f32::EPSILON {
        return if x < edge0 { 0.0 } else { 1.0 };
    }
    let t = clamp01((x - edge0) / (edge1 - edge0));
    t * t * (3.0 - 2.0 * t)
}

/// Projection output is just a `Pt`: screen coordinates plus a depth key for
/// painter's-algorithm sorting.
pub use crate::viz::pen::Pt as Projected;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// Rotation about the world Y (up) axis, radians.
    pub yaw: f32,
    /// Rotation about the camera's X axis, radians. Positive looks down.
    pub pitch: f32,
    /// World units -> SVG units.
    pub scale: f32,
    /// World point that lands on `origin`.
    pub center: Vec3,
    /// Where `center` sits in SVG coordinates.
    pub origin: (f32, f32),
}

impl Camera {
    pub fn project(&self, p: Vec3) -> Projected {
        let dx = p.x - self.center.x;
        let dy = p.y - self.center.y;
        let dz = p.z - self.center.z;

        // Yaw about Y.
        let (sy, cy) = self.yaw.sin_cos();
        let x1 = dx * cy + dz * sy;
        let z1 = -dx * sy + dz * cy;

        // Pitch about the (already yawed) X.
        let (sp, cp) = self.pitch.sin_cos();
        let y2 = dy * cp - z1 * sp;
        let z2 = dy * sp + z1 * cp;

        Projected {
            x: self.origin.0 + x1 * self.scale,
            // SVG y grows downward; world y grows up.
            y: self.origin.1 - y2 * self.scale,
            depth: z2,
        }
    }
}

/// Opacity for a caption belonging to integer stage `i` of `count`.
///
/// Ranges are disjoint on purpose: a caption is fully gone before the next
/// appears. Cross-dissolving two texts at one position just means the reader
/// sees both at once, which is illegible — shapes may overlap, text may not.
pub fn beat(t: f32, i: usize, count: usize) -> f32 {
    let f = i as f32;
    let fade_in = if i == 0 { 1.0 } else { smoothstep(f - 0.42, f - 0.20, t) };
    let fade_out = if i + 1 == count { 1.0 } else { 1.0 - smoothstep(f + 0.20, f + 0.42, t) };
    fade_in * fade_out
}
