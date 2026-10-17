//! Svg Path

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2},
};

use crate::path::InnerSvgPath;

/// Svg path
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SvgPath {
    /// inner path
    #[serde(skip)]
    inner: InnerSvgPath,
    /// points rendered
    points: Vec<Pos2>,
}

// TODO CHANGE
impl Default for SvgPath {
    fn default() -> Self {
        Self {
            inner: InnerSvgPath { items: Vec::new() },
            points: vec![
                Pos2::new(10.0, 10.0),
                Pos2::new(10.0, 60.0),
                Pos2::new(60.0, 70.0),
                Pos2::new(150.0, 150.0),
            ],
        }
    }
}

/// get the bouding box
fn bounding_box(points: &[Pos2]) -> Option<Rect> {
    let first = *points.first()?;

    let (mut min_x, mut min_y) = (first.x, first.y);
    let (mut max_x, mut max_y) = (first.x, first.y);

    for p in points.iter().skip(1) {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }

    Some(Rect::from_min_max(
        Pos2::new(min_x, min_y),
        Pos2::new(max_x, max_y),
    ))
}

impl SvgPath {
    /// show the path
    pub(crate) fn show(
        &mut self,
        ui: &mut egui::Ui,
        idx: usize,
        size_point: f32,
        error_manager: &mut ErrorManager,
    ) {
        let is_debug = error_manager.is_debug();
        let points = &mut self.points;
        ui.painter().add(egui::Shape::convex_polygon(
            points.clone(),
            egui::Color32::GREEN,                         // Fill color
            egui::Stroke::new(1.0, egui::Color32::BLACK), // Border stroke
        ));
        let bounding = bounding_box(points).unwrap(); // TODO
        let bounding = bounding.expand(size_point);
        let resp = ui.interact(
            bounding,
            ui.id().with(("offset_rect", idx, "interact")),
            egui::Sense::click_and_drag(),
        );
        if is_debug {
            ui.painter().debug_rect(bounding, Color32::YELLOW, "shape");
        }
        let mut moved = false;
        if resp.hovered() {
            for (_index_point, one_point) in points.iter_mut().enumerate() {
                let rect = Rect::from_center_size(*one_point, Vec2::splat(size_point));
                let zone = rect.expand(15.0);

                if is_debug {
                    ui.painter().debug_rect(zone, Color32::YELLOW, "point");
                }
                let painter = ui.painter();
                painter.rect(
                    rect,
                    0.0,
                    Color32::WHITE,
                    Stroke::default(),
                    egui::StrokeKind::Middle,
                );
                if resp.dragged()
                    && let Some(pos) = resp.interact_pointer_pos()
                    && zone.contains(pos)
                {
                    moved = true;
                    *one_point += resp.drag_delta();
                }
            }
        }
        if resp.dragged() && !moved {
            for one_point in points {
                *one_point += resp.drag_delta();
            }
        }
    }
}
