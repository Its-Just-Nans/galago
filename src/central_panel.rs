//! Central panel
use bladvak::{
    eframe::egui::{self, Pos2, Rect},
    log,
};

use crate::GalagoApp;

impl GalagoApp {
    /// Central panel
    pub(crate) fn app_central_panel(
        &mut self,
        ui: &mut egui::Ui,
        _error_manager: &mut bladvak::ErrorManager,
    ) {
        self.svg_is_valid = match self.update_svg(ui.ctx()) {
            Ok(()) => true,
            Err(e) => {
                if let Some(err) = e {
                    log::error!("SVG render error: {err}");
                }
                false
            }
        };
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
}
