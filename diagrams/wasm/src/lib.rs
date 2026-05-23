//! Browser entry point for every diagram.
//!
//! Intentionally stateless: JS owns the slider value and any drag offsets and
//! asks for an SVG string. One function per diagram, one module for the page —
//! the same code the build-time binary runs.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_lang_axes(t: f32, yaw_offset: f32, pitch_offset: f32, width: f32, height: f32) -> String {
    diagrams_core::lang_axes::svg(diagrams_core::lang_axes::Params {
        t,
        yaw_offset,
        pitch_offset,
        width,
        height,
    })
}

#[wasm_bindgen]
pub fn render_concurrency(t: f32, run: f32, width: f32, height: f32) -> String {
    diagrams_core::concurrency::svg(diagrams_core::concurrency::Params { t, run, width, height })
}

#[wasm_bindgen]
pub fn render_async(t: f32, width: f32, height: f32) -> String {
    diagrams_core::async_tasks::svg(diagrams_core::async_tasks::Params { t, width, height })
}
