//! svg line

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2},
};
use resvg::usvg::roxmltree::Node;

use crate::editor::COLOR_DEBUG;

/// svg line
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SvgLine {
    /// x1
    x1: f32,
    /// y1
    y1: f32,
    /// x2
    x2: f32,
    /// y2
    y2: f32,
}

impl SvgLine {
    /// Try to parse the value
    pub(crate) fn try_parse(node: Node<'_, '_>) -> Result<Self, String> {
        let x1 = node
            .attribute("x1")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse x property")?;
        let y1 = node
            .attribute("y1")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse y property")?;
        let x2 = node
            .attribute("x2")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse x property")?;
        let y2 = node
            .attribute("y2")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse y property")?;
        Ok(Self { x1, y1, x2, y2 })
    }

    /// show the node
    pub(crate) fn show(
        &mut self,
        ui: &mut egui::Ui,
        idx: usize,
        size_point: f32,
        error_manager: &mut ErrorManager,
    ) {
        let p1 = Pos2::new(self.x1, self.y1);
        let p2 = Pos2::new(self.x2, self.y2);
        let zone = Rect::from_two_pos(p1, p2);
        let zone_inter = zone.expand(size_point);
        if error_manager.is_debug() {
            ui.painter().debug_rect(zone_inter, COLOR_DEBUG, "line");
        }
        let resp = ui.interact(
            zone_inter,
            ui.scope_id().with(("offset_rect", idx)),
            egui::Sense::click_and_drag(),
        );
        let painter = ui.painter();
        painter.line_segment([p1, p2], Stroke::new(1.0, Color32::RED));
        if resp.dragged() || resp.hovered() {
            let rect_p1 = Rect::from_center_size(p1, Vec2::splat(size_point));
            painter.rect_filled(rect_p1, 1.0, Color32::WHITE);

            let rect_p2 = Rect::from_center_size(p2, Vec2::splat(size_point));
            painter.rect_filled(rect_p2, 1.0, Color32::WHITE);

            if let Some(pos) = resp.interact_pointer_pos() {
                painter.circle_filled(pos, 1.0, Color32::WHITE);
                if rect_p1.contains(pos) {
                    self.x1 = pos.x;
                    self.y1 = pos.y;
                } else if rect_p2.contains(pos) {
                    self.x2 = pos.x;
                    self.y2 = pos.y;
                } else {
                    let delta = resp.drag_delta();
                    self.x1 += delta.x;
                    self.y1 += delta.y;
                    self.x2 += delta.x;
                    self.y2 += delta.y;
                }
            }
        }
    }
}
