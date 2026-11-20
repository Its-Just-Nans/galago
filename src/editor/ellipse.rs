//! svg ellipse

use bladvak::{
    ErrorManager,
    eframe::{
        egui::{self, Color32, Pos2, Rect, Shape, Stroke, Vec2},
        epaint::EllipseShape,
    },
};
use resvg::usvg::roxmltree::Node;

use crate::editor::COLOR_DEBUG;

/// svg ellipse
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SvgEllipse {
    /// cx
    cx: f32,
    /// cy
    cy: f32,
    /// rx
    rx: f32,
    /// ry
    ry: f32,
}

impl SvgEllipse {
    /// Try to parse the value
    pub(crate) fn try_parse(node: Node<'_, '_>) -> Result<Self, String> {
        let cx = node
            .attribute("cx")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse cx property")?;
        let cy = node
            .attribute("cy")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse cy property")?;
        let rx = node
            .attribute("rx")
            .ok_or("Cannot get rx property")?
            .parse()
            .map_err(|_e| "Cannot parse rx property")?;
        let ry = node
            .attribute("ry")
            .ok_or("Cannot get ry property")?
            .parse()
            .map_err(|_e| "Cannot parse ry property")?;
        Ok(Self { cx, cy, rx, ry })
    }

    /// show the node
    pub(crate) fn show(
        &mut self,
        ui: &mut egui::Ui,
        idx: usize,
        size_point: f32,
        error_manager: &mut ErrorManager,
    ) {
        let p1 = Pos2::new(self.cx - self.rx, self.cy + self.ry);
        let p2 = Pos2::new(self.cx + self.rx, self.cy - self.ry);
        let zone = Rect::from_two_pos(p1, p2);
        let zone_inter = zone.expand(size_point);
        if error_manager.is_debug() {
            ui.painter().debug_rect(zone_inter, COLOR_DEBUG, "ellipse");
        }
        let resp = ui.interact(
            zone_inter,
            ui.scope_id().with(("offset_rect", idx)),
            egui::Sense::click_and_drag(),
        );
        let painter = ui.painter();
        let center = Pos2::new(self.cx, self.cy);
        painter.add(Shape::Ellipse(EllipseShape {
            center,
            radius: Vec2::new(self.rx, self.ry),
            fill: Color32::TRANSPARENT,
            stroke: Stroke::new(1.0, Color32::RED),
            angle: 0.0,
        }));
        if resp.dragged() || resp.hovered() {
            let center_rect = Rect::from_center_size(center, Vec2::splat(size_point));
            painter.rect_filled(center_rect, 1.0, Color32::WHITE);
            if error_manager.is_debug() {
                painter.debug_rect(center_rect, COLOR_DEBUG, "center");
            }

            let radius_x = Pos2::new(self.cx + self.rx, self.cy);
            let rect_x = Rect::from_center_size(radius_x, Vec2::splat(size_point));
            painter.rect_filled(rect_x, 1.0, Color32::WHITE);
            if error_manager.is_debug() {
                painter.debug_rect(rect_x, COLOR_DEBUG, "x");
            }

            let radius_y = Pos2::new(self.cx, self.cy + self.ry);
            let rect_y = Rect::from_center_size(radius_y, Vec2::splat(size_point));
            painter.rect_filled(rect_y, 1.0, Color32::WHITE);
            if error_manager.is_debug() {
                painter.debug_rect(rect_y, COLOR_DEBUG, "y");
            }

            if let Some(pos) = resp.interact_pointer_pos() {
                painter.circle_filled(pos, 1.0, Color32::WHITE);
                if center_rect.contains(pos) {
                    self.cx = pos.x;
                    self.cy = pos.y;
                } else if rect_x.contains(pos) {
                    self.rx = pos.x - self.cx;
                } else if rect_y.contains(pos) {
                    self.ry = pos.y - self.cy;
                } else {
                    let delta = resp.drag_delta();
                    self.cx += delta.x;
                    self.cy += delta.y;
                }
            }
        }
    }
}
