//! Path
//! Good reading https://razrfalcon.github.io/notes-on-svg-parsing/path-data.html

use std::fmt;

use svgtypes::{PathParser, PathSegment};

/// Represents a single SVG path segment.
#[derive(Debug, Clone)]
pub struct SvgItem {
    /// Represents a single SVG path segment.
    pub inner: PathSegment,
}

fn round_to(value: f64, decimals: u64) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}

impl SvgItem {
    /// Is PathSegment Abs
    pub fn is_abs(&self) -> bool {
        match self.inner {
            PathSegment::MoveTo { abs, .. } => abs,
            PathSegment::LineTo { abs, .. } => abs,
            PathSegment::HorizontalLineTo { abs, .. } => abs,
            PathSegment::VerticalLineTo { abs, .. } => abs,
            PathSegment::CurveTo { abs, .. } => abs,
            PathSegment::SmoothCurveTo { abs, .. } => abs,
            PathSegment::Quadratic { abs, .. } => abs,
            PathSegment::SmoothQuadratic { abs, .. } => abs,
            PathSegment::EllipticalArc { abs, .. } => abs,
            PathSegment::ClosePath { abs } => abs,
        }
    }

    /// Get the letter of PathSegment
    pub fn get_letter(&self) -> char {
        match self.is_abs() {
            true => match self.inner {
                PathSegment::MoveTo { abs: _, .. } => 'M',
                PathSegment::LineTo { abs: _, .. } => 'L',
                PathSegment::HorizontalLineTo { abs: _, .. } => 'H',
                PathSegment::VerticalLineTo { abs: _, .. } => 'V',
                PathSegment::CurveTo { abs: _, .. } => 'C',
                PathSegment::SmoothCurveTo { abs: _, .. } => 'S',
                PathSegment::Quadratic { abs: _, .. } => 'Q',
                PathSegment::SmoothQuadratic { abs: _, .. } => 'T',
                PathSegment::EllipticalArc { abs: _, .. } => 'A',
                PathSegment::ClosePath { abs: _ } => 'Z',
            },
            false => match self.inner {
                PathSegment::MoveTo { abs: _, .. } => 'm',
                PathSegment::LineTo { abs: _, .. } => 'l',
                PathSegment::HorizontalLineTo { abs: _, .. } => 'h',
                PathSegment::VerticalLineTo { abs: _, .. } => 'v',
                PathSegment::CurveTo { abs: _, .. } => 'c',
                PathSegment::SmoothCurveTo { abs: _, .. } => 's',
                PathSegment::Quadratic { abs: _, .. } => 'q',
                PathSegment::SmoothQuadratic { abs: _, .. } => 't',
                PathSegment::EllipticalArc { abs: _, .. } => 'a',
                PathSegment::ClosePath { abs: _ } => 'z',
            },
        }
    }
    /// Returns the value of the segment as a string.
    pub fn value(&self) -> String {
        self.to_string()
    }

    /// Returns the value of the segment as a string.
    /// # Errors
    /// Returns an error if the segment cannot be parsed.
    pub fn set_value(&mut self, to_add: SvgItem) {
        *self = to_add;
    }
}

impl fmt::Display for SvgItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = match self.inner {
            PathSegment::MoveTo { abs: _, x, y } => {
                format!("{}{} {}", self.get_letter(), x, y)
            }
            PathSegment::LineTo { abs: _, x, y } => {
                format!("{}{} {}", self.get_letter(), x, y)
            }
            PathSegment::HorizontalLineTo { abs: _, x } => {
                format!("{}{}", self.get_letter(), x)
            }
            PathSegment::VerticalLineTo { abs: _, y } => {
                format!("{}{}", self.get_letter(), y)
            }
            PathSegment::CurveTo {
                abs: _,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => format!(
                "{}{} {} {} {} {} {}",
                self.get_letter(),
                x1,
                y1,
                x2,
                y2,
                x,
                y
            ),
            PathSegment::SmoothCurveTo {
                abs: _,
                x2,
                y2,
                x,
                y,
            } => {
                format!("{}{} {} {} {}", self.get_letter(), x2, y2, x, y)
            }
            PathSegment::Quadratic {
                abs: _,
                x1,
                y1,
                x,
                y,
            } => {
                format!("{}{} {} {} {}", self.get_letter(), x1, y1, x, y)
            }
            PathSegment::SmoothQuadratic { abs: _, x, y } => {
                format!("{}{} {}", self.get_letter(), x, y)
            }
            PathSegment::EllipticalArc {
                abs: _,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => format!(
                "{}{} {} {} {} {} {} {}",
                self.get_letter(),
                rx,
                ry,
                x_axis_rotation,
                if large_arc { "1" } else { "0" },
                if sweep { "1" } else { "0" },
                x,
                y
            ),
            PathSegment::ClosePath { abs } => {
                if abs {
                    "Z".to_string()
                } else {
                    "z".to_string()
                }
            }
        };
        write!(f, "{fmt}")?;
        Ok(())
    }
}

/// Represents an SVG path, which is a collection of SVG path items.
#[derive(Debug, Clone)]
pub struct SvgPath {
    /// A vector of SVG path items.
    pub items: Vec<SvgItem>,
}

impl SvgPath {
    /// Parses a string into an SVG path.
    /// # Errors
    /// Returns an error if the string cannot be parsed into a valid SVG path.
    pub fn parse(path_str: &str) -> Result<Self, String> {
        let s = PathParser::from(path_str);
        let mut items = Vec::new();

        for segment in s {
            let one_segment = segment
                .map_err(|e| format!("Failed to parse SVG path segment: {e} in {path_str}"))?;
            let to_add = SvgItem { inner: one_segment };
            items.push(to_add);
        }

        Ok(SvgPath { items })
    }

    /// Converts the path to absolute coordinates.
    pub fn absolute(&mut self) {
        let mut pos = (0.0, 0.0);
        let mut subpath_start = (0.0, 0.0);

        for item in &mut self.items {
            match &mut item.inner {
                PathSegment::MoveTo { abs, x, y } => {
                    if !*abs {
                        *x += pos.0;
                        *y += pos.1;
                        *abs = true;
                    }
                    pos.0 = *x;
                    pos.1 = *y;
                    subpath_start = pos;
                }
                PathSegment::LineTo { abs, x, y } => {
                    if !*abs {
                        *x += pos.0;
                        *y += pos.1;
                        *abs = true;
                    }
                    pos.0 = *x;
                    pos.1 = *y;
                }
                PathSegment::HorizontalLineTo { abs, x } => {
                    if !*abs {
                        *x += pos.0;
                        *abs = true;
                    }
                    pos.0 = *x;
                }
                PathSegment::VerticalLineTo { abs, y } => {
                    if !*abs {
                        *y += pos.1;
                        *abs = true;
                    }
                    pos.1 = *y;
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
                    if !*abs {
                        *x1 += pos.0;
                        *y1 += pos.1;
                        *x2 += pos.0;
                        *y2 += pos.1;
                        *x += pos.0;
                        *y += pos.1;
                        *abs = true;
                    }
                    pos.0 = *x;
                    pos.1 = *y;
                }
                PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                    if !*abs {
                        *x2 += pos.0;
                        *y2 += pos.1;
                        *x += pos.0;
                        *y += pos.1;
                        *abs = true;
                    }
                    pos.0 = *x;
                    pos.1 = *y;
                }
                PathSegment::Quadratic { abs, x1, y1, x, y } => {
                    if !*abs {
                        *x1 += pos.0;
                        *y1 += pos.1;
                        *x += pos.0;
                        *y += pos.1;
                        *abs = true;
                    }
                    pos.0 = *x;
                    pos.1 = *y;
                }
                PathSegment::SmoothQuadratic { abs, x, y } => {
                    if !*abs {
                        *x += pos.0;
                        *y += pos.1;
                        *abs = true;
                    }
                    pos.0 = *x;
                    pos.1 = *y;
                }
                PathSegment::EllipticalArc {
                    abs,
                    rx: _,
                    ry: _,
                    x_axis_rotation: _,
                    large_arc: _,
                    sweep: _,
                    x,
                    y,
                } => {
                    if !*abs {
                        *x += pos.0;
                        *y += pos.1;
                        *abs = true;
                    }
                    pos.0 = *x;
                    pos.1 = *y;
                }
                PathSegment::ClosePath { abs } => {
                    *abs = true;
                    pos = subpath_start;
                }
            }
        }
    }

    /// Converts the path to relative coordinates.
    pub fn relative(&mut self) {
        let mut pos = (0.0, 0.0);
        let mut subpath_start = (0.0, 0.0);

        for item in &mut self.items {
            match &mut item.inner {
                PathSegment::MoveTo { abs, x, y } => {
                    if *abs {
                        let rel = (*x - pos.0, *y - pos.1);
                        *x = rel.0;
                        *y = rel.1;
                        *abs = false;
                    }
                    pos.0 += *x;
                    pos.1 += *y;
                    subpath_start = pos;
                }
                PathSegment::LineTo { abs, x, y } => {
                    if *abs {
                        let rel = (*x - pos.0, *y - pos.1);
                        *x = rel.0;
                        *y = rel.1;
                        *abs = false;
                    }
                    pos.0 += *x;
                    pos.1 += *y;
                }
                PathSegment::HorizontalLineTo { abs, x } => {
                    if *abs {
                        let rel = *x - pos.0;
                        *x = rel;
                        *abs = false;
                    }
                    pos.0 += *x;
                }
                PathSegment::VerticalLineTo { abs, y } => {
                    if *abs {
                        let rel = *y - pos.1;
                        *y = rel;
                        *abs = false;
                    }
                    pos.1 += *y;
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
                    if *abs {
                        *x1 -= pos.0;
                        *y1 -= pos.1;
                        *x2 -= pos.0;
                        *y2 -= pos.1;
                        *x -= pos.0;
                        *y -= pos.1;
                        *abs = false;
                    }
                    pos.0 += *x;
                    pos.1 += *y;
                }
                PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                    if *abs {
                        *x2 -= pos.0;
                        *y2 -= pos.1;
                        *x -= pos.0;
                        *y -= pos.1;
                        *abs = false;
                    }
                    pos.0 += *x;
                    pos.1 += *y;
                }
                PathSegment::Quadratic { abs, x1, y1, x, y } => {
                    if *abs {
                        *x1 -= pos.0;
                        *y1 -= pos.1;
                        *x -= pos.0;
                        *y -= pos.1;
                        *abs = false;
                    }
                    pos.0 += *x;
                    pos.1 += *y;
                }
                PathSegment::SmoothQuadratic { abs, x, y } => {
                    if *abs {
                        *x -= pos.0;
                        *y -= pos.1;
                        *abs = false;
                    }
                    pos.0 += *x;
                    pos.1 += *y;
                }
                PathSegment::EllipticalArc {
                    abs,
                    rx: _,
                    ry: _,
                    x_axis_rotation: _,
                    large_arc: _,
                    sweep: _,
                    x,
                    y,
                } => {
                    if *abs {
                        *x -= pos.0;
                        *y -= pos.1;
                        *abs = false;
                    }
                    pos.0 += *x;
                    pos.1 += *y;
                }
                PathSegment::ClosePath { abs } => {
                    *abs = false;
                    pos = subpath_start;
                }
            }
        }
    }

    /// Translates the path by (dx, dy).
    pub fn translate(&mut self, dx: f64, dy: f64) {
        for item in &mut self.items {
            match &mut item.inner {
                PathSegment::MoveTo { x, y, .. }
                | PathSegment::LineTo { x, y, .. }
                | PathSegment::SmoothQuadratic { x, y, .. } => {
                    *x += dx;
                    *y += dy;
                }
                PathSegment::HorizontalLineTo { x, .. } => {
                    *x += dx;
                }
                PathSegment::VerticalLineTo { y, .. } => {
                    *y += dy;
                }
                PathSegment::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                    ..
                } => {
                    *x1 += dx;
                    *y1 += dy;
                    *x2 += dx;
                    *y2 += dy;
                    *x += dx;
                    *y += dy;
                }
                PathSegment::SmoothCurveTo { x2, y2, x, y, .. } => {
                    *x2 += dx;
                    *y2 += dy;
                    *x += dx;
                    *y += dy;
                }
                PathSegment::Quadratic { x1, y1, x, y, .. } => {
                    *x1 += dx;
                    *y1 += dy;
                    *x += dx;
                    *y += dy;
                }
                PathSegment::EllipticalArc { x, y, .. } => {
                    *x += dx;
                    *y += dy;
                }
                PathSegment::ClosePath { .. } => {} // No coordinates to modify
            }
        }
    }

    /// Scales the path by (sx, sy) relative to the origin (0, 0).
    pub fn scale(&mut self, sx: f64, sy: f64) {
        for item in &mut self.items {
            match &mut item.inner {
                PathSegment::MoveTo { x, y, .. }
                | PathSegment::LineTo { x, y, .. }
                | PathSegment::SmoothQuadratic { x, y, .. } => {
                    *x *= sx;
                    *y *= sy;
                }
                PathSegment::HorizontalLineTo { x, .. } => {
                    *x *= sx;
                }
                PathSegment::VerticalLineTo { y, .. } => {
                    *y *= sy;
                }
                PathSegment::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                    ..
                } => {
                    *x1 *= sx;
                    *y1 *= sy;
                    *x2 *= sx;
                    *y2 *= sy;
                    *x *= sx;
                    *y *= sy;
                }
                PathSegment::SmoothCurveTo { x2, y2, x, y, .. } => {
                    *x2 *= sx;
                    *y2 *= sy;
                    *x *= sx;
                    *y *= sy;
                }
                PathSegment::Quadratic { x1, y1, x, y, .. } => {
                    *x1 *= sx;
                    *y1 *= sy;
                    *x *= sx;
                    *y *= sy;
                }
                PathSegment::EllipticalArc { rx, ry, x, y, .. } => {
                    *rx *= sx.abs();
                    *ry *= sy.abs();
                    *x *= sx;
                    *y *= sy;
                }
                PathSegment::ClosePath { .. } => {}
            }
        }
    }

    /// Rotates the path around (cx, cy) by angle_deg degrees.
    pub fn rotate(&mut self, angle_deg: f64, cx: f64, cy: f64) {
        let angle_rad = angle_deg.to_radians();
        let (sin, cos) = angle_rad.sin_cos();

        let rotate_point = |x: &mut f64, y: &mut f64| {
            let dx = *x - cx;
            let dy = *y - cy;
            let x_new = dx * cos - dy * sin + cx;
            let y_new = dx * sin + dy * cos + cy;
            *x = x_new;
            *y = y_new;
        };

        for item in &mut self.items {
            match &mut item.inner {
                PathSegment::MoveTo { x, y, .. }
                | PathSegment::LineTo { x, y, .. }
                | PathSegment::SmoothQuadratic { x, y, .. } => {
                    rotate_point(x, y);
                }
                PathSegment::HorizontalLineTo { .. } | PathSegment::VerticalLineTo { .. } => {
                    // You could convert these to LineTo or skip them
                }
                PathSegment::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                    ..
                } => {
                    rotate_point(x1, y1);
                    rotate_point(x2, y2);
                    rotate_point(x, y);
                }
                PathSegment::SmoothCurveTo { x2, y2, x, y, .. } => {
                    rotate_point(x2, y2);
                    rotate_point(x, y);
                }
                PathSegment::Quadratic { x1, y1, x, y, .. } => {
                    rotate_point(x1, y1);
                    rotate_point(x, y);
                }
                PathSegment::EllipticalArc {
                    x,
                    y,
                    x_axis_rotation,
                    ..
                } => {
                    rotate_point(x, y);
                    *x_axis_rotation += angle_deg;
                }
                PathSegment::ClosePath { .. } => {}
            }
        }
    }

    /// Rounds all coordinates and values in the path to the given number of decimal places.
    pub fn round(&mut self, decimal: u64) {
        for item in &mut self.items {
            match &mut item.inner {
                PathSegment::MoveTo { x, y, .. }
                | PathSegment::LineTo { x, y, .. }
                | PathSegment::SmoothQuadratic { x, y, .. } => {
                    *x = round_to(*x, decimal);
                    *y = round_to(*y, decimal);
                }
                PathSegment::HorizontalLineTo { x, .. } => {
                    *x = round_to(*x, decimal);
                }
                PathSegment::VerticalLineTo { y, .. } => {
                    *y = round_to(*y, decimal);
                }
                PathSegment::CurveTo {
                    x1,
                    y1,
                    x2,
                    y2,
                    x,
                    y,
                    ..
                } => {
                    *x1 = round_to(*x1, decimal);
                    *y1 = round_to(*y1, decimal);
                    *x2 = round_to(*x2, decimal);
                    *y2 = round_to(*y2, decimal);
                    *x = round_to(*x, decimal);
                    *y = round_to(*y, decimal);
                }
                PathSegment::SmoothCurveTo { x2, y2, x, y, .. } => {
                    *x2 = round_to(*x2, decimal);
                    *y2 = round_to(*y2, decimal);
                    *x = round_to(*x, decimal);
                    *y = round_to(*y, decimal);
                }
                PathSegment::Quadratic { x1, y1, x, y, .. } => {
                    *x1 = round_to(*x1, decimal);
                    *y1 = round_to(*y1, decimal);
                    *x = round_to(*x, decimal);
                    *y = round_to(*y, decimal);
                }
                PathSegment::EllipticalArc {
                    rx,
                    ry,
                    x_axis_rotation,
                    x,
                    y,
                    ..
                } => {
                    *rx = round_to(*rx, decimal);
                    *ry = round_to(*ry, decimal);
                    *x_axis_rotation = round_to(*x_axis_rotation, decimal);
                    *x = round_to(*x, decimal);
                    *y = round_to(*y, decimal);
                }
                PathSegment::ClosePath { .. } => {}
            }
        }
    }

    /// Toggles the coordinate type (absolute/relative) of the path segment at the given index.
    pub fn toggle_coord_type_at(&mut self, i: usize) {
        let mut pos = (0.0, 0.0);
        let mut subpath_start = (0.0, 0.0);

        // We need to recalculate the position up to item i
        for (j, item) in self.items.iter_mut().enumerate() {
            if j == i {
                match &mut item.inner {
                    PathSegment::MoveTo { abs, x, y } => {
                        if *abs {
                            *x -= pos.0;
                            *y -= pos.1;
                        } else {
                            *x += pos.0;
                            *y += pos.1;
                        }
                        *abs = !*abs;
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                        pos.1 = if *abs { *y } else { pos.1 + *y };
                        // subpath_start = pos;
                    }
                    PathSegment::LineTo { abs, x, y } => {
                        if *abs {
                            *x -= pos.0;
                            *y -= pos.1;
                        } else {
                            *x += pos.0;
                            *y += pos.1;
                        }
                        *abs = !*abs;
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                        pos.1 = if *abs { *y } else { pos.1 + *y };
                    }
                    PathSegment::HorizontalLineTo { abs, x } => {
                        if *abs {
                            *x -= pos.0;
                        } else {
                            *x += pos.0;
                        }
                        *abs = !*abs;
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                    }
                    PathSegment::VerticalLineTo { abs, y } => {
                        if *abs {
                            *y -= pos.1;
                        } else {
                            *y += pos.1;
                        }
                        *abs = !*abs;
                        pos.1 = if *abs { *y } else { pos.1 + *y };
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
                        if *abs {
                            *x1 -= pos.0;
                            *y1 -= pos.1;
                            *x2 -= pos.0;
                            *y2 -= pos.1;
                            *x -= pos.0;
                            *y -= pos.1;
                        } else {
                            *x1 += pos.0;
                            *y1 += pos.1;
                            *x2 += pos.0;
                            *y2 += pos.1;
                            *x += pos.0;
                            *y += pos.1;
                        }
                        *abs = !*abs;
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                        pos.1 = if *abs { *y } else { pos.1 + *y };
                    }
                    PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                        if *abs {
                            *x2 -= pos.0;
                            *y2 -= pos.1;
                            *x -= pos.0;
                            *y -= pos.1;
                        } else {
                            *x2 += pos.0;
                            *y2 += pos.1;
                            *x += pos.0;
                            *y += pos.1;
                        }
                        *abs = !*abs;
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                        pos.1 = if *abs { *y } else { pos.1 + *y };
                    }
                    PathSegment::Quadratic { abs, x1, y1, x, y } => {
                        if *abs {
                            *x1 -= pos.0;
                            *y1 -= pos.1;
                            *x -= pos.0;
                            *y -= pos.1;
                        } else {
                            *x1 += pos.0;
                            *y1 += pos.1;
                            *x += pos.0;
                            *y += pos.1;
                        }
                        *abs = !*abs;
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                        pos.1 = if *abs { *y } else { pos.1 + *y };
                    }
                    PathSegment::SmoothQuadratic { abs, x, y } => {
                        if *abs {
                            *x -= pos.0;
                            *y -= pos.1;
                        } else {
                            *x += pos.0;
                            *y += pos.1;
                        }
                        *abs = !*abs;
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                        pos.1 = if *abs { *y } else { pos.1 + *y };
                    }
                    PathSegment::EllipticalArc { abs, x, y, .. } => {
                        if *abs {
                            *x -= pos.0;
                            *y -= pos.1;
                        } else {
                            *x += pos.0;
                            *y += pos.1;
                        }
                        *abs = !*abs;
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                        pos.1 = if *abs { *y } else { pos.1 + *y };
                    }
                    PathSegment::ClosePath { abs } => {
                        *abs = !*abs;
                        // pos = subpath_start;
                    }
                }
                break;
            } else {
                // Update the position so we know the current point at index i
                match &item.inner {
                    PathSegment::MoveTo { abs, x, y } => {
                        if *abs {
                            pos = (*x, *y);
                        } else {
                            pos.0 += *x;
                            pos.1 += *y;
                        }
                        subpath_start = pos;
                    }
                    PathSegment::LineTo { abs, x, y }
                    | PathSegment::SmoothQuadratic { abs, x, y }
                    | PathSegment::Quadratic {
                        abs,
                        x1: _,
                        y1: _,
                        x,
                        y,
                    }
                    | PathSegment::SmoothCurveTo {
                        abs,
                        x2: _,
                        y2: _,
                        x,
                        y,
                    }
                    | PathSegment::CurveTo {
                        abs,
                        x1: _,
                        y1: _,
                        x2: _,
                        y2: _,
                        x,
                        y,
                    }
                    | PathSegment::EllipticalArc { abs, x, y, .. } => {
                        if *abs {
                            pos = (*x, *y);
                        } else {
                            pos.0 += *x;
                            pos.1 += *y;
                        }
                    }
                    PathSegment::HorizontalLineTo { abs, x } => {
                        pos.0 = if *abs { *x } else { pos.0 + *x };
                    }
                    PathSegment::VerticalLineTo { abs, y } => {
                        pos.1 = if *abs { *y } else { pos.1 + *y };
                    }
                    PathSegment::ClosePath { .. } => {
                        pos = subpath_start;
                    }
                }
            }
        }
    }

    /// Try replace element
    /// # Errors
    /// Error if it's invalid
    pub fn try_replace_element_at(&mut self, idx: usize, val: &str) -> Result<String, String> {
        if idx < self.items.len() {
            let mut result = String::new();
            for (index, one_item) in self.items.iter().enumerate() {
                let segment_str = if index == idx {
                    val
                } else {
                    &one_item.to_string()
                };
                result.push_str(segment_str);
            }
            match SvgPath::parse(&result) {
                Ok(path) => {
                    return Ok(path.to_string());
                }

                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err("Invalid index".to_string())
    }
}

impl fmt::Display for SvgPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for item in &self.items {
            let segment_str = item.to_string();
            if !segment_str.is_empty() {
                result.push_str(&segment_str);
            }
        }
        write!(f, "{result}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_parsed() {
        let mut parsed = SvgPath::parse("M174 270C113 271 84 237 80 207 78 189 79 172 76 172 46 154 34 142 21 109-4 49 45 62 55 66 71 72 79 83 90 94 103 104 160 104 174 104ZM110 150C104 153 98 160 95 166S92 184 95 190C103 208 126 216 142 205 149 200 153 195 155 188 161 172 150 153 133 147 128 145 115 147 110 150ZM174 270C236 271 265 237 269 207 271 189 270 172 273 172 303 154 315 142 328 109 353 49 304 62 294 66 278 72 270 83 259 94 246 104 189 104 174 104ZM239 150C245 153 251 160 254 166S257 184 254 190C246 208 223 216 207 205 200 200 196 195 194 188 188 172 199 153 216 147 221 145 234 147 239 150Z").unwrap();
        assert_eq!(
            parsed.to_string(),
            "M174 270C113 271 84 237 80 207C78 189 79 172 76 172C46 154 34 142 21 109C-4 49 45 62 55 66C71 72 79 83 90 94C103 104 160 104 174 104ZM110 150C104 153 98 160 95 166S92 184 95 190C103 208 126 216 142 205C149 200 153 195 155 188C161 172 150 153 133 147C128 145 115 147 110 150ZM174 270C236 271 265 237 269 207C271 189 270 172 273 172C303 154 315 142 328 109C353 49 304 62 294 66C278 72 270 83 259 94C246 104 189 104 174 104ZM239 150C245 153 251 160 254 166S257 184 254 190C246 208 223 216 207 205C200 200 196 195 194 188C188 172 199 153 216 147C221 145 234 147 239 150Z"
        );
        parsed.absolute();
        assert_eq!(
            parsed.to_string(),
            "M174 270C113 271 84 237 80 207C78 189 79 172 76 172C46 154 34 142 21 109C-4 49 45 62 55 66C71 72 79 83 90 94C103 104 160 104 174 104ZM110 150C104 153 98 160 95 166S92 184 95 190C103 208 126 216 142 205C149 200 153 195 155 188C161 172 150 153 133 147C128 145 115 147 110 150ZM174 270C236 271 265 237 269 207C271 189 270 172 273 172C303 154 315 142 328 109C353 49 304 62 294 66C278 72 270 83 259 94C246 104 189 104 174 104ZM239 150C245 153 251 160 254 166S257 184 254 190C246 208 223 216 207 205C200 200 196 195 194 188C188 172 199 153 216 147C221 145 234 147 239 150Z"
        );

        parsed.relative();
        assert_eq!(
            parsed.to_string(),
            "m174 270c-61 1 -90 -33 -94 -63c-2 -18 -1 -35 -4 -35c-30 -18 -42 -30 -55 -63c-25 -60 24 -47 34 -43c16 6 24 17 35 28c13 10 70 10 84 10zm-64 -120c-6 3 -12 10 -15 16s-3 18 0 24c8 18 31 26 47 15c7 -5 11 -10 13 -17c6 -16 -5 -35 -22 -41c-5 -2 -18 0 -23 3zm64 120c62 1 91 -33 95 -63c2 -18 1 -35 4 -35c30 -18 42 -30 55 -63c25 -60 -24 -47 -34 -43c-16 6 -24 17 -35 28c-13 10 -70 10 -85 10zm65 -120c6 3 12 10 15 16s3 18 0 24c-8 18 -31 26 -47 15c-7 -5 -11 -10 -13 -17c-6 -16 5 -35 22 -41c5 -2 18 0 23 3z"
        );

        let mut parsed_relative = SvgPath::parse("m174 270c-61 1 -90 -33 -94 -63c-2 -18 -1 -35 -4 -35c-30 -18 -42 -30 -55 -63c-25 -60 24 -47 34 -43c16 6 24 17 35 28c13 10 70 10 84 10zm-64 -120c-6 3 -12 10 -15 16s-3 18 0 24c8 18 31 26 47 15c7 -5 11 -10 13 -17c6 -16 -5 -35 -22 -41c-5 -2 -18 0 -23 3zm64 120c62 1 91 -33 95 -63c2 -18 1 -35 4 -35c30 -18 42 -30 55 -63c25 -60 -24 -47 -34 -43c-16 6 -24 17 -35 28c-13 10 -70 10 -85 10zm65 -120c6 3 12 10 15 16s3 18 0 24c-8 18 -31 26 -47 15c-7 -5 -11 -10 -13 -17c-6 -16 5 -35 22 -41c5 -2 18 0 23 3z").unwrap();

        assert_eq!(
            parsed_relative.to_string(),
            "m174 270c-61 1 -90 -33 -94 -63c-2 -18 -1 -35 -4 -35c-30 -18 -42 -30 -55 -63c-25 -60 24 -47 34 -43c16 6 24 17 35 28c13 10 70 10 84 10zm-64 -120c-6 3 -12 10 -15 16s-3 18 0 24c8 18 31 26 47 15c7 -5 11 -10 13 -17c6 -16 -5 -35 -22 -41c-5 -2 -18 0 -23 3zm64 120c62 1 91 -33 95 -63c2 -18 1 -35 4 -35c30 -18 42 -30 55 -63c25 -60 -24 -47 -34 -43c-16 6 -24 17 -35 28c-13 10 -70 10 -85 10zm65 -120c6 3 12 10 15 16s3 18 0 24c-8 18 -31 26 -47 15c-7 -5 -11 -10 -13 -17c-6 -16 5 -35 22 -41c5 -2 18 0 23 3z"
        );
        parsed_relative.absolute();
        assert_eq!(
            parsed_relative.to_string(),
            "M174 270C113 271 84 237 80 207C78 189 79 172 76 172C46 154 34 142 21 109C-4 49 45 62 55 66C71 72 79 83 90 94C103 104 160 104 174 104ZM110 150C104 153 98 160 95 166S92 184 95 190C103 208 126 216 142 205C149 200 153 195 155 188C161 172 150 153 133 147C128 145 115 147 110 150ZM174 270C236 271 265 237 269 207C271 189 270 172 273 172C303 154 315 142 328 109C353 49 304 62 294 66C278 72 270 83 259 94C246 104 189 104 174 104ZM239 150C245 153 251 160 254 166S257 184 254 190C246 208 223 216 207 205C200 200 196 195 194 188C188 172 199 153 216 147C221 145 234 147 239 150Z"
        );
    }
}
