//! SVG editor

use bladvak::{
    ErrorManager,
    app::BladvakPanel,
    eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2},
};
use resvg::usvg::{Group, Node, Options};

use crate::GalagoApp;

/// Data
#[derive(serde::Deserialize, serde::Serialize, Default, Debug, Clone)]
pub struct ShapeEditor {
    /// shapes
    shapes: Vec<Rect>,
}

/// recursivily add shape
fn recursive_add(rects: &mut Vec<Rect>, group: &Group) {
    for one_node in group.children() {
        match one_node {
            Node::Path(p) => {
                let rect = p.abs_bounding_box();
                let min = Pos2::new(rect.left(), rect.top());
                let max = Pos2::new(rect.right(), rect.bottom());
                rects.push(Rect::from_min_max(min, max));
            }
            Node::Group(g) => {
                recursive_add(rects, g);
            }
            _ => {}
        }
    }
}

impl ShapeEditor {
    /// Show ui
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        svg: &str,
        usvg_options: &Options<'_>,
        error_manager: &mut ErrorManager,
    ) {
        if ui.button("Convert to shape").clicked() {
            match resvg::usvg::Tree::from_str(svg, usvg_options) {
                Ok(tree) => {
                    recursive_add(&mut self.shapes, tree.root());
                }
                Err(_e) => {
                    error_manager.add_error("Cannot conver the svg");
                }
            }
        }
    }

    /// show in scene
    #[allow(clippy::too_many_lines)]
    pub(crate) fn show_in_scene(&mut self, ui: &mut egui::Ui, error_manager: &mut ErrorManager) {
        for (idx, rect) in self.shapes.iter_mut().enumerate() {
            let width = rect.width();
            let height = rect.height();
            let margin_size = if width > 40.0 || height > 40.0 {
                10.0
            } else {
                5.0
            };
            let zone = Rect::from_min_max(
                rect.min - Vec2::new(margin_size, margin_size),
                rect.max + Vec2::new(margin_size, margin_size),
            );
            // painter.debug_rect(zone, Color32::RED, "interact");
            let resp = ui.interact(
                zone,
                ui.id().with(("offset_rect", idx)),
                egui::Sense::click_and_drag(),
            );
            let painter = ui.painter();
            painter.rect(
                *rect,
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
                        *rect = Rect::from_min_max(pos, rect.max);
                    } else if right && top {
                        *rect = Rect::from_min_max(
                            Pos2::new(rect.min.x, pos.y),
                            Pos2::new(pos.x, rect.max.y),
                        );
                    } else if left && bottom {
                        *rect = Rect::from_min_max(
                            Pos2::new(pos.x, rect.min.y),
                            Pos2::new(rect.max.x, pos.y),
                        );
                    } else if right && bottom {
                        *rect = Rect::from_min_max(rect.min, pos);
                    } else if is_top_bt && top {
                        // Move top edge
                        *rect = Rect::from_min_max(Pos2::new(rect.min.x, pos.y), rect.max);
                    } else if is_top_bt && bottom {
                        // Move bottom edge
                        *rect = Rect::from_min_max(rect.min, Pos2::new(rect.max.x, pos.y));
                    } else if is_r_l && left {
                        // Move left edge
                        *rect = Rect::from_min_max(Pos2::new(pos.x, rect.min.y), rect.max);
                    } else if is_r_l && right {
                        // Move right edge
                        *rect = Rect::from_min_max(rect.min, Pos2::new(pos.x, rect.max.y));
                    } else {
                        let delta = resp.drag_delta();
                        *rect = Rect::from_min_max(rect.min + delta, rect.max + delta);
                    }
                }
            }
        }
    }
}

/// Panel
#[derive(Debug)]
pub struct ShapeEditorPanel;

impl BladvakPanel for ShapeEditorPanel {
    type App = GalagoApp;

    fn name(&self) -> &'static str {
        "Shape editor"
    }

    fn has_settings(&self) -> bool {
        false
    }

    fn ui_settings(
        &self,
        _app: &mut Self::App,
        _ui: &mut egui::Ui,
        _error_manager: &mut ErrorManager,
    ) {
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&self, app: &mut Self::App, ui: &mut egui::Ui, error_manager: &mut ErrorManager) {
        let Some(document) = app.documents.get_current_doc_mut() else {
            return;
        };
        document
            .shape_editor
            .show(ui, &document.svg, &app.usvg_options, error_manager);
    }
}
