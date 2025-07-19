//! String Viewer

use egui::Color32;
use egui::Ui;

/// String Viewer
#[derive(serde::Deserialize, serde::Serialize)]
pub struct StringViewer {
    /// is windows mode
    pub is_windows: bool,

    /// Theme
    pub theme: egui_extras::syntax_highlighting::CodeTheme,

    /// is theme dark
    pub is_theme_dark: bool,

    /// Font size for the theme
    pub theme_font_size: f32,
}

const DEFAULT_FONT_SIZE: f32 = 12.0;

impl Default for StringViewer {
    fn default() -> Self {
        Self {
            is_windows: false,
            theme: egui_extras::syntax_highlighting::CodeTheme::dark(DEFAULT_FONT_SIZE),
            is_theme_dark: true,
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
    pub fn show_settings(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.is_windows, "String as windows");
        if ui
            .add(egui::Slider::new(&mut self.theme_font_size, 8.0..=32.0).text("Font Size"))
            .on_hover_text("Font size for the code editor")
            .changed()
        {
            if self.is_theme_dark {
                self.theme =
                    egui_extras::syntax_highlighting::CodeTheme::dark(self.theme_font_size);
                self.is_theme_dark = true;
            } else {
                self.theme =
                    egui_extras::syntax_highlighting::CodeTheme::light(self.theme_font_size);
                self.is_theme_dark = false;
            }
        }
        ui.horizontal(|ui| {
            let mut current_value_is_dark = self.is_theme_dark;
            ui.radio_value(&mut current_value_is_dark, true, "Dark Theme")
                .on_hover_text("Use dark theme for the code editor");
            ui.radio_value(&mut current_value_is_dark, false, "Light Theme");
            if current_value_is_dark != self.is_theme_dark {
                if current_value_is_dark {
                    self.theme =
                        egui_extras::syntax_highlighting::CodeTheme::dark(self.theme_font_size);
                    self.is_theme_dark = true;
                } else {
                    self.theme =
                        egui_extras::syntax_highlighting::CodeTheme::light(self.theme_font_size);
                    self.is_theme_dark = false;
                }
            }
        });
        self.theme.ui(ui);
        self.theme.clone().store_in_memory(ui.ctx());
    }

    /// Show the String Viewer
    pub fn show(&self, ui: &mut Ui, svg: &mut String, is_correct: bool) {
        let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
            let mut layout_job = egui_extras::syntax_highlighting::highlight(
                ui.ctx(),
                ui.style(),
                &self.theme,
                buf.as_str(),
                "rs",
            );
            layout_job.wrap.max_width = wrap_width;
            ui.fonts(|f| f.layout_job(layout_job))
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            let multiliner = egui::TextEdit::multiline(svg)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(10)
                .lock_focus(true)
                .desired_width(f32::INFINITY);

            if is_correct {
                ui.add(multiliner.layouter(&mut layouter));
            } else {
                ui.add(multiliner.text_color(Color32::RED))
                    .on_hover_text("The SVG string is not correct, please check the syntax.");
            }
        });
        // egui::ScrollArea::vertical()
        //     .id_salt("string_viewer")
        //     .max_height(200.0)
        //     .show(ui, |ui| {
        //         let text_edit = TextEdit::multiline(svg).text_color(color);
        //         ui.add(text_edit);
        //     });
        if ui.button("Copy svg").clicked() {
            ui.ctx().copy_text(svg.clone());
        }
    }
}
