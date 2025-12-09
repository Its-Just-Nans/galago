//! Central panel
use bladvak::{
    eframe::egui::{self, Pos2, Rect, Window},
    log,
};

use crate::GalagoApp;

impl GalagoApp {
    /// Central panel
    pub(crate) fn app_central_panel(
        &mut self,
        ui: &mut egui::Ui,
        error_manager: &mut bladvak::ErrorManager,
    ) {
        self.svg_is_valid = match self.svg_render.update(ui.ctx(), &self.svg) {
            Ok(_) => true,
            Err(e) => {
                if let Some(err) = e {
                    log::error!("SVG render error: {err}");
                }
                false
            }
        };
        self.central_panel_windows(ui, error_manager);
        let rect = ui.available_rect_before_wrap();
        let response = egui::Scene::new()
            .max_inner_size([350.0, 1000.0])
            .zoom_range(0.1..=50.0)
            .show(ui, &mut self.scene_rect, |ui| {
                let painter = ui.painter();
                let bg_r: egui::Response = ui.response();
                if bg_r.rect.is_finite() {
                    self.grid.draw(&bg_r.rect, painter);
                }
                let _response = self.svg_render.show(ui);
                // if response.clicked() {
                //     println!("SVG clicked!");
                // }
            })
            .response;

        if self.should_reset_view || response.double_clicked() {
            let real_rect = Rect::from_two_pos(Pos2::ZERO, (rect.max - rect.min).to_pos2());
            self.scene_rect = real_rect;
        }
    }

    /// Other central panel
    fn central_panel_windows(
        &mut self,
        ui: &mut egui::Ui,
        error_manager: &mut bladvak::ErrorManager,
    ) {
        let ctx = ui.ctx();
        if self.string_viewer.is_windows {
            let mut current_open = true;
            Window::new(self.string_viewer.title())
                .min_width(500.0)
                .min_height(100.0)
                .open(&mut current_open)
                .resizable(true)
                .show(ctx, |ui| {
                    self.string_viewer
                        .show(ui, &mut self.svg, self.svg_is_valid, error_manager);
                });
            self.string_viewer.is_windows = current_open;
        }
        if self.tree_viewer.is_windows {
            let mut current_open = true;

            Window::new(self.tree_viewer.title())
                .resizable(true)
                .min_width(500.0)
                .min_height(100.0)
                .open(&mut current_open)
                .show(ctx, |ui| {
                    self.tree_viewer.show(ui, &mut self.svg, error_manager);
                });
            self.tree_viewer.is_windows = current_open;
        }
    }
}
