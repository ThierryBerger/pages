//! Languages positioned by control, safety and cost to wield.

pub mod data;
pub mod render;
pub mod scene;

pub use data::{Family, Lang, LANGS};
pub use scene::{Params, Scene};

pub fn svg(params: Params) -> String {
    render::render(&Scene::build(params))
}
