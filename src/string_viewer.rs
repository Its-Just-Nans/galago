//! String Viewer

use std::sync::Arc;

use bladvak::eframe::egui::{self, Color32, Frame};
use bladvak::{AppError, ErrorManager, egui_extras};
use resvg::usvg::WriteOptions;

/// String Viewer
#[derive(serde::Deserialize, serde::Serialize)]
pub struct StringViewer {
    /// is windows mode
    pub is_windows: bool,

    /// Theme
    pub theme: egui_extras::syntax_highlighting::CodeTheme,

    /// Font size for the theme
    pub theme_font_size: f32,
}

const DEFAULT_FONT_SIZE: f32 = 12.0;

impl Default for StringViewer {
    fn default() -> Self {
        Self {
            is_windows: false,
            theme: egui_extras::syntax_highlighting::CodeTheme::dark(DEFAULT_FONT_SIZE),
            theme_font_size: DEFAULT_FONT_SIZE,
        }
    }
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
    pub fn show_settings(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.is_windows, "String as windows");
        if ui
            .add(egui::Slider::new(&mut self.theme_font_size, 8.0..=32.0).text("Font Size"))
            .on_hover_text("Font size for the code editor")
            .changed()
        {
            // TODO wait for https://github.com/emilk/egui/pull/7684
        }
        self.theme.ui(ui);
        self.theme.clone().store_in_memory(ui.ctx());
    }

    /// Show the String Viewer
    pub fn show(
        &self,
        ui: &mut egui::Ui,
        svg: &mut String,
        is_correct: bool,
        error_manager: &mut ErrorManager,
    ) {
        Frame::new().show(ui, |ui| {
            let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    ui.style(),
                    &self.theme,
                    buf.as_str(),
                    "svg",
                );
                layout_job.wrap.max_width = wrap_width;
                ui.fonts_mut(|f| f.layout_job(layout_job))
            };
            let height = ui.ctx().viewport_rect().height();
            egui::ScrollArea::vertical()
                .max_height(height / 2.0 - 40.0)
                .show(ui, |ui| {
                    let multiliner = egui::TextEdit::multiline(svg)
                        .font(egui::FontId::monospace(self.theme_font_size)) // for cursor height
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY);

                    if is_correct {
                        ui.add(multiliner.layouter(&mut layouter));
                    } else {
                        ui.add(multiliner.text_color(Color32::RED)).on_hover_text(
                            "The SVG string is not correct, please check the syntax.",
                        );
                    }
                });
            ui.horizontal(|ui| {
                if ui.button("Copy svg").clicked() {
                    ui.ctx().copy_text(svg.clone());
                }
                if ui.button("Simplify").clicked() {
                    match resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()) {
                        Ok(tree) => *svg = tree.to_string(&WriteOptions::default()),
                        Err(e) => {
                            error_manager.add_error(AppError::new_with_source(
                                "Cannot simplify the svg",
                                Arc::new(e),
                            ));
                        }
                    }
                }
            });
        });
    }
}
