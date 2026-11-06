//! Path segment

use std::fmt::Display;

use svgtypes::PathSegment;

/// Galago version of `PathSegment`
#[allow(missing_docs)]
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum GalagoPathSegment {
    MoveTo {
        abs: bool,
        x: f64,
        y: f64,
    },
    LineTo {
        abs: bool,
        x: f64,
        y: f64,
    },
    HorizontalLineTo {
        abs: bool,
        x: f64,
    },
    VerticalLineTo {
        abs: bool,
        y: f64,
    },
    CurveTo {
        abs: bool,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    SmoothCurveTo {
        abs: bool,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
    },
    Quadratic {
        abs: bool,
        x1: f64,
        y1: f64,
        x: f64,
        y: f64,
    },
    SmoothQuadratic {
        abs: bool,
        x: f64,
        y: f64,
    },
    EllipticalArc {
        abs: bool,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    },
    ClosePath {
        abs: bool,
    },
}

impl From<PathSegment> for GalagoPathSegment {
    fn from(value: PathSegment) -> Self {
        match value {
            PathSegment::MoveTo { abs, x, y } => Self::MoveTo { abs, x, y },
            PathSegment::LineTo { abs, x, y } => Self::LineTo { abs, x, y },
            PathSegment::HorizontalLineTo { abs, x } => Self::HorizontalLineTo { abs, x },
            PathSegment::VerticalLineTo { abs, y } => Self::VerticalLineTo { abs, y },
            PathSegment::CurveTo {
                abs,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => Self::CurveTo {
                abs,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            },
            PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                Self::SmoothCurveTo { abs, x2, y2, x, y }
            }
            PathSegment::Quadratic { abs, x1, y1, x, y } => Self::Quadratic { abs, x1, y1, x, y },
            PathSegment::SmoothQuadratic { abs, x, y } => Self::SmoothQuadratic { abs, x, y },
            PathSegment::EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => Self::EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            },
            PathSegment::ClosePath { abs } => Self::ClosePath { abs },
        }
    }
}

impl GalagoPathSegment {
    /// Returns `true` if the segment is absolute.
    pub fn is_abs(&self) -> bool {
        match *self {
            Self::MoveTo { abs, .. }
            | Self::LineTo { abs, .. }
            | Self::HorizontalLineTo { abs, .. }
            | Self::VerticalLineTo { abs, .. }
            | Self::CurveTo { abs, .. }
            | Self::SmoothCurveTo { abs, .. }
            | Self::Quadratic { abs, .. }
            | Self::SmoothQuadratic { abs, .. }
            | Self::EllipticalArc { abs, .. }
            | Self::ClosePath { abs } => abs,
        }
    }

    /// Returns the segment letter.
    pub fn command(&self) -> u8 {
        if self.is_abs() {
            match *self {
                Self::MoveTo { .. } => b'M',
                Self::LineTo { .. } => b'L',
                Self::HorizontalLineTo { .. } => b'H',
                Self::VerticalLineTo { .. } => b'V',
                Self::CurveTo { .. } => b'C',
                Self::SmoothCurveTo { .. } => b'S',
                Self::Quadratic { .. } => b'Q',
                Self::SmoothQuadratic { .. } => b'T',
                Self::EllipticalArc { .. } => b'A',
                Self::ClosePath { .. } => b'Z',
            }
        } else {
            match *self {
                Self::MoveTo { .. } => b'm',
                Self::LineTo { .. } => b'l',
                Self::HorizontalLineTo { .. } => b'h',
                Self::VerticalLineTo { .. } => b'v',
                Self::CurveTo { .. } => b'c',
                Self::SmoothCurveTo { .. } => b's',
                Self::Quadratic { .. } => b'q',
                Self::SmoothQuadratic { .. } => b't',
                Self::EllipticalArc { .. } => b'a',
                Self::ClosePath { .. } => b'z',
            }
        }
    }

    /// `GalagoPathSegment` to string
    pub(crate) fn string_impl(&self) -> String {
        let letter = self.command() as char;
        match self {
            GalagoPathSegment::MoveTo { abs: _, x, y }
            | GalagoPathSegment::LineTo { abs: _, x, y } => {
                // move to
                format!("{letter}{x} {y}")
            }
            GalagoPathSegment::HorizontalLineTo { abs: _, x } => {
                format!("{letter}{x}")
            }
            GalagoPathSegment::VerticalLineTo { abs: _, y } => {
                format!("{letter}{y}")
            }
            GalagoPathSegment::CurveTo {
                abs: _,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => format!("{letter}{x1} {y1} {x2} {y2} {x} {y}"),
            GalagoPathSegment::SmoothCurveTo {
                abs: _,
                x2,
                y2,
                x,
                y,
            } => {
                format!("{letter}{x2} {y2} {x} {y}")
            }
            GalagoPathSegment::Quadratic {
                abs: _,
                x1,
                y1,
                x,
                y,
            } => {
                format!("{letter}{x1} {y1} {x} {y}")
            }
            GalagoPathSegment::SmoothQuadratic { abs: _, x, y } => {
                // smooth quadratic
                format!("{letter}{x} {y}")
            }
            GalagoPathSegment::EllipticalArc {
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
                letter,
                rx,
                ry,
                x_axis_rotation,
                if *large_arc { "1" } else { "0" },
                if *sweep { "1" } else { "0" },
                x,
                y
            ),
            GalagoPathSegment::ClosePath { abs: _ } => letter.to_string(),
        }
    }
}

impl Display for GalagoPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.string_impl())
    }
}
