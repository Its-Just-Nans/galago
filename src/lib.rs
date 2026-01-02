//! Galago

#![warn(clippy::all, rust_2018_idioms)]
#![deny(
    missing_docs,
    clippy::all,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::cargo,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::pedantic
)]
#![warn(clippy::multiple_crate_versions)]

mod app;
pub use app::GalagoApp;
mod central_panel;
pub mod path;
mod string_viewer;
mod svg_render;
mod top_panel;
mod tree_viewer;
