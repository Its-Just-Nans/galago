use egui::Color32;
use egui::TextEdit;
use egui::Ui;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct StringViewer {}

impl StringViewer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn title(&self) -> &'static str {
        "SVG String"
    }

    pub fn show(&self, ui: &mut Ui, svg: &mut String, color: Color32) {
        egui::ScrollArea::vertical()
            .id_salt("string_viewer")
            .show(ui, |ui| {
                let text_edit = TextEdit::multiline(svg).text_color(color);
                ui.add(text_edit);
            });
        if ui.button("Copy svg").clicked() {
            ui.ctx().copy_text(svg.clone());
        }
    }
}
