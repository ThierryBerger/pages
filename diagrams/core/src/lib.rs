//! Diagrams for thierryberger.com.
//!
//! Every diagram is a pure function from parameters to an SVG string, with no
//! web or rendering dependencies. That is what lets the build-time binary and
//! the wasm module be literally the same code: the static fallback shipped in
//! the page cannot drift from what the reader drags around.

pub mod viz;

pub mod async_tasks;
pub mod concurrency;
pub mod lang_axes;

pub use lang_axes::{Family, Lang, Params, Scene, LANGS};

/// Render the language-axes diagram.
pub fn svg(params: Params) -> String {
    lang_axes::svg(params)
}
