//! SVG Circle

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2},
};
use resvg::usvg::roxmltree::Node;

use crate::editor::DEFAULT_COLOR;

/// svg circle
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SvgCircle {
    /// r
    r: f32,
    /// cx
    cx: f32,
    /// ry
    cy: f32,
    /// fill
    pub fill: Option<Color32>,
    /// Stroke
    pub stroke: Option<Stroke>,
}

impl SvgCircle {
    /// Try to parse the value
    pub(crate) fn try_parse(node: Node<'_, '_>) -> Result<Self, String> {
        let cx = node
            .attribute("cx")
            .ok_or("Cannot get cx property")?
            .parse()
            .map_err(|_e| "Cannot parse cx property")?;
        let cy = node
            .attribute("cy")
            .ok_or("Cannot get cy property")?
            .parse()
            .map_err(|_e| "Cannot parse cy property")?;
        let r = node
            .attribute("r")
            .ok_or("Cannot get r property")?
            .parse()
            .map_err(|_e| "Cannot parse r property")?;
        Ok(SvgCircle {
            cx,
            cy,
            r,
            fill: None,
            stroke: None,
        })
    }

    /// Show the node
    #[allow(clippy::too_many_lines)]
    pub(crate) fn show(
        &mut self,
        ui: &mut egui::Ui,
        idx: usize,
        size_point: f32,
        _error_manager: &mut ErrorManager,
    ) {
        let margin_size = 10.0;
        let min = Pos2::new(self.cx - self.r, self.cy - self.r);
        let max = Pos2::new(self.cx + self.r, self.cy + self.r);
        let center = Pos2::new(self.cx, self.cy);
        let zone = Rect::from_min_max(
            min - Vec2::new(margin_size, margin_size),
            max + Vec2::new(margin_size, margin_size),
        );
        let resp = ui.interact(
            zone,
            ui.id().with(("offset_circle", idx)),
            egui::Sense::click_and_drag(),
        );
        let painter = ui.painter();
        painter.circle(
            center,
            self.r,
            self.fill.unwrap_or(DEFAULT_COLOR),
            self.stroke.unwrap_or_default(),
        );
        if resp.dragged() || resp.hovered() {
            let rect_pos = Rect::from_center_size(center, Vec2::splat(size_point));
            painter.rect_filled(rect_pos, 1.0, Color32::WHITE);
        }
    }
}
