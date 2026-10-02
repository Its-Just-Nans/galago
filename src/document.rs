//! Document

use bladvak::eframe::egui::{self, Color32};
use bladvak::utils::document::DocumentTrait;
use std::path::{Path, PathBuf};

use crate::svg_editor::ShapeEditor;
use crate::svg_render::SvgRender;

/// Document
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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
    /// background color
    pub(crate) background_color: Option<Color32>,
    /// rect
    pub(crate) shape_editor: ShapeEditor,
}

impl Default for Document {
    fn default() -> Self {
        // default impl for scene_rect
        Self {
            saved_svg: String::new(),
            svg: String::new(),
            scene_rect: egui::Rect::ZERO,
            svg_render: SvgRender::default(),
            should_reset_view: false,
            filename: PathBuf::new(),
            svg_is_valid: true,
            background_color: None,
            shape_editor: ShapeEditor::default(),
        }
    }
}

impl DocumentTrait for Document {
    fn path(&self) -> &Path {
        &self.filename
    }
    fn set_path(&mut self, new_path: PathBuf) {
        self.filename = new_path;
    }
    fn deep_clone(&self) -> Self {
        self.clone()
    }
}
