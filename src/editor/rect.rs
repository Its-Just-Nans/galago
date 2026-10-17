//! Svg rect

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2},
};
use resvg::usvg::roxmltree::Node;

/// svg rect
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SvgRect {
    /// x
    x: f32,
    /// y
    y: f32,
    /// width
    width: f32,
    /// height
    height: f32,
    /// rx
    rx: Option<f32>,
    /// ry
    ry: Option<f32>,
    /// fill
    fill: Option<Color32>,
    /// Stroke
    stroke: Option<Stroke>,
}

impl SvgRect {
    /// From min max
    fn from_min_max(min: Pos2, max: Pos2) -> Self {
        SvgRect {
            x: min.x,
            y: min.y,
            width: max.x - min.x,
            height: max.y - min.y,
            rx: None,
            ry: None,
            fill: None,
            stroke: None,
        }
    }
    /// Try to parse the value
    pub(crate) fn try_parse(node: Node<'_, '_>) -> Result<Self, String> {
        let x = node
            .attribute("x")
            .ok_or("Cannot get x property")?
            .parse()
            .map_err(|_e| "Cannot parse x property")?;
        let y = node
            .attribute("y")
            .ok_or("Cannot get y property")?
            .parse()
            .map_err(|_e| "Cannot parse y property")?;
        let width = node
            .attribute("width")
            .ok_or("Cannot get width property")?
            .parse()
            .map_err(|_e| "Cannot parse width property")?;
        let height = node
            .attribute("height")
            .ok_or("Cannot get height property")?
            .parse()
            .map_err(|_e| "Cannot parse height property")?;
        Ok(SvgRect {
            x,
            y,
            width,
            height,
            rx: None,
            ry: None,
            fill: None,
            stroke: None,
        })
    }

    /// Show the node
    pub(crate) fn show(
        &mut self,
        ui: &mut egui::Ui,
        idx: usize,
        size_point: f32,
        error_manager: &mut ErrorManager,
    ) {
        let width = self.width;

        let height = self.height;
        let margin_size = if width > 40.0 || height > 40.0 {
            10.0
        } else {
            5.0
        };
        let min = Pos2::new(self.x, self.y);
        let max = Pos2::new(self.x + width, self.y + height);
        let rect = Rect::from_min_max(min, max);
        let zone = Rect::from_min_max(
            min - Vec2::new(margin_size, margin_size),
            max + Vec2::new(margin_size, margin_size),
        );
        // painter.debug_rect(zone, Color32::RED, "interact");
        let resp = ui.interact(
            zone,
            ui.id().with(("offset_rect", idx)),
            egui::Sense::click_and_drag(),
        );
        let painter = ui.painter();
        painter.rect(
            rect,
            0.0,
            Color32::RED,
            Stroke::default(),
            egui::StrokeKind::Middle,
        );
        if resp.dragged() || resp.hovered() {
            let (corner_length_width, is_top_bt) = if width > (margin_size * 4.0) {
                ((width / 5.0).max(margin_size).floor(), true)
            } else {
                ((width / 5.0).floor(), false)
            };
            let (corner_length_height, is_r_l) = if height > (margin_size * 4.0) {
                ((height / 5.0).max(margin_size), true)
            } else {
                ((height / 5.0).floor(), false)
            };

            let corner_pos = rect.right_top();
            let rect_pos = Rect::from_center_size(corner_pos, Vec2::splat(size_point));
            painter.rect_filled(rect_pos, 1.0, Color32::WHITE);
            let corner_pos = rect.left_top();
            let rect_pos = Rect::from_center_size(corner_pos, Vec2::splat(size_point));
            painter.rect_filled(rect_pos, 1.0, Color32::WHITE);
            let corner_pos = rect.right_bottom();
            let rect_pos = Rect::from_center_size(corner_pos, Vec2::splat(size_point));
            painter.rect_filled(rect_pos, 1.0, Color32::WHITE);
            let corner_pos = rect.left_bottom();
            let rect_pos = Rect::from_center_size(corner_pos, Vec2::splat(size_point));
            painter.rect_filled(rect_pos, 1.0, Color32::WHITE);

            if error_manager.is_debug() {
                // let center = Rect::from_min_max(
                //     rect.min + Vec2::new(corner_length_width, corner_length_height),
                //     rect.max - Vec2::new(corner_length_width, corner_length_height),
                // );
                painter.debug_rect(zone, Color32::WHITE, "center");
                let corner_top_left = Rect::from_min_max(
                    zone.min,
                    rect.min + Vec2::new(corner_length_width, corner_length_height),
                );
                painter.debug_rect(corner_top_left, Color32::YELLOW, "nw");
                let corner_top_right = Rect::from_min_max(
                    Pos2::new(rect.max.x - corner_length_width, zone.min.y),
                    Pos2::new(zone.max.x, rect.min.y + corner_length_height),
                );
                painter.debug_rect(corner_top_right, Color32::BLUE, "ne");

                let corner_bottom_left = Rect::from_min_max(
                    Pos2::new(zone.min.x, rect.max.y - corner_length_height),
                    Pos2::new(rect.min.x + corner_length_width, zone.max.y),
                );
                painter.debug_rect(corner_bottom_left, Color32::DARK_BLUE, "sw");

                let corner_bottom_right = Rect::from_min_max(
                    rect.max - Vec2::new(corner_length_width, corner_length_height),
                    zone.max,
                );
                painter.debug_rect(corner_bottom_right, Color32::PURPLE, "se");

                // Zones between the corners
                if is_top_bt {
                    let zone_top = Rect::from_min_max(
                        Pos2::new(rect.min.x + corner_length_width, zone.min.y),
                        Pos2::new(
                            rect.max.x - corner_length_width,
                            rect.min.y + corner_length_height,
                        ),
                    );
                    painter.debug_rect(zone_top, Color32::GREEN, "top");

                    let zone_bottom = Rect::from_min_max(
                        Pos2::new(
                            rect.min.x + corner_length_width,
                            rect.max.y - corner_length_height,
                        ),
                        Pos2::new(rect.max.x - corner_length_width, zone.max.y),
                    );
                    painter.debug_rect(zone_bottom, Color32::MAGENTA, "bottom");
                }
                if is_r_l {
                    let zone_left = Rect::from_min_max(
                        Pos2::new(zone.min.x, rect.min.y + corner_length_height),
                        Pos2::new(
                            rect.min.x + corner_length_width,
                            rect.max.y - corner_length_height,
                        ),
                    );
                    painter.debug_rect(zone_left, Color32::CYAN, "left");

                    let zone_right = Rect::from_min_max(
                        Pos2::new(
                            rect.max.x - corner_length_width,
                            rect.min.y + corner_length_height,
                        ),
                        Pos2::new(zone.max.x, rect.max.y - corner_length_height),
                    );
                    painter.debug_rect(zone_right, Color32::KHAKI, "right");
                }
            }
            if let Some(pos) = resp.interact_pointer_pos() {
                painter.circle_filled(pos, 1.0, Color32::WHITE);
                let left = pos.x < rect.min.x + corner_length_width;
                let right = pos.x > rect.max.x - corner_length_width;
                let top = pos.y < rect.min.y + corner_length_height;
                let bottom = pos.y > rect.max.y - corner_length_height;
                let pos = pos.floor();
                if left && top {
                    *self = Self::from_min_max(pos, rect.max);
                } else if right && top {
                    *self = Self::from_min_max(
                        Pos2::new(rect.min.x, pos.y),
                        Pos2::new(pos.x, rect.max.y),
                    );
                } else if left && bottom {
                    *self = Self::from_min_max(
                        Pos2::new(pos.x, rect.min.y),
                        Pos2::new(rect.max.x, pos.y),
                    );
                } else if right && bottom {
                    *self = Self::from_min_max(rect.min, pos);
                } else if is_top_bt && top {
                    // Move top edge
                    *self = Self::from_min_max(Pos2::new(rect.min.x, pos.y), rect.max);
                } else if is_top_bt && bottom {
                    // Move bottom edge
                    *self = Self::from_min_max(rect.min, Pos2::new(rect.max.x, pos.y));
                } else if is_r_l && left {
                    // Move left edge
                    *self = Self::from_min_max(Pos2::new(pos.x, rect.min.y), rect.max);
                } else if is_r_l && right {
                    // Move right edge
                    *self = Self::from_min_max(rect.min, Pos2::new(pos.x, rect.max.y));
                } else {
                    let delta = resp.drag_delta();
                    *self = Self::from_min_max(rect.min + delta, rect.max + delta);
                }
            }
        }
    }
}
