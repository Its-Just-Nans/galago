//! Svg Polygon

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Color32, Pos2, Rect, Shape, Stroke, Vec2},
};
use resvg::usvg::roxmltree::Node;

use crate::editor::path::bounding_box;

/// Polygon
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(default)]
pub(crate) struct SvgPolygon {
    /// points
    points: Vec<Pos2>,
    /// points
    #[serde(skip)]
    shape: Shape,
    /// bounding box
    bounding_box: Rect,
}

impl Default for SvgPolygon {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            shape: Shape::Noop,
            bounding_box: Rect::ZERO,
        }
    }
}

/// Shape from points
fn shape_from_points(points: &[Pos2]) -> Shape {
    Shape::convex_polygon(
        points.to_vec(),
        Color32::BROWN,
        Stroke::new(1.0, Color32::BLACK),
    )
}

impl SvgPolygon {
    /// try to parse
    pub(crate) fn try_parse(node: Node<'_, '_>) -> Result<Self, String> {
        let points = node
            .attribute("points")
            .ok_or("Cannot get points property")?;
        let points = points
            .split_whitespace()
            .map(|s| s.parse::<f32>().map_err(|_| "Invalid coordinate"))
            .collect::<Result<Vec<_>, _>>()?
            .as_chunks::<2>().0.iter()
            .map(|c| Pos2::new(c[0], c[1]))
            .collect::<Vec<Pos2>>();
        let bounding_box = bounding_box(&points).ok_or("Cannot get bouding box")?;
        let shape = shape_from_points(&points);
        Ok(Self {
            points,
            shape,
            bounding_box,
        })
    }
    /// show the path
    pub(crate) fn show(
        &mut self,
        ui: &mut egui::Ui,
        idx: usize,
        size_point: f32,
        error_manager: &mut ErrorManager,
    ) {
        let is_debug = error_manager.is_debug();
        let bounding = self.bounding_box.expand(size_point);
        let resp = ui.interact(
            bounding,
            ui.id().with(("path", "interact", idx)),
            egui::Sense::click_and_drag(),
        );
        if is_debug {
            ui.painter().debug_rect(bounding, Color32::YELLOW, "shape");
        }
        ui.painter().add(self.shape.clone());
        let mut moved = false;
        if resp.hovered() {
            for one_point in &mut *self.points {
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
            for one_point in &mut self.points {
                *one_point += resp.drag_delta();
                moved = true;
            }
        }
        if moved {
            self.shape = shape_from_points(&self.points);
            if let Some(b) = bounding_box(&self.points) {
                self.bounding_box = b;
            }
        }
    }
}
