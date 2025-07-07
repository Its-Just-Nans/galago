#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::GalagoApp;
mod grid;
mod path;
mod settings;
mod string_viewer;
mod svg_render;
mod tree_viewer;
