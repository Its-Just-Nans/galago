//! Svg Path

use bladvak::{
    ErrorManager,
    eframe::{
        egui::{self, Color32, Pos2, Rect, Shape, Stroke, Vec2},
        epaint::{CubicBezierShape, PathStroke, QuadraticBezierShape},
    },
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
    #[allow(clippy::too_many_lines)]
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
        let mut last_control = Pos2::new(0.0, 0.0);
        let mut last_m = Pos2::new(0.0, 0.0);
        let detection_zone = 5.0;
        let mut moved = false;
        let mut new_bb = Rect::ZERO; // self.bounding_box; // TODO
        #[allow(clippy::cast_possible_truncation)]
        for one_point in &mut self.d.items {
            match one_point {
                PathSegment::MoveTo { abs, x, y } => {
                    let (x, y) = (*x as f32, *y as f32);
                    let (current_x, current_y) = if *abs { (x, y) } else { (p.x + x, p.y + y) };
                    p = Pos2::new(current_x, current_y);
                    last_m = p;
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
                    if new_bb == Rect::ZERO {
                        new_bb = Rect::from_pos(p);
                    }
                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = p + resp.drag_delta();
                        *one_point = PathSegment::MoveTo {
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
                        *one_point = PathSegment::LineTo {
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
                PathSegment::HorizontalLineTo { abs, x } => {
                    let start = p;
                    let x = *x as f32;
                    let (end_x, end_y) = if *abs { (x, p.y) } else { (p.x + x, p.y) };
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
                        *one_point = PathSegment::HorizontalLineTo {
                            abs: true,
                            x: f64::from(new_pos.x),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }
                }
                PathSegment::VerticalLineTo { abs, y } => {
                    let start = p;
                    let y = *y as f32;
                    let (end_x, end_y) = if *abs { (p.x, y) } else { (p.x, p.y + y) };
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
                        *one_point = PathSegment::VerticalLineTo {
                            abs: true,
                            y: f64::from(new_pos.y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }
                }
                PathSegment::Quadratic { abs, x1, y1, x, y } => {
                    let start = p;
                    let (x1, y1) = (*x1 as f32, *y1 as f32);
                    let middle = if *abs {
                        Pos2::new(x1, y1)
                    } else {
                        Pos2::new(p.x + x1, p.y + y1)
                    };
                    last_control = middle;
                    let (x, y) = (*x as f32, *y as f32);
                    let end = if *abs {
                        Pos2::new(x, y)
                    } else {
                        Pos2::new(p.x + x, p.y + y)
                    };
                    p = end;
                    ui.painter()
                        .add(Shape::QuadraticBezier(QuadraticBezierShape {
                            points: [start, middle, end],
                            closed: false,
                            fill: Color32::TRANSPARENT,
                            stroke: PathStroke::new(1.0, Color32::BLACK),
                        }));
                    let rect_middle = Rect::from_center_size(middle, Vec2::splat(size_point));
                    let zone = rect_middle.expand(detection_zone);
                    if is_debug {
                        ui.painter().debug_rect(zone, Color32::YELLOW, "point");
                    }
                    ui.painter().circle(
                        middle,
                        size_point / 2.0,
                        Color32::WHITE,
                        Stroke::default(),
                    );
                    ui.painter()
                        .line_segment([start, middle], Stroke::new(1.0, Color32::RED));
                    if !new_bb.contains(middle) {
                        new_bb = new_bb.union(Rect::from_pos(middle));
                    }

                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = middle + resp.drag_delta();
                        *one_point = PathSegment::Quadratic {
                            abs: true,
                            x1: f64::from(new_pos.x),
                            y1: f64::from(new_pos.y),
                            x: f64::from(x),
                            y: f64::from(y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }

                    let rect_end = Rect::from_center_size(end, Vec2::splat(size_point));
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
                    ui.painter()
                        .line_segment([middle, end], Stroke::new(1.0, Color32::RED));
                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = end + resp.drag_delta();
                        *one_point = PathSegment::Quadratic {
                            abs: true,
                            x1: f64::from(x1),
                            y1: f64::from(y1),
                            x: f64::from(new_pos.x),
                            y: f64::from(new_pos.y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }
                }
                PathSegment::SmoothQuadratic { abs, x, y } => {
                    let start = p;
                    let middle = Pos2::new(2.0 * p.x - last_control.x, 2.0 * p.y - last_control.y);

                    last_control = middle;
                    let (x, y) = (*x as f32, *y as f32);
                    let end = if *abs {
                        Pos2::new(x, y)
                    } else {
                        Pos2::new(p.x + x, p.y + y)
                    };
                    p = end;
                    ui.painter()
                        .add(Shape::QuadraticBezier(QuadraticBezierShape {
                            points: [start, middle, end],
                            closed: false,
                            fill: Color32::TRANSPARENT,
                            stroke: PathStroke::new(1.0, Color32::BLACK),
                        }));
                    // ui.painter()
                    //     .line_segment([start, middle], Stroke::new(1.0, Color32::RED));

                    let rect_end = Rect::from_center_size(end, Vec2::splat(size_point));
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
                    ui.painter()
                        .line_segment([middle, end], Stroke::new(1.0, Color32::RED));
                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = end + resp.drag_delta();
                        *one_point = PathSegment::SmoothQuadratic {
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

                PathSegment::CurveTo {
                    abs,
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                } => {
                    let start = p;
                    let (x1, y1) = (*x1 as f32, *y1 as f32);
                    let middle1 = if *abs {
                        Pos2::new(x1, y1)
                    } else {
                        Pos2::new(p.x + x1, p.y + y1)
                    };
                    let (x2, y2) = (*x2 as f32, *y2 as f32);
                    let middle2 = if *abs {
                        Pos2::new(x2, y2)
                    } else {
                        Pos2::new(p.x + x2, p.y + y2)
                    };
                    last_control = middle2;
                    let (x, y) = (*x as f32, *y as f32);
                    let end = if *abs {
                        Pos2::new(x, y)
                    } else {
                        Pos2::new(p.x + x, p.y + y)
                    };
                    p = end;
                    ui.painter().add(Shape::CubicBezier(CubicBezierShape {
                        points: [start, middle1, middle2, end],
                        closed: false,
                        fill: Color32::TRANSPARENT,
                        stroke: PathStroke::new(1.0, Color32::BLACK),
                    }));
                    let rect_middle = Rect::from_center_size(middle1, Vec2::splat(size_point));
                    let zone = rect_middle.expand(detection_zone);
                    if is_debug {
                        ui.painter().debug_rect(zone, Color32::YELLOW, "point");
                    }
                    ui.painter().circle(
                        middle1,
                        size_point / 2.0,
                        Color32::WHITE,
                        Stroke::default(),
                    );
                    ui.painter()
                        .line_segment([start, middle1], Stroke::new(1.0, Color32::RED));
                    if !new_bb.contains(middle1) {
                        new_bb = new_bb.union(Rect::from_pos(middle1));
                    }

                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = middle1 + resp.drag_delta();
                        *one_point = PathSegment::CurveTo {
                            abs: true,
                            x1: f64::from(new_pos.x),
                            y1: f64::from(new_pos.y),
                            x2: f64::from(x2),
                            y2: f64::from(y2),
                            x: f64::from(x),
                            y: f64::from(y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }

                    let rect_middle = Rect::from_center_size(middle2, Vec2::splat(size_point));
                    let zone = rect_middle.expand(detection_zone);
                    if is_debug {
                        ui.painter().debug_rect(zone, Color32::YELLOW, "point");
                    }
                    ui.painter().circle(
                        middle2,
                        size_point / 2.0,
                        Color32::WHITE,
                        Stroke::default(),
                    );
                    if !new_bb.contains(middle2) {
                        new_bb = new_bb.union(Rect::from_pos(middle2));
                    }

                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = middle2 + resp.drag_delta();
                        *one_point = PathSegment::CurveTo {
                            abs: true,
                            x1: f64::from(x1),
                            y1: f64::from(y1),
                            x2: f64::from(new_pos.x),
                            y2: f64::from(new_pos.y),
                            x: f64::from(x),
                            y: f64::from(y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }

                    let rect_end = Rect::from_center_size(end, Vec2::splat(size_point));
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
                    ui.painter()
                        .line_segment([middle2, end], Stroke::new(1.0, Color32::RED));
                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = end + resp.drag_delta();
                        *one_point = PathSegment::CurveTo {
                            abs: true,
                            x1: f64::from(x1),
                            y1: f64::from(y1),
                            x2: f64::from(x2),
                            y2: f64::from(y2),
                            x: f64::from(new_pos.x),
                            y: f64::from(new_pos.y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }
                }
                PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                    let start = p;
                    let middle1 = Pos2::new(2.0 * p.x - last_control.x, 2.0 * p.y - last_control.y);
                    let (x2, y2) = (*x2 as f32, *y2 as f32);
                    let middle2 = if *abs {
                        Pos2::new(x2, y2)
                    } else {
                        Pos2::new(p.x + x2, p.y + y2)
                    };
                    last_control = middle2;
                    let (x, y) = (*x as f32, *y as f32);
                    let end = if *abs {
                        Pos2::new(x, y)
                    } else {
                        Pos2::new(p.x + x, p.y + y)
                    };
                    p = end;
                    ui.painter().add(Shape::CubicBezier(CubicBezierShape {
                        points: [start, middle1, middle2, end],
                        closed: false,
                        fill: Color32::TRANSPARENT,
                        stroke: PathStroke::new(1.0, Color32::BLACK),
                    }));

                    let rect_middle = Rect::from_center_size(middle2, Vec2::splat(size_point));
                    let zone = rect_middle.expand(detection_zone);
                    if is_debug {
                        ui.painter().debug_rect(zone, Color32::YELLOW, "point");
                    }
                    ui.painter().circle(
                        middle2,
                        size_point / 2.0,
                        Color32::WHITE,
                        Stroke::default(),
                    );
                    if !new_bb.contains(middle2) {
                        new_bb = new_bb.union(Rect::from_pos(middle2));
                    }

                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = middle2 + resp.drag_delta();
                        *one_point = PathSegment::SmoothCurveTo {
                            abs: true,
                            x2: f64::from(new_pos.x),
                            y2: f64::from(new_pos.y),
                            x: f64::from(x),
                            y: f64::from(y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }

                    let rect_end = Rect::from_center_size(end, Vec2::splat(size_point));
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
                    ui.painter()
                        .line_segment([middle2, end], Stroke::new(1.0, Color32::RED));
                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                        && zone.contains(pos)
                    {
                        let new_pos = end + resp.drag_delta();
                        *one_point = PathSegment::SmoothCurveTo {
                            abs: true,
                            x2: f64::from(x2),
                            y2: f64::from(y2),
                            x: f64::from(new_pos.x),
                            y: f64::from(new_pos.y),
                        };
                        moved = true;
                        if !new_bb.contains(new_pos) {
                            new_bb = new_bb.union(Rect::from_pos(new_pos));
                        }
                    }
                }
                PathSegment::ClosePath { abs: _ } => {
                    ui.painter()
                        .line_segment([last_m, p], Stroke::new(1.0, Color32::BLACK));
                }
                PathSegment::EllipticalArc {
                    abs,
                    rx,
                    ry,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    x,
                    y,
                } => {
                    use kurbo;
                    let (x, y) = if *abs {
                        (*x, *y)
                    } else {
                        (f64::from(p.x) + *x, f64::from(p.y) + *y)
                    };
                    // ARC - convert to bezier curves with
                    // https://docs.rs/svgtypes/latest/src/svgtypes/path.rs.html#975
                    let svg_arc = kurbo::SvgArc {
                        from: kurbo::Point::new(f64::from(p.x), f64::from(p.y)),
                        to: kurbo::Point::new(x, y),
                        radii: kurbo::Vec2::new(*rx, *ry),
                        x_rotation: x_axis_rotation.to_radians(),
                        large_arc: *large_arc,
                        sweep: *sweep,
                    };

                    match kurbo::Arc::from_svg_arc(&svg_arc) {
                        Some(arc) => {
                            arc.to_cubic_beziers(0.1, |p1, p2, end| {
                                let start = Pos2::new(p.x, p.y);
                                let middle1 = Pos2::new(p1.x as f32, p1.y as f32);
                                let middle2 = Pos2::new(p2.x as f32, p2.y as f32);
                                let end = Pos2::new(end.x as f32, end.y as f32);
                                p = end;
                                ui.painter().add(Shape::CubicBezier(CubicBezierShape {
                                    points: [start, middle1, middle2, end],
                                    closed: false,
                                    fill: Color32::TRANSPARENT,
                                    stroke: PathStroke::new(1.0, Color32::BLACK),
                                }));
                            });
                        }
                        None => {
                            ui.painter().line_segment(
                                [p, Pos2::new(x as f32, y as f32)],
                                Stroke::new(1.0, Color32::BLACK),
                            );
                        }
                    }
                }
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
