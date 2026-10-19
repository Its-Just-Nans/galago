//! Svg Path

use bladvak::{
    ErrorManager,
    eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2},
};
use resvg::usvg::roxmltree::Node;
use svgtypes::PathSegment;

use crate::path::InnerSvgPath;

/// Svg path
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SvgPath {
    /// inner path
    #[serde(skip)]
    d: InnerSvgPath,
    /// bounding box
    bounding_box: Rect,
}

/// get the bounding box
pub(crate) fn bounding_box(points: &[Pos2]) -> Option<Rect> {
    let first = *points.first()?;

    let (mut min_x, mut min_y) = (first.x, first.y);
    let (mut max_x, mut max_y) = (first.x, first.y);

    for pos in points.iter().skip(1) {
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);
    }

    Some(Rect::from_min_max(
        Pos2::new(min_x, min_y),
        Pos2::new(max_x, max_y),
    ))
}

impl SvgPath {
    /// try to parse
    pub(crate) fn try_parse(node: Node<'_, '_>) -> Result<Self, String> {
        let d = node.attribute("d").ok_or("Cannot get d property")?;
        let d = InnerSvgPath::parse(d)?;
        let bounding_box = Rect::ZERO;
        Ok(Self { d, bounding_box })
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
        let mut p = Pos2::new(0.0, 0.0);
        let detection_zone = 15.0;
        let mut moved = false;
        let mut new_bb = self.bounding_box;
        #[allow(clippy::cast_possible_truncation)]
        for one_point in &mut self.d.items {
            match &mut one_point.inner {
                PathSegment::MoveTo { abs, x, y } => {
                    let (x, y) = (*x as f32, *y as f32);
                    let (current_x, current_y) = if *abs { (x, y) } else { (p.x + x, p.y + y) };
                    p = Pos2::new(current_x, current_y);
                    let rect = Rect::from_center_size(p, Vec2::splat(size_point));
                    let zone = rect.expand(detection_zone);
                    if is_debug {
                        ui.painter().debug_rect(zone, Color32::YELLOW, "point");
                    }
                    ui.painter().rect(
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
                        let new_pos = p + resp.drag_delta();
                        one_point.inner = PathSegment::MoveTo {
                            abs: true,
                            x: f64::from(new_pos.x),
                            y: f64::from(new_pos.y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }
                }
                PathSegment::LineTo { abs, x, y } => {
                    let start = p;
                    let (x, y) = (*x as f32, *y as f32);
                    let (end_x, end_y) = if *abs { (x, y) } else { (p.x + x, p.y + y) };
                    let end = Pos2::new(end_x, end_y);
                    p = end;
                    ui.painter()
                        .line_segment([start, end], Stroke::new(1.0, Color32::BLACK));
                    let rect_end = Rect::from_center_size(p, Vec2::splat(size_point));
                    let zone = rect_end.expand(detection_zone);
                    if is_debug {
                        ui.painter().debug_rect(zone, Color32::YELLOW, "point");
                    }
                    ui.painter().rect(
                        rect_end,
                        0.0,
                        Color32::WHITE,
                        Stroke::default(),
                        egui::StrokeKind::Middle,
                    );
                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = p + resp.drag_delta();
                        one_point.inner = PathSegment::LineTo {
                            abs: true,
                            x: f64::from(new_pos.x),
                            y: f64::from(new_pos.y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }
                }
                _ => {}
            }
            if !new_bb.contains(p) {
                new_bb = new_bb.union(Rect::from_pos(p));
            }
        }
        if resp.hovered() {
            // TODO
        }
        if resp.dragged() && !moved {
            // for one_point in points {
            //     *one_point += resp.drag_delta();
            //     moved = true;
            // }
        }
        self.bounding_box = new_bb;
    }
}
