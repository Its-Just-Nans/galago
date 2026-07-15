//! Document

use bladvak::eframe::egui;
use bladvak::utils::document::DocumentTrait;
use std::path::{Path, PathBuf};

use crate::svg_render::SvgRender;

/// Document
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(default)]
pub(crate) struct Document {
    /// Save of the SVG
    pub(crate) saved_svg: String,
    /// SVG Screen
    pub(crate) svg: String,
    /// Current scene zoom
    #[serde(skip)]
    pub(crate) scene_rect: egui::Rect,
    /// `SvgRender`
    pub(crate) svg_render: SvgRender,
    /// should reset the view
    pub(crate) should_reset_view: bool,
    /// Path to save the svg
    pub(crate) filename: PathBuf,
    /// Svg is valid
    pub(crate) svg_is_valid: bool,
}

impl Default for Document {
    fn default() -> Self {
        // default impl for scene_rect
        Self {
            saved_svg: String::new(),
            svg: String::new(),
            scene_rect: egui::Rect::NAN,
            svg_render: SvgRender::default(),
            should_reset_view: false,
            filename: PathBuf::new(),
            svg_is_valid: true,
        }
    }
}

impl DocumentTrait for Document {
    fn path(&self) -> &Path {
        &self.filename
    }
}
