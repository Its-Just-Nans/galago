//! String Viewer

use egui::Color32;
use egui::TextEdit;
use egui::Ui;

/// String Viewer
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct StringViewer {
    /// is windows mode
    pub is_windows: bool,
}

impl StringViewer {
    /// Create new StringViewer
    pub fn new() -> Self {
        Default::default()
    }

    /// Title of String Viewer
    pub fn title(&self) -> &'static str {
        "SVG String"
    }

    /// Show settings for String Viewer
    pub fn show_settings(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.is_windows, "String as windows");
    }

    /// Show the String Viewer
    pub fn show(&self, ui: &mut Ui, svg: &mut String, color: Color32) {
        egui::ScrollArea::vertical()
            .id_salt("string_viewer")
            .max_height(200.0)
            .show(ui, |ui| {
                let text_edit = TextEdit::multiline(svg).text_color(color);
                ui.add(text_edit);
            });
        if ui.button("Copy svg").clicked() {
            ui.ctx().copy_text(svg.clone());
        }
    }
}
