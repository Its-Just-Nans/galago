//! Svg Render
use std::{cmp::max, sync::Arc};

use bladvak::{eframe::egui, AppError};
use egui::{
    load::SizedTexture, Context, Image, ImageData, Sense, TextureHandle, TextureOptions, Ui,
};
use resvg::tiny_skia::Pixmap;

/// Svg Render Struct
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SvgRender {
    /// Texture Handle
    #[serde(skip)]
    texture_save: Option<TextureHandle>,

    /// Cached svg string
    cached_svg: String,

    /// Whether to auto scale the SVG
    auto_scale: bool,

    /// Sizer for scaling the SVG
    scaler: u32,

    /// Whether to reload the SVG
    need_reload: bool,
}

impl Default for SvgRender {
    fn default() -> Self {
        Self {
            texture_save: None,
            cached_svg: String::new(),
            auto_scale: true,
            scaler: 1,
            need_reload: false,
        }
    }
}

impl SvgRender {
    /// Create a new SvgRender
    pub fn new() -> Self {
        Default::default()
    }

    /// Title of the SVG Render
    pub fn title(&self) -> &'static str {
        "SVG Render"
    }

    /// Settings
    pub fn show_settings(&mut self, ui: &mut Ui) {
        let current_auto_scale = self.auto_scale;
        ui.checkbox(&mut self.auto_scale, "Auto Scale SVG");
        let current_value = self.scaler;
        ui.add_enabled(
            !self.auto_scale,
            egui::Slider::new(&mut self.scaler, 1..=10).text("SVG Scaler"),
        );
        self.need_reload = self.scaler != current_value || self.auto_scale != current_auto_scale;
    }

    /// Update the svg
    pub fn update(&mut self, ctx: &Context, svg: &str) -> Result<(), Option<AppError>> {
        if !self.need_reload && self.texture_save.is_some() && svg == self.cached_svg {
            return Ok(());
        }

        if let Ok(rtree) = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()) {
            if self.auto_scale {
                // Calculate the sizer based on the SVG size
                let size = max(rtree.size().width() as u32, rtree.size().height() as u32);
                if size < 500 {
                    self.scaler = 6;
                } else if size < 1000 {
                    self.scaler = 4;
                } else if size < 2000 {
                    self.scaler = 2;
                } else {
                    self.scaler = 1;
                }
            }
            let (w, h) = (
                rtree.size().width() as u32 * self.scaler,
                rtree.size().height() as u32 * self.scaler,
            );
            let mut pixmap = Pixmap::new(w, h).ok_or_else(|| {
                Some(AppError::new(format!(
                    "Failed to create SVG Pixmap of size {w}x{h}"
                )))
            })?;

            let transform = resvg::tiny_skia::Transform {
                sx: self.scaler as f32,
                sy: self.scaler as f32,
                ..Default::default()
            };
            resvg::render(&rtree, transform, &mut pixmap.as_mut());

            let image = egui::ColorImage::from_rgba_unmultiplied([w as _, h as _], pixmap.data());

            let texture_loaded = ctx.load_texture(
                "svg",
                ImageData::Color(Arc::new(image)),
                TextureOptions::default(),
            );
            self.texture_save = Some(texture_loaded);
            self.cached_svg = svg.to_string();
            self.need_reload = false;
            return Ok(());
        }
        Err(None)
    }

    /// Show the rendered svg
    pub fn show(&self, ui: &mut Ui) -> Result<egui::Response, ()> {
        if let Some(texture_save) = &self.texture_save {
            let sized_texture = SizedTexture::from_handle(texture_save);
            return Ok(ui.add(
                Image::new(sized_texture)
                    .fit_to_original_size(1.0 / self.scaler as f32)
                    .sense(Sense::click()),
            ));
        }
        Err(())
    }
}
