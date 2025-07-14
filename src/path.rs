//! Path

use std::fmt;

use svgtypes::{Error, PathParser, PathSegment};

/// A parsed path data.
#[derive(Debug)]
pub struct PathParsed {
    /// Segments of the path
    segments: Vec<PathSegment>,
}

impl PathParsed {
    /// Returns the path segments vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{PathParsed, PathSegment};
    ///
    /// let parsed = PathParsed::from("M10-20l30.1.5.1-20z").unwrap();
    /// let segments = parsed.segments();
    ///
    /// assert_eq!(segments, &[
    ///     PathSegment::MoveTo { abs: true, x: 10.0, y: -20.0 },
    ///     PathSegment::LineTo { abs: false, x: 30.1, y: 0.5 },
    ///     PathSegment::LineTo { abs: false, x: 0.1, y: -20.0 },
    ///     PathSegment::ClosePath { abs: false },
    /// ]);
    /// ```
    ///
    pub fn from(data: &str) -> Result<Self, Error> {
        let mut s = PathParser::from(data);
        let mut segments = Vec::new();
        for segment in &mut s {
            segments.push(segment?);
        }
        Ok(Self { segments })
    }

    /// Returns the path segments vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{PathParsed, PathSegment};
    ///
    /// let parsed = PathParsed::from("M10-20l30.1.5.1-20z").unwrap();
    /// let segments = parsed.segments();
    ///
    /// assert_eq!(segments, &[
    ///     PathSegment::MoveTo { abs: true, x: 10.0, y: -20.0 },
    ///     PathSegment::LineTo { abs: false, x: 30.1, y: 0.5 },
    ///     PathSegment::LineTo { abs: false, x: 0.1, y: -20.0 },
    ///     PathSegment::ClosePath { abs: false },
    /// ]);
    /// ```
    ///
    pub fn segments(&self) -> &[PathSegment] {
        &self.segments
    }

    /// Returns the path data as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{PathParsed};
    ///
    /// let parsed = PathParsed::from("M10-20l30.1.5.1-20z").unwrap();
    /// let segment_str = parsed.path_data();
    ///
    /// assert_eq!(segment_str, "M10 -20l30.1 0.5l0.1 -20z");
    /// ```
    ///
    pub fn path_data(&self) -> String {
        self.path_data_with_separator("")
    }

    /// Returns the path data as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{PathParsed};
    ///
    /// let parsed = PathParsed::from("M10-20l30.1.5.1-20z").unwrap();
    /// let segment_str = parsed.path_data_with_separator(" ");
    ///
    /// assert_eq!(segment_str, "M10 -20 l30.1 0.5 l0.1 -20 z");
    /// ```
    ///
    pub fn path_data_with_separator(&self, separator: &str) -> String {
        let strings = self.segments.iter().map(to_string);
        strings.collect::<Vec<String>>().join(separator)
    }

    /// Converts all segments to relative.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{PathParsed, PathSegment};
    ///
    /// let mut parsed = PathParsed::from("M10-20l30.1.5.1-20z").unwrap();
    /// let segment_str = parsed.to_relative().to_string();
    ///
    /// assert_eq!(segment_str, "m10 -20l30.1 0.5l0.1 -20z");
    /// ```
    ///
    pub fn to_relative(&self) -> Self {
        let mut segments = Vec::new();
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;

        for segment in &self.segments {
            if is_abs(segment) {
                // need to be converted to relative
                match *segment {
                    PathSegment::MoveTo { abs: _, x, y } => {
                        segments.push(PathSegment::MoveTo {
                            abs: false,
                            x: x - prev_x,
                            y: y - prev_y,
                        });
                        prev_x = x;
                        prev_y = y;
                    }
                    PathSegment::LineTo { abs: _, x, y } => {
                        segments.push(PathSegment::LineTo {
                            abs: false,
                            x: x - prev_x,
                            y: y - prev_y,
                        });
                        prev_x = x;
                        prev_y = y;
                    }
                    PathSegment::HorizontalLineTo { abs: _, x } => {
                        segments.push(PathSegment::HorizontalLineTo {
                            abs: false,
                            x: x - prev_x,
                        });
                        prev_x = x;
                    }
                    PathSegment::VerticalLineTo { abs: _, y } => {
                        segments.push(PathSegment::VerticalLineTo {
                            abs: false,
                            y: y - prev_y,
                        });
                        prev_y = y;
                    }
                    PathSegment::CurveTo {
                        abs: _,
                        x1,
                        y1,
                        x2,
                        y2,
                        x,
                        y,
                    } => {
                        segments.push(PathSegment::CurveTo {
                            abs: false,
                            x1: x1 - prev_x,
                            y1: y1 - prev_y,
                            x2: x2 - prev_x,
                            y2: y2 - prev_y,
                            x: x - prev_x,
                            y: y - prev_y,
                        });
                        prev_x = x;
                        prev_y = y;
                    }
                    PathSegment::SmoothCurveTo {
                        abs: _,
                        x2,
                        y2,
                        x,
                        y,
                    } => {
                        segments.push(PathSegment::SmoothCurveTo {
                            abs: false,
                            x2: x2 - prev_x,
                            y2: y2 - prev_y,
                            x: x - prev_x,
                            y: y - prev_y,
                        });
                        prev_x = x;
                        prev_y = y;
                    }
                    PathSegment::Quadratic {
                        abs: _,
                        x1,
                        y1,
                        x,
                        y,
                    } => {
                        segments.push(PathSegment::Quadratic {
                            abs: false,
                            x1: x1 - prev_x,
                            y1: y1 - prev_y,
                            x: x - prev_x,
                            y: y - prev_y,
                        });
                        prev_x = x;
                        prev_y = y;
                    }
                    PathSegment::SmoothQuadratic { abs: _, x, y } => {
                        segments.push(PathSegment::SmoothQuadratic {
                            abs: false,
                            x: x - prev_x,
                            y: y - prev_y,
                        });
                        prev_x = x;
                        prev_y = y;
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
                    } => {
                        segments.push(PathSegment::EllipticalArc {
                            abs: false,
                            rx,
                            ry,
                            x_axis_rotation,
                            large_arc,
                            sweep,
                            x: x - prev_x,
                            y: y - prev_y,
                        });
                        prev_x = x;
                        prev_y = y;
                    }
                    PathSegment::ClosePath { .. } => {
                        segments.push(PathSegment::ClosePath { abs: false });
                    }
                }
            } else {
                segments.push(*segment);
            }
        }
        Self { segments }
    }

    /// Converts all segments to absolute.
    ///
    /// # Examples
    ///
    /// ```
    /// use svgtypes::{PathParsed};
    ///
    /// let mut parsed = PathParsed::from("M10-20l30.1.5.1-20z").unwrap();
    /// let segment_str = parsed.to_absolute().to_string();
    ///
    /// assert_eq!(segment_str, "M10 -20L40.1 -19.5L40.2 -39.5Z");
    /// ```
    ///
    pub fn to_absolute(&self) -> Self {
        let mut converted = Vec::new();
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;

        for segment in &self.segments {
            if is_abs(segment) {
                converted.push(*segment);
            } else {
                match *segment {
                    PathSegment::MoveTo { abs: _, x, y } => {
                        converted.push(PathSegment::MoveTo {
                            abs: true,
                            x: x + prev_x,
                            y: y + prev_y,
                        });
                        prev_x += x;
                        prev_y += y;
                    }
                    PathSegment::LineTo { abs: _, x, y } => {
                        converted.push(PathSegment::LineTo {
                            abs: true,
                            x: x + prev_x,
                            y: y + prev_y,
                        });
                        prev_x += x;
                        prev_y += y;
                    }
                    PathSegment::HorizontalLineTo { abs: _, x } => {
                        converted.push(PathSegment::HorizontalLineTo {
                            abs: true,
                            x: x + prev_x,
                        });
                        prev_x += x;
                    }
                    PathSegment::VerticalLineTo { abs: _, y } => {
                        converted.push(PathSegment::VerticalLineTo {
                            abs: true,
                            y: y + prev_y,
                        });
                        prev_y += y;
                    }
                    PathSegment::CurveTo {
                        abs: _,
                        x1,
                        y1,
                        x2,
                        y2,
                        x,
                        y,
                    } => {
                        converted.push(PathSegment::CurveTo {
                            abs: true,
                            x1: x1 + prev_x,
                            y1: y1 + prev_y,
                            x2: x2 + prev_x,
                            y2: y2 + prev_y,
                            x: x + prev_x,
                            y: y + prev_y,
                        });
                        prev_x += x;
                        prev_y += y;
                    }
                    PathSegment::SmoothCurveTo {
                        abs: _,
                        x2,
                        y2,
                        x,
                        y,
                    } => {
                        converted.push(PathSegment::SmoothCurveTo {
                            abs: true,
                            x2: x2 + prev_x,
                            y2: y2 + prev_y,
                            x: x + prev_x,
                            y: y + prev_y,
                        });
                        prev_x += x;
                        prev_y += y;
                    }
                    PathSegment::Quadratic {
                        abs: _,
                        x1,
                        y1,
                        x,
                        y,
                    } => {
                        converted.push(PathSegment::Quadratic {
                            abs: true,
                            x1: x1 + prev_x,
                            y1: y1 + prev_y,
                            x: x + prev_x,
                            y: y + prev_y,
                        });
                        prev_x += x;
                        prev_y += y;
                    }
                    PathSegment::SmoothQuadratic { abs: _, x, y } => {
                        converted.push(PathSegment::SmoothQuadratic {
                            abs: true,
                            x: x + prev_x,
                            y: y + prev_y,
                        });
                        prev_x += x;
                        prev_y += y;
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
                    } => {
                        converted.push(PathSegment::EllipticalArc {
                            abs: true,
                            rx,
                            ry,
                            x_axis_rotation,
                            large_arc,
                            sweep,
                            x: x + prev_x,
                            y: y + prev_y,
                        });
                        prev_x += x;
                        prev_y += y;
                    }
                    PathSegment::ClosePath { .. } => {
                        converted.push(PathSegment::ClosePath { abs: true });
                    }
                }
            }
        }
        Self {
            segments: converted,
        }
    }
}

/// Convert PathSegment to String
pub fn to_string(path_seg: &PathSegment) -> String {
    match path_seg {
        PathSegment::MoveTo { abs, x, y } => format!("{}{} {}", if *abs { "M" } else { "m" }, x, y),
        PathSegment::LineTo { abs, x, y } => format!("{}{} {}", if *abs { "L" } else { "l" }, x, y),
        PathSegment::HorizontalLineTo { abs, x } => {
            format!("{}{}", if *abs { "H" } else { "h" }, x)
        }
        PathSegment::VerticalLineTo { abs, y } => format!("{}{}", if *abs { "V" } else { "v" }, y),
        PathSegment::CurveTo {
            abs,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        } => format!(
            "{}{} {} {} {} {} {}",
            if *abs { "C" } else { "c" },
            x1,
            y1,
            x2,
            y2,
            x,
            y
        ),
        PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
            format!("{}{} {} {} {}", if *abs { "S" } else { "s" }, x2, y2, x, y)
        }
        PathSegment::Quadratic { abs, x1, y1, x, y } => {
            format!("{}{} {} {} {}", if *abs { "Q" } else { "q" }, x1, y1, x, y)
        }
        PathSegment::SmoothQuadratic { abs, x, y } => {
            format!("{}{} {}", if *abs { "T" } else { "t" }, x, y)
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
        } => format!(
            "{}{} {} {} {} {} {} {}",
            if *abs { "A" } else { "a" },
            rx,
            ry,
            x_axis_rotation,
            if *large_arc { "1" } else { "0" },
            if *sweep { "1" } else { "0" },
            x,
            y
        ),
        PathSegment::ClosePath { abs } => {
            if *abs {
                "Z".to_string()
            } else {
                "z".to_string()
            }
        }
    }
}

/// is PathSegment Abs
pub fn is_abs(path_seg: &PathSegment) -> bool {
    match path_seg {
        PathSegment::MoveTo { abs, .. } => *abs,
        PathSegment::LineTo { abs, .. } => *abs,
        PathSegment::HorizontalLineTo { abs, .. } => *abs,
        PathSegment::VerticalLineTo { abs, .. } => *abs,
        PathSegment::CurveTo { abs, .. } => *abs,
        PathSegment::SmoothCurveTo { abs, .. } => *abs,
        PathSegment::Quadratic { abs, .. } => *abs,
        PathSegment::SmoothQuadratic { abs, .. } => *abs,
        PathSegment::EllipticalArc { abs, .. } => *abs,
        PathSegment::ClosePath { abs } => *abs,
    }
}

/// Get the letter of PathSegment
pub fn get_letter(path_seg: &PathSegment) -> String {
    match is_abs(path_seg) {
        true => match path_seg {
            PathSegment::MoveTo { abs: _, .. } => "M".to_string(),
            PathSegment::LineTo { abs: _, .. } => "L".to_string(),
            PathSegment::HorizontalLineTo { abs: _, .. } => "H".to_string(),
            PathSegment::VerticalLineTo { abs: _, .. } => "V".to_string(),
            PathSegment::CurveTo { abs: _, .. } => "C".to_string(),
            PathSegment::SmoothCurveTo { abs: _, .. } => "S".to_string(),
            PathSegment::Quadratic { abs: _, .. } => "Q".to_string(),
            PathSegment::SmoothQuadratic { abs: _, .. } => "T".to_string(),
            PathSegment::EllipticalArc { abs: _, .. } => "A".to_string(),
            PathSegment::ClosePath { abs: _ } => "Z".to_string(),
        },
        false => match path_seg {
            PathSegment::MoveTo { abs: _, .. } => "m".to_string(),
            PathSegment::LineTo { abs: _, .. } => "l".to_string(),
            PathSegment::HorizontalLineTo { abs: _, .. } => "h".to_string(),
            PathSegment::VerticalLineTo { abs: _, .. } => "v".to_string(),
            PathSegment::CurveTo { abs: _, .. } => "c".to_string(),
            PathSegment::SmoothCurveTo { abs: _, .. } => "s".to_string(),
            PathSegment::Quadratic { abs: _, .. } => "q".to_string(),
            PathSegment::SmoothQuadratic { abs: _, .. } => "t".to_string(),
            PathSegment::EllipticalArc { abs: _, .. } => "a".to_string(),
            PathSegment::ClosePath { abs: _ } => "z".to_string(),
        },
    }
}

impl fmt::Display for PathParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in &self.segments {
            write!(f, "{}", to_string(segment))?;
        }
        Ok(())
    }
}
