//! Document

use std::path::PathBuf;

use bladvak::eframe::egui;

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
    pub(crate) filename: Option<PathBuf>,
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
            filename: None,
            svg_is_valid: true,
        }
    }
}

/// Documents
#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub(crate) struct Documents {
    /// current index
    pub(crate) current_idx: usize,
    /// documents
    inner: Vec<Document>,
}

impl Documents {
    /// get current document as mutable
    pub(crate) fn get_current_doc_mut(&mut self) -> Option<&mut Document> {
        if self.inner.is_empty() {
            return None;
        }
        let idx = self.current_idx % self.inner.len();
        Some(&mut self.inner[idx])
    }

    /// add a new document
    pub(crate) fn push(&mut self, document: Document) {
        self.inner.push(document);
        self.current_idx = self.inner.len() - 1;
    }

    /// iter on documents
    #[allow(unused)]
    pub(crate) fn iter(&mut self) -> std::slice::Iter<'_, Document> {
        self.inner.iter()
    }

    /// iter mut on documents
    pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<'_, Document> {
        self.inner.iter_mut()
    }

    /// Remove a document
    #[allow(unused)]
    pub(crate) fn remove(&mut self, index: usize) {
        self.inner.remove(index);
        self.current_idx = self.current_idx.saturating_sub(1);
    }

    /// Check if is some
    #[allow(unused)]
    pub(crate) fn is_some(&self) -> bool {
        !self.inner.is_empty()
    }

    /// Clear
    #[allow(unused)]
    pub(crate) fn clear(&mut self) {
        self.current_idx = 0;
        self.inner.clear();
    }
}
