//! Galago


#![warn(clippy::all, rust_2018_idioms)]
#![deny(
    missing_docs,
    clippy::all,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cargo
)]

mod app;
pub use app::GalagoApp;
mod grid;
mod path;
mod settings;
mod string_viewer;
mod svg_render;
mod tree_viewer;
