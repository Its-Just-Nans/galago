use std::sync::Arc;

use egui::{
    load::SizedTexture, Context, Image, ImageData, Sense, TextureHandle, TextureOptions, Ui,
};
use resvg::tiny_skia::Pixmap;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct SvgRender {
    #[serde(skip)]
    texture_save: Option<TextureHandle>,

    cached_svg: String,
}

impl SvgRender {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, ctx: &Context, svg: &str) -> Result<(), ()> {
        if self.texture_save.is_some() && svg == self.cached_svg {
            return Ok(());
        }
        if let Ok(rtree) = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()) {
            let (w, h) = (rtree.size().width() as u32, rtree.size().height() as u32);
            let mut pixmap = Pixmap::new(w, h)
                .ok_or_else(|| format!("Failed to create SVG Pixmap of size {w}x{h}"))
                .expect("sqdfd");

            resvg::render(&rtree, Default::default(), &mut pixmap.as_mut());

            let image = egui::ColorImage::from_rgba_unmultiplied([w as _, h as _], pixmap.data());

            let texture_loaded = ctx.load_texture(
                "svg",
                ImageData::Color(Arc::new(image)),
                TextureOptions::default(),
            );
            self.texture_save = Some(texture_loaded);
            self.cached_svg = svg.to_string();
            return Ok(());
        }
        Err(())
    }

    pub fn show(&self, ui: &mut Ui) -> Result<egui::Response, ()> {
        if let Some(texture_save) = &self.texture_save {
            let sized_texture = SizedTexture::from_handle(texture_save);
            return Ok(ui.add(Image::new(sized_texture).sense(Sense::click())));
        }
        Err(())
    }
}
