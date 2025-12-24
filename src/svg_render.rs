//! Svg Render
use std::sync::Arc;

use bladvak::{
    AppError, ErrorManager,
    app::BladvakPanel,
    eframe::{egui, epaint::RectShape},
    log,
};
use egui::{
    Color32, Context, CornerRadius, ImageData, ImageFit, ImageSize, Rect, Sense, TextureHandle,
    TextureOptions, Ui, Vec2, WidgetInfo, WidgetType, pos2,
};
use resvg::tiny_skia::Pixmap;

use crate::GalagoApp;

/// Svg Render Struct
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SvgRender {
    /// Texture Handle
    #[serde(skip)]
    texture_save: Option<TextureHandle>,

    /// Cached svg string
    #[serde(skip)]
    cached_svg: Option<String>,

    /// Whether to auto scale the SVG
    auto_scale: bool,

    /// Sizer for scaling the SVG
    scaler: u32,
}

impl Default for SvgRender {
    fn default() -> Self {
        Self {
            texture_save: None,
            cached_svg: None,
            auto_scale: true,
            scaler: 1,
        }
    }
}

impl SvgRender {
    /// Mark the render as stale - need to re-render
    pub fn stale_render(&mut self) {
        self.cached_svg = None;
    }

    /// Show the rendered svg
    pub fn show(&self, ui: &mut Ui) -> Result<egui::Response, ()> {
        if let Some(texture_save) = &self.texture_save {
            let texture_size = texture_save.size();
            let image_size = ImageSize {
                maintain_aspect_ratio: true,
                max_size: Vec2::INFINITY,
                fit: ImageFit::Original {
                    scale: 1.0 / self.scaler as f32,
                },
            };
            let ui_size = image_size.calc_size(
                ui.available_size(),
                Vec2::new(texture_size[0] as f32, texture_size[1] as f32),
            );
            let (rect, response) = ui.allocate_exact_size(ui_size, Sense::click());
            response.widget_info(|| {
                let mut info = WidgetInfo::new(WidgetType::Image);
                info.label = Some("rendered svg".to_string());
                info
            });
            if ui.is_rect_visible(rect) {
                let painter = ui.painter();
                let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
                painter.add(
                    RectShape::filled(rect, CornerRadius::ZERO, Color32::WHITE)
                        .with_texture(texture_save.id(), uv),
                );

                // rect
                // let visuals = ui.style().interact_selectable(&response, true);
                // let height_draw = ui_size.y.min(ui_size.x);
                // let rect = Rect::from_center_size(
                //     Pos2::new(100.0, 100.0),
                //     Vec2 {
                //         x: 0.01 * height_draw,
                //         y: 0.01 * height_draw,
                //     },
                // );
                // let rect = rect.expand(visuals.expansion);
                // let radius = 0.05 * rect.height();
                // ui.painter().rect(
                //     rect,
                //     radius,
                //     visuals.bg_fill,
                //     visuals.bg_stroke,
                //     egui::StrokeKind::Inside,
                // );
            }
            return Ok(response);
        }
        Err(())
    }
}

impl GalagoApp {
    /// Render settings
    pub fn show_render_settings(&mut self, ui: &mut Ui) {
        if ui
            .checkbox(&mut self.svg_render.auto_scale, "Auto Scale SVG")
            .changed()
        {
            self.svg_render.stale_render();
        }
        if ui
            .add_enabled(
                !self.svg_render.auto_scale,
                egui::Slider::new(&mut self.svg_render.scaler, 1..=10).text("SVG Scaler"),
            )
            .changed()
        {
            self.svg_render.stale_render();
        }
        ui.collapsing("Loaded fonts", |ui| {
            egui::ScrollArea::vertical()
                .max_height(40.0)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());
                    for font in self.usvg_options.fontdb.faces() {
                        for font_family in &font.families {
                            ui.label(format!(
                                "{} ({}-{})",
                                font_family.0,
                                font.weight.0,
                                font.stretch.to_number()
                            ));
                        }
                    }
                });
        });
    }

    /// Update the svg
    /// # Errors
    /// Return error if fails to render svg
    pub fn update_svg(&mut self, ctx: &Context) -> Result<(), Option<AppError>> {
        if self.svg_render.texture_save.is_some()
            && self
                .svg_render
                .cached_svg
                .as_deref()
                .is_some_and(|cached| self.svg == cached)
        {
            return Ok(());
        }
        log::debug!("Rendering SVG");

        if let Ok(rtree) = resvg::usvg::Tree::from_str(&self.svg, &self.usvg_options) {
            if self.svg_render.auto_scale {
                // Calculate the sizer based on the SVG size
                let size = rtree.size().width().max(rtree.size().height()) as u32;
                if size < 500 {
                    self.svg_render.scaler = 6;
                } else if size < 1000 {
                    self.svg_render.scaler = 4;
                } else if size < 2000 {
                    self.svg_render.scaler = 2;
                } else {
                    self.svg_render.scaler = 1;
                }
            }
            let (w, h) = (
                rtree.size().width() as u32 * self.svg_render.scaler,
                rtree.size().height() as u32 * self.svg_render.scaler,
            );
            let mut pixmap = Pixmap::new(w, h).ok_or_else(|| {
                Some(AppError::new(format!(
                    "Failed to create SVG Pixmap of size {w}x{h}"
                )))
            })?;

            let transform = resvg::tiny_skia::Transform {
                sx: self.svg_render.scaler as f32,
                sy: self.svg_render.scaler as f32,
                ..Default::default()
            };
            resvg::render(&rtree, transform, &mut pixmap.as_mut());

            let image = egui::ColorImage::from_rgba_unmultiplied([w as _, h as _], pixmap.data());

            let texture_loaded = ctx.load_texture(
                "svg",
                ImageData::Color(Arc::new(image)),
                TextureOptions::default(),
            );
            self.svg_render.texture_save = Some(texture_loaded);
            self.svg_render.cached_svg = Some(self.svg.to_string());
            return Ok(());
        }
        Err(None)
    }
}

/// Svg viewer and grid setting
#[derive(Debug)]
pub struct SvgViewerPanel;

impl BladvakPanel for SvgViewerPanel {
    type App = GalagoApp;

    fn name(&self) -> &str {
        "SVG viewer"
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
        app.show_render_settings(ui);
        ui.separator();
        ui.heading("Grid settings");
        app.grid.show_settings(ui);

        ui.separator();
        if ui.button("Default svg").clicked() {
            app.saved_svg = GalagoApp::BASE_SVG.to_string();
            app.svg = GalagoApp::BASE_SVG.to_string();
        }
    }

    fn has_ui(&self) -> bool {
        false
    }

    fn ui(&self, _app: &mut Self::App, _ui: &mut egui::Ui, _error_manager: &mut ErrorManager) {}
}
