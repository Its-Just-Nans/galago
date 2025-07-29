//! This module provides functions to convert various geometric shapes and text into SVG path data strings.
// use std::{fs::File, io::Read};

// use font_kit::{handle::Handle, source::SystemSource};
// use rusttype::{Font, OutlineBuilder, Point, Scale};

// /// Text to path
// pub fn text_to_path(
//     text: &str,
//     font_name: &str,
//     size: f32,
//     x: f32,
//     y: f32,
//     letter_spacing: f32,
// ) -> Result<String, String> {
//     let handle = SystemSource::new()
//         .select_family_by_name(font_name)
//         .map_err(|e| format!("Failed to select font family: {e} for {font_name}"))?
//         .fonts()[0]
//         .clone();

//     let font = match handle {
//         Handle::Path { path, font_index } => {
//             let mut file = File::open(path).unwrap();
//             let mut buf = Vec::new();
//             file.read_to_end(&mut buf).unwrap();
//             Font::try_from_vec_and_index(buf, font_index).unwrap()
//         }
//         Handle::Memory { bytes, font_index } => {
//             Font::try_from_vec_and_index(bytes.to_vec(), font_index).unwrap()
//         }
//     };
//     text_to_path_wit_font(text, &font, size, x, y, letter_spacing)
// }

// /// Convert a text to a path
// pub fn text_to_path_wit_font(
//     text: &str,
//     font: &Font<'_>,
//     size: f32,
//     x: f32,
//     y: f32,
//     letter_spacing: f32,
// ) -> Result<String, String> {
//     let scale = Scale::uniform(size);
//     let v_metrics = font.v_metrics(scale);
//     let glyphs_height = v_metrics.ascent - v_metrics.descent;

//     let mut builder = PathBuilder::new(x, y);
//     for glyph in font.layout(
//         text,
//         scale,
//         Point {
//             x,
//             y: x + v_metrics.ascent,
//         },
//     ) {
//         let bounding_box = glyph.unpositioned().exact_bounding_box().unwrap();
//         builder.x += bounding_box.min.x;
//         builder.y = glyphs_height + bounding_box.min.y;
//         glyph.build_outline(&mut builder);

//         builder.x += bounding_box.width() + letter_spacing;
//     }
//     Ok(builder.buffer.clone())
// }

// /// PathBuilder
// pub struct PathBuilder {
//     /// The x coordinate offset
//     x: f32,
//     /// The y coordinate offset
//     y: f32,
//     /// The buffer to store the path commands
//     buffer: String,
// }

// impl PathBuilder {
//     /// Create a new instance of the Builder
//     pub fn new(x: f32, y: f32) -> Self {
//         Self {
//             x,
//             y,
//             buffer: String::new(),
//         }
//     }
// }

// impl OutlineBuilder for PathBuilder {
//     fn move_to(&mut self, x: f32, y: f32) {
//         self.buffer
//             .push_str(&format!("M{} {}", x + self.x, y + self.y));
//     }

//     fn line_to(&mut self, x: f32, y: f32) {
//         self.buffer
//             .push_str(&format!("L{} {}", x + self.x, y + self.y));
//     }

//     fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
//         self.buffer.push_str(&format!(
//             "Q{} {},{} {}",
//             x1 + self.x,
//             y1 + self.y,
//             x + self.x,
//             y + self.y
//         ));
//     }

//     fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
//         self.buffer.push_str(&format!(
//             "C{} {},{} {},{} {}",
//             x1 + self.x,
//             y1 + self.y,
//             x2 + self.x,
//             y2 + self.y,
//             x + self.x,
//             y + self.y
//         ));
//     }

//     fn close(&mut self) {
//         self.buffer.push('Z');
//     }
// }

// /// Tests for the transformation module
// mod test {

//     #[test]
//     fn render_str() {
//         let x = 10.;
//         let y = 20.;

//         let result_text =
//             crate::transformation::text_to_path("text-svg", "Noto Sans", 50.0, x, y, 1.0).unwrap();
//         assert_eq!(result_text, "sdfq");
//     }
// }

/// Convert a polyline string to an SVG path data string
pub fn polyline_to_path(points: &str) -> String {
    let mut path_data = String::new();
    let mut first = true;
    let all_points = points.split_whitespace().collect::<Vec<&str>>();
    for idx in (0..all_points.len()).step_by(2) {
        let coords1 = all_points[idx];
        let coords2 = all_points[idx + 1];
        if first {
            path_data.push_str(&format!("M {coords1} {coords2}"));
            first = false;
        } else {
            path_data.push_str(&format!(" L {coords1} {coords2}"));
        }
    }
    path_data
}

/// Convert a line string to an SVG path data string
pub fn line_to_path(x1: &str, y1: &str, x2: &str, y2: &str) -> String {
    format!("M {x1} {y1} L {x2} {y2}")
}

/// Convert a polygon string to an SVG path data string
pub fn polygon_to_path(points: &str) -> String {
    let mut path_data = String::new();
    let mut first = true;
    let all_points = points.split_whitespace().collect::<Vec<&str>>();
    for idx in (0..all_points.len()).step_by(2) {
        let coords1 = all_points[idx];
        let coords2 = all_points[idx + 1];
        if first {
            path_data.push_str(&format!("M {coords1} {coords2}"));
            first = false;
        } else {
            path_data.push_str(&format!(" L {coords1} {coords2}"));
        }
    }
    path_data.push('Z'); // Close the polygon
    path_data
}

/// Convert a svg rect to a path data string
pub fn rect_to_path(x: &str, y: &str, width: &str, height: &str) -> String {
    format!("M {x} {y} h {width} v {height} h -{width} Z")
}

/// Convert a circle to a path data string
pub fn circle_to_path(cx: &str, cy: &str, r: &str) -> String {
    let double_r = r.parse::<f32>().unwrap() * 2.0;
    format!("M {cx} {cy} m {r}, 0a {r},{r} 0 1,0 -{double_r},0 a {r},{r} 0 1,0 {double_r},0")
}

/// Convert an ellipse to a path data string
pub fn ellipse_to_path(cx: &str, cy: &str, rx: &str, ry: &str) -> String {
    let double_rx = rx.parse::<f32>().unwrap() * 2.0;
    let start_x = cx.parse::<f32>().unwrap() - rx.parse::<f32>().unwrap();
    format!("M{start_x} {cy} a{rx} {ry} 0 1,0 {double_rx} 0 a{rx} {ry} 0 1,0 -{double_rx} 0 Z")
}
