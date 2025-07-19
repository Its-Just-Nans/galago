//! Path
//! Good reading https://razrfalcon.github.io/notes-on-svg-parsing/path-data.html

use std::fmt;

use svgtypes::{Error, PathParser, PathSegment};

/// A wrapper for a PathSegment
#[derive(Debug, Clone)]
pub struct PathSegmentWrap {
    /// The value of the segment as a string
    pub _value: String,
    /// The path segment
    pub inner: PathSegment,
    /// Previous absolute x coordinate
    pub previous_abs_x: f64,
    /// Previous absolute y coordinate
    pub previous_abs_y: f64,
}

impl PathSegmentWrap {
    /// new PathSegmentWrap
    pub fn new_with_pos(segment: PathSegment, x: f64, y: f64) -> Self {
        Self {
            inner: segment,
            _value: String::new(),
            previous_abs_x: x,
            previous_abs_y: y,
        }
    }

    /// Returns the value of the segment as a string.
    pub fn value(&mut self) -> &mut String {
        self._value = self.to_string();
        &mut self._value
    }

    /// Get the letter of PathSegment
    pub fn get_letter(&self) -> String {
        match self.is_abs() {
            true => match self.inner {
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
            false => match self.inner {
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

    /// Converts the segment to relative or absolute coordinates based on the current state.
    pub fn to_relative(&self) -> Self {
        if !self.is_abs() {
            return self.clone();
        }
        let new_segment = match self.inner {
            PathSegment::MoveTo { abs: _, x, y } => PathSegment::MoveTo {
                abs: false,
                x: x - self.previous_abs_x,
                y: y - self.previous_abs_y,
            },
            PathSegment::LineTo { abs: _, x, y } => PathSegment::LineTo {
                abs: false,
                x: x - self.previous_abs_x,
                y: y - self.previous_abs_y,
            },
            PathSegment::HorizontalLineTo { abs: _, x } => PathSegment::HorizontalLineTo {
                abs: false,
                x: x - self.previous_abs_x,
            },
            PathSegment::VerticalLineTo { abs: _, y } => PathSegment::VerticalLineTo {
                abs: false,
                y: y - self.previous_abs_y,
            },
            PathSegment::CurveTo {
                abs: _,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => PathSegment::CurveTo {
                abs: false,
                x1: x1 - self.previous_abs_x,
                y1: y1 - self.previous_abs_y,
                x2: x2 - self.previous_abs_x,
                y2: y2 - self.previous_abs_y,
                x: x - self.previous_abs_x,
                y: y - self.previous_abs_y,
            },
            PathSegment::SmoothCurveTo {
                abs: _,
                x2,
                y2,
                x,
                y,
            } => PathSegment::SmoothCurveTo {
                abs: false,
                x2: x2 - self.previous_abs_x,
                y2: y2 - self.previous_abs_y,
                x: x - self.previous_abs_x,
                y: y - self.previous_abs_y,
            },
            PathSegment::Quadratic {
                abs: _,
                x1,
                y1,
                x,
                y,
            } => PathSegment::Quadratic {
                abs: false,
                x1: x1 - self.previous_abs_x,
                y1: y1 - self.previous_abs_y,
                x: x - self.previous_abs_x,
                y: y - self.previous_abs_y,
            },
            PathSegment::SmoothQuadratic { abs: _, x, y } => PathSegment::SmoothQuadratic {
                abs: false,
                x: x - self.previous_abs_x,
                y: y - self.previous_abs_y,
            },
            PathSegment::EllipticalArc {
                abs: _,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => PathSegment::EllipticalArc {
                abs: false,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x: x - self.previous_abs_x,
                y: y - self.previous_abs_y,
            },
            PathSegment::ClosePath { abs: _ } => PathSegment::ClosePath { abs: false },
        };
        // Update the previous absolute coordinates
        Self {
            inner: new_segment,
            _value: String::new(),
            previous_abs_x: self.previous_abs_x,
            previous_abs_y: self.previous_abs_y,
        }
    }

    /// Converts the segment to absolute coordinates based on the previous absolute coordinates.
    pub fn to_absolute(&self) -> Self {
        if self.is_abs() {
            return self.clone();
        }
        let new_segment = match self.inner {
            PathSegment::MoveTo { abs: _, x, y } => PathSegment::MoveTo {
                abs: true,
                x: x + self.previous_abs_x,
                y: y + self.previous_abs_y,
            },
            PathSegment::LineTo { abs: _, x, y } => PathSegment::LineTo {
                abs: true,
                x: x + self.previous_abs_x,
                y: y + self.previous_abs_y,
            },
            PathSegment::HorizontalLineTo { abs: _, x } => PathSegment::HorizontalLineTo {
                abs: true,
                x: x + self.previous_abs_x,
            },
            PathSegment::VerticalLineTo { abs: _, y } => PathSegment::VerticalLineTo {
                abs: true,
                y: y + self.previous_abs_y,
            },
            PathSegment::CurveTo {
                abs: _,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => PathSegment::CurveTo {
                abs: true,
                x1: x1 + self.previous_abs_x,
                y1: y1 + self.previous_abs_y,
                x2: x2 + self.previous_abs_x,
                y2: y2 + self.previous_abs_y,
                x: x + self.previous_abs_x,
                y: y + self.previous_abs_y,
            },
            PathSegment::SmoothCurveTo {
                abs: _,
                x2,
                y2,
                x,
                y,
            } => PathSegment::SmoothCurveTo {
                abs: true,
                x2: x2 + self.previous_abs_x,
                y2: y2 + self.previous_abs_y,
                x: x + self.previous_abs_x,
                y: y + self.previous_abs_y,
            },
            PathSegment::Quadratic {
                abs: _,
                x1,
                y1,
                x,
                y,
            } => PathSegment::Quadratic {
                abs: true,
                x1: x1 + self.previous_abs_x,
                y1: y1 + self.previous_abs_y,
                x: x + self.previous_abs_x,
                y: y + self.previous_abs_y,
            },
            PathSegment::SmoothQuadratic { abs: _, x, y } => PathSegment::SmoothQuadratic {
                abs: true,
                x: x + self.previous_abs_x,
                y: y + self.previous_abs_y,
            },
            PathSegment::EllipticalArc {
                abs: _,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => PathSegment::EllipticalArc {
                abs: true,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x: x + self.previous_abs_x,
                y: y + self.previous_abs_y,
            },
            PathSegment::ClosePath { abs: _ } => PathSegment::ClosePath { abs: true },
        };
        // Update the previous absolute coordinates
        Self {
            inner: new_segment,
            _value: String::new(),
            previous_abs_x: self.previous_abs_x,
            previous_abs_y: self.previous_abs_y,
        }
    }
}

impl fmt::Display for PathSegmentWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = match self.inner {
            PathSegment::MoveTo { abs, x, y } => {
                format!("{}{} {}", if abs { "M" } else { "m" }, x, y)
            }
            PathSegment::LineTo { abs, x, y } => {
                format!("{}{} {}", if abs { "L" } else { "l" }, x, y)
            }
            PathSegment::HorizontalLineTo { abs, x } => {
                format!("{}{}", if abs { "H" } else { "h" }, x)
            }
            PathSegment::VerticalLineTo { abs, y } => {
                format!("{}{}", if abs { "V" } else { "v" }, y)
            }
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
                if abs { "C" } else { "c" },
                x1,
                y1,
                x2,
                y2,
                x,
                y
            ),
            PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                format!("{}{} {} {} {}", if abs { "S" } else { "s" }, x2, y2, x, y)
            }
            PathSegment::Quadratic { abs, x1, y1, x, y } => {
                format!("{}{} {} {} {}", if abs { "Q" } else { "q" }, x1, y1, x, y)
            }
            PathSegment::SmoothQuadratic { abs, x, y } => {
                format!("{}{} {}", if abs { "T" } else { "t" }, x, y)
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
                if abs { "A" } else { "a" },
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

/// A parsed path data.
#[derive(Debug)]
pub struct PathParsed {
    /// Segments of the path
    segments: Vec<PathSegmentWrap>,
}

/// https://github.com/linebender/svgtypes/pull/47
pub fn is_abs(path: &PathSegment) -> bool {
    match *path {
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

impl PathParsed {
    /// Returns the path segments vector.
    /// # Errors
    /// If the input string is not a valid path data.
    pub fn from(data: &str) -> Result<Self, Error> {
        let s = PathParser::from(data);
        let mut segments = Vec::new();
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;
        let mut initial_x = 0.0;
        let mut initial_y = 0.0;
        let mut last_was_end = true;
        for segment in s {
            let segment = segment?;
            let to_add = if is_abs(&segment) {
                match segment {
                    PathSegment::MoveTo { abs, x, y } => {
                        if last_was_end {
                            initial_x = x;
                            initial_y = y;
                            last_was_end = false;
                        }
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::MoveTo { abs, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        seg
                    }
                    PathSegment::LineTo { abs, x, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::LineTo { abs, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        last_was_end = false;
                        seg
                    }
                    PathSegment::HorizontalLineTo { abs, x } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::HorizontalLineTo { abs, x },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        last_was_end = false;
                        seg
                    }
                    PathSegment::VerticalLineTo { abs, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::VerticalLineTo { abs, y },
                            prev_x,
                            prev_y,
                        );
                        prev_y = y;
                        last_was_end = false;
                        seg
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
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::CurveTo {
                                abs,
                                x1,
                                y1,
                                x2,
                                y2,
                                x,
                                y,
                            },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        last_was_end = false;
                        seg
                    }
                    PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::SmoothCurveTo { abs, x2, y2, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        seg
                    }
                    PathSegment::Quadratic { abs, x1, y1, x, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::Quadratic { abs, x1, y1, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        seg
                    }
                    PathSegment::SmoothQuadratic { abs, x, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::SmoothQuadratic { abs, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        seg
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
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::EllipticalArc {
                                abs,
                                rx,
                                ry,
                                x_axis_rotation,
                                large_arc,
                                sweep,
                                x,
                                y,
                            },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        last_was_end = false;
                        seg
                    }
                    PathSegment::ClosePath { abs } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::ClosePath { abs },
                            prev_x,
                            prev_y,
                        );
                        // Reset to initial position
                        prev_x = initial_x;
                        prev_y = initial_y;
                        last_was_end = true;
                        seg
                    }
                }
            } else {
                match segment {
                    PathSegment::MoveTo { abs, x, y } => {
                        if last_was_end {
                            initial_x = prev_x + x;
                            initial_y = prev_y + y;
                            last_was_end = false;
                        }
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::MoveTo { abs, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x += x;
                        prev_y += y;
                        seg
                    }
                    PathSegment::LineTo { abs, x, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::LineTo { abs, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x += x;
                        prev_y += y;
                        last_was_end = false;
                        seg
                    }
                    PathSegment::HorizontalLineTo { abs, x } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::HorizontalLineTo { abs, x },
                            prev_x,
                            prev_y,
                        );
                        prev_x += x;
                        last_was_end = false;
                        seg
                    }
                    PathSegment::VerticalLineTo { abs, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::VerticalLineTo { abs, y },
                            prev_x,
                            prev_y,
                        );
                        prev_y += y;
                        last_was_end = false;
                        seg
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
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::CurveTo {
                                abs,
                                x1,
                                y1,
                                x2,
                                y2,
                                x,
                                y,
                            },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        last_was_end = false;
                        seg
                    }
                    PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::SmoothCurveTo { abs, x2, y2, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        seg
                    }
                    PathSegment::Quadratic { abs, x1, y1, x, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::Quadratic { abs, x1, y1, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        seg
                    }
                    PathSegment::SmoothQuadratic { abs, x, y } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::SmoothQuadratic { abs, x, y },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        seg
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
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::EllipticalArc {
                                abs,
                                rx,
                                ry,
                                x_axis_rotation,
                                large_arc,
                                sweep,
                                x,
                                y,
                            },
                            prev_x,
                            prev_y,
                        );
                        prev_x = x;
                        prev_y = y;
                        last_was_end = false;
                        seg
                    }
                    PathSegment::ClosePath { abs } => {
                        let seg = PathSegmentWrap::new_with_pos(
                            PathSegment::ClosePath { abs },
                            prev_x,
                            prev_y,
                        );
                        // Reset to initial position
                        prev_x = initial_x;
                        prev_y = initial_y;
                        last_was_end = true;
                        seg
                    }
                }
            };

            segments.push(to_add);
        }
        Ok(Self { segments })
    }

    /// Returns the path segments vector.
    pub fn segments(&mut self) -> &mut Vec<PathSegmentWrap> {
        &mut self.segments
    }

    /// Returns the path data as a string.
    pub fn path_data(&self) -> String {
        self.path_data_with_separator("")
    }

    /// Returns the path data as a string.
    pub fn path_data_with_separator(&self, separator: &str) -> String {
        let strings = self.segments.iter().map(|r| r.to_string());
        strings.collect::<Vec<String>>().join(separator)
    }

    /// Converts all segments to relative.
    pub fn to_relative(&self) -> Self {
        let new_segments = self
            .segments
            .iter()
            .map(|segment| segment.to_relative())
            .collect::<Vec<PathSegmentWrap>>();
        Self {
            segments: new_segments,
        }
    }

    /// Converts all segments to absolute.
    pub fn to_absolute(&self) -> Self {
        let new_segments = self
            .segments
            .iter()
            .map(|segment| segment.to_absolute())
            .collect::<Vec<PathSegmentWrap>>();
        Self {
            segments: new_segments,
        }
    }

    /// Toggles the segment at the given index between absolute and relative coordinates.
    pub fn toggle_segment(&mut self, idx: usize) {
        if let Some(segment) = self.segments.get_mut(idx) {
            if segment.is_abs() {
                *segment = segment.to_relative();
            } else {
                *segment = segment.to_absolute();
            }
        }
    }
}

impl fmt::Display for PathParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_parsed() {
        let parsed = PathParsed::from("M174 270C113 271 84 237 80 207 78 189 79 172 76 172 46 154 34 142 21 109-4 49 45 62 55 66 71 72 79 83 90 94 103 104 160 104 174 104ZM110 150C104 153 98 160 95 166S92 184 95 190C103 208 126 216 142 205 149 200 153 195 155 188 161 172 150 153 133 147 128 145 115 147 110 150ZM174 270C236 271 265 237 269 207 271 189 270 172 273 172 303 154 315 142 328 109 353 49 304 62 294 66 278 72 270 83 259 94 246 104 189 104 174 104ZM239 150C245 153 251 160 254 166S257 184 254 190C246 208 223 216 207 205 200 200 196 195 194 188 188 172 199 153 216 147 221 145 234 147 239 150Z").unwrap();
        assert_eq!(parsed.to_string(), "M174 270C113 271 84 237 80 207C78 189 79 172 76 172C46 154 34 142 21 109C-4 49 45 62 55 66C71 72 79 83 90 94C103 104 160 104 174 104ZM110 150C104 153 98 160 95 166S92 184 95 190C103 208 126 216 142 205C149 200 153 195 155 188C161 172 150 153 133 147C128 145 115 147 110 150ZM174 270C236 271 265 237 269 207C271 189 270 172 273 172C303 154 315 142 328 109C353 49 304 62 294 66C278 72 270 83 259 94C246 104 189 104 174 104ZM239 150C245 153 251 160 254 166S257 184 254 190C246 208 223 216 207 205C200 200 196 195 194 188C188 172 199 153 216 147C221 145 234 147 239 150Z");
        assert_eq!(
            parsed.to_absolute().to_string(),
            "M174 270C113 271 84 237 80 207C78 189 79 172 76 172C46 154 34 142 21 109C-4 49 45 62 55 66C71 72 79 83 90 94C103 104 160 104 174 104ZM110 150C104 153 98 160 95 166S92 184 95 190C103 208 126 216 142 205C149 200 153 195 155 188C161 172 150 153 133 147C128 145 115 147 110 150ZM174 270C236 271 265 237 269 207C271 189 270 172 273 172C303 154 315 142 328 109C353 49 304 62 294 66C278 72 270 83 259 94C246 104 189 104 174 104ZM239 150C245 153 251 160 254 166S257 184 254 190C246 208 223 216 207 205C200 200 196 195 194 188C188 172 199 153 216 147C221 145 234 147 239 150Z"
        );
        assert_eq!(
            parsed.to_relative().to_string(),
            "m174 270c-61 1 -90 -33 -94 -63c-2 -18 -1 -35 -4 -35c-30 -18 -42 -30 -55 -63c-25 -60 24 -47 34 -43c16 6 24 17 35 28c13 10 70 10 84 10zm-64 -120c-6 3 -12 10 -15 16s-3 18 0 24c8 18 31 26 47 15c7 -5 11 -10 13 -17c6 -16 -5 -35 -22 -41c-5 -2 -18 0 -23 3zm64 120c62 1 91 -33 95 -63c2 -18 1 -35 4 -35c30 -18 42 -30 55 -63c25 -60 -24 -47 -34 -43c-16 6 -24 17 -35 28c-13 10 -70 10 -85 10zm65 -120c6 3 12 10 15 16s3 18 0 24c-8 18 -31 26 -47 15c-7 -5 -11 -10 -13 -17c-6 -16 5 -35 22 -41c5 -2 18 0 23 3z"
        );

        let parsed_relative = PathParsed::from("m174 270c-61 1 -90 -33 -94 -63c-2 -18 -1 -35 -4 -35c-30 -18 -42 -30 -55 -63c-25 -60 24 -47 34 -43c16 6 24 17 35 28c13 10 70 10 84 10zm-64 -120c-6 3 -12 10 -15 16s-3 18 0 24c8 18 31 26 47 15c7 -5 11 -10 13 -17c6 -16 -5 -35 -22 -41c-5 -2 -18 0 -23 3zm64 120c62 1 91 -33 95 -63c2 -18 1 -35 4 -35c30 -18 42 -30 55 -63c25 -60 -24 -47 -34 -43c-16 6 -24 17 -35 28c-13 10 -70 10 -85 10zm65 -120c6 3 12 10 15 16s3 18 0 24c-8 18 -31 26 -47 15c-7 -5 -11 -10 -13 -17c-6 -16 5 -35 22 -41c5 -2 18 0 23 3z").unwrap();

        assert_eq!(parsed_relative.to_string(), "m174 270c-61 1 -90 -33 -94 -63c-2 -18 -1 -35 -4 -35c-30 -18 -42 -30 -55 -63c-25 -60 24 -47 34 -43c16 6 24 17 35 28c13 10 70 10 84 10zm-64 -120c-6 3 -12 10 -15 16s-3 18 0 24c8 18 31 26 47 15c7 -5 11 -10 13 -17c6 -16 -5 -35 -22 -41c-5 -2 -18 0 -23 3zm64 120c62 1 91 -33 95 -63c2 -18 1 -35 4 -35c30 -18 42 -30 55 -63c25 -60 -24 -47 -34 -43c-16 6 -24 17 -35 28c-13 10 -70 10 -85 10zm65 -120c6 3 12 10 15 16s3 18 0 24c-8 18 -31 26 -47 15c-7 -5 -11 -10 -13 -17c-6 -16 5 -35 22 -41c5 -2 18 0 23 3z");
        assert_eq!(
            parsed_relative.to_absolute().to_string(),
            "M174 270C113 271 84 237 80 207C78 189 79 172 76 172C46 154 34 142 21 109C-4 49 45 62 55 66C71 72 79 83 90 94C103 104 160 104 174 104ZM110 150C104 153 98 160 95 166S92 184 95 190C103 208 126 216 142 205C149 200 153 195 155 188C161 172 150 153 133 147C128 145 115 147 110 150ZM174 270C236 271 265 237 269 207C271 189 270 172 273 172C303 154 315 142 328 109C353 49 304 62 294 66C278 72 270 83 259 94C246 104 189 104 174 104ZM239 150C245 153 251 160 254 166S257 184 254 190C246 208 223 216 207 205C200 200 196 195 194 188C188 172 199 153 216 147C221 145 234 147 239 150Z");
    }
}
