//! Shared drawing toolkit: easing, projection and SVG primitives that every
//! diagram in this crate builds on.

pub mod math;
pub mod pen;

pub use math::{clamp01, lerp, smoothstep};
pub use pen::{esc, n, Pen, Pt, FONT, MONO, VISIBLE};
