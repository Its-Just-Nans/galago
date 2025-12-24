//! String Viewer

use bladvak::app::BladvakPanel;
use bladvak::eframe::egui::{self, Color32, Frame};
use bladvak::{AppError, ErrorManager, egui_extras};
use resvg::usvg::WriteOptions;
use std::sync::Arc;

use crate::GalagoApp;

/// String Viewer
#[derive(serde::Deserialize, serde::Serialize)]
pub struct StringViewer {
    /// Theme
    pub theme: egui_extras::syntax_highlighting::CodeTheme,

    /// Font size for the theme
    pub theme_font_size: f32,
}

const DEFAULT_FONT_SIZE: f32 = 12.0;

impl Default for StringViewer {
    fn default() -> Self {
        Self {
            theme: egui_extras::syntax_highlighting::CodeTheme::dark(DEFAULT_FONT_SIZE),
            theme_font_size: DEFAULT_FONT_SIZE,
        }
    }
}

/// String viewer panel
#[derive(Debug)]
pub struct StringViewerPanel;

impl BladvakPanel for StringViewerPanel {
    type App = GalagoApp;

    fn name(&self) -> &str {
        "SVG String"
    }

    fn has_settings(&self) -> bool {
        true
    }

    fn ui_settings(
        &self,
        app: &mut Self::App,
        ui: &mut egui::Ui,
        _error_manager: &mut ErrorManager,
    ) {
        if ui
            .add(
                egui::Slider::new(&mut app.string_viewer.theme_font_size, 8.0..=32.0)
                    .text("Font Size"),
            )
            .on_hover_text("Font size for the code editor")
            .changed()
        {
            // TODO wait for https://github.com/emilk/egui/pull/7684
        }
        app.string_viewer.theme.ui(ui);
        app.string_viewer.theme.clone().store_in_memory(ui.ctx());
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&self, app: &mut Self::App, ui: &mut egui::Ui, error_manager: &mut ErrorManager) {
        app.show_svg_string(ui, error_manager);
    }
}

impl GalagoApp {
    /// Show the String Viewer
    pub fn show_svg_string(&mut self, ui: &mut egui::Ui, error_manager: &mut ErrorManager) {
        Frame::new().show(ui, |ui| {
            let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                let mut layout_job = egui_extras::syntax_highlighting::highlight(
                    ui.ctx(),
                    ui.style(),
                    &self.string_viewer.theme,
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
                    let multiliner = egui::TextEdit::multiline(&mut self.svg)
                        .font(egui::FontId::monospace(self.string_viewer.theme_font_size)) // for cursor height
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY);

                    if self.svg_is_valid {
                        ui.add(multiliner.layouter(&mut layouter));
                    } else {
                        ui.add(multiliner.text_color(Color32::RED)).on_hover_text(
                            "The SVG string is not correct, please check the syntax.",
                        );
                    }
                });
            ui.horizontal(|ui| {
                if ui.button("Copy svg").clicked() {
                    ui.ctx().copy_text(self.svg.clone());
                }
                if ui.button("Simplify").clicked() {
                    match resvg::usvg::Tree::from_str(&self.svg, &self.usvg_options) {
                        Ok(tree) => self.svg = tree.to_string(&WriteOptions::default()),
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
