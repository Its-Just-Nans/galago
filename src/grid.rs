//! Grid

use std::f32::consts::PI;

use egui::{emath::Rot2, vec2, Color32, Painter, Rect, Stroke, Vec2};

/// Grid options
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Grid {
    /// Spacing between grid lines.
    pub spacing: Vec2,

    /// Angle of the grid.
    pub angle: f32,

    /// Color of the grid lines.
    pub color: Color32,

    /// Stroke width of the grid lines.
    pub stroke_width: f32,
}
const DEFAULT_GRID_SPACING: Vec2 = vec2(20.0, 20.0);

/// Gri rotation
const DEFAULT_GRID_ANGLE: f32 = 0.0;

impl Default for Grid {
    fn default() -> Self {
        Self {
            spacing: DEFAULT_GRID_SPACING,
            angle: DEFAULT_GRID_ANGLE,
            color: Color32::BLACK,
            stroke_width: 0.5,
        }
    }
}

impl Grid {
    /// Tittle
    pub fn title(&self) -> &'static str {
        "Grid"
    }

    /// Grid settings
    pub fn show_settings(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Spacing:");
            ui.add(
                egui::DragValue::new(&mut self.spacing.x)
                    .speed(0.1)
                    .range(1.0..=100.0),
            );
            ui.add(
                egui::DragValue::new(&mut self.spacing.y)
                    .speed(0.1)
                    .range(1.0..=100.0),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Angle:");

            ui.add(
                egui::DragValue::new(&mut self.angle)
                    .speed(0.01)
                    .range(-PI..=PI),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Color:");
            ui.color_edit_button_srgba(&mut self.color);
        });
        ui.add(egui::Slider::new(&mut self.stroke_width, 0.1..=5.0).text("Stroke Width"));
    }
    /// draw the grid
    pub fn draw(&self, viewport: &Rect, painter: &Painter) {
        let bg_stroke = Stroke::new(self.stroke_width, self.color);

        let spacing = vec2(self.spacing.x.max(1.0), self.spacing.y.max(1.0));

        let rot = Rot2::from_angle(self.angle);
        let rot_inv = rot.inverse();

        let pattern_bounds = viewport.rotate_bb(rot_inv);

        let min_x = (pattern_bounds.min.x / spacing.x).ceil();
        let max_x = (pattern_bounds.max.x / spacing.x).floor();

        #[allow(clippy::cast_possible_truncation)]
        for x in 0..=f32::ceil(max_x - min_x) as i64 {
            #[allow(clippy::cast_precision_loss)]
            let x = (x as f32 + min_x) * spacing.x;

            let top = (rot * vec2(x, pattern_bounds.min.y)).to_pos2();
            let bottom = (rot * vec2(x, pattern_bounds.max.y)).to_pos2();

            painter.line_segment([top, bottom], bg_stroke);
        }

        let min_y = (pattern_bounds.min.y / spacing.y).ceil();
        let max_y = (pattern_bounds.max.y / spacing.y).floor();

        #[allow(clippy::cast_possible_truncation)]
        for y in 0..=f32::ceil(max_y - min_y) as i64 {
            #[allow(clippy::cast_precision_loss)]
            let y = (y as f32 + min_y) * spacing.y;

            let top = (rot * vec2(pattern_bounds.min.x, y)).to_pos2();
            let bottom = (rot * vec2(pattern_bounds.max.x, y)).to_pos2();

            painter.line_segment([top, bottom], bg_stroke);
        }
    }
}
