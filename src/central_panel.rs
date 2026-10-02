//! Central panel
use bladvak::{
    eframe::egui::{self, Color32, Pos2, Rect, Stroke},
    log,
};

use crate::GalagoApp;

impl GalagoApp {
    /// Central panel
    #[allow(clippy::too_many_lines)]
    pub(crate) fn app_central_panel(
        &mut self,
        ui: &mut egui::Ui,
        error_manager: &mut bladvak::ErrorManager,
    ) {
        if self.documents.get_current_doc_mut().is_none() {
            bladvak::utils::central_ui(ui, |ui| {
                ui.heading(concat!("Welcome to ", env!("CARGO_PKG_NAME")));
                ui.label("No document opened");
            });
            return;
        }
        let svg_is_valid = match self.update_svg(ui.ctx()) {
            Ok(()) => true,
            Err(e) => {
                if let Some(err) = e {
                    log::error!("SVG render error: {err}");
                }
                false
            }
        };
        let Some(document) = self.documents.get_current_doc_mut() else {
            return;
        };
        document.svg_is_valid = svg_is_valid;
        let rect = ui.available_rect_before_wrap();
        let response = egui::Scene::new()
            .max_inner_size([350.0, 1000.0])
            .zoom_range(0.1..=50.0)
            .show(ui, &mut document.scene_rect, |ui| {
                let painter = ui.painter();
                let bg_r: egui::Response = ui.response();
                if bg_r.rect.is_finite() {
                    self.grid.draw(&bg_r.rect, painter);
                }
                let _response = document.svg_render.show(ui);
                document.shape_editor.show_in_scene(ui, error_manager);

                if error_manager.is_debug()
                    && bg_r.dragged()
                    && let Some(pos) = bg_r.interact_pointer_pos()
                {
                    ui.painter()
                        .circle(pos, 0.0, Color32::BLUE, Stroke::default());
                }

                // if response.clicked() {
                //     println!("SVG clicked!");
                // }
            })
            .response;

        if document.should_reset_view || response.double_clicked() {
            let real_rect = Rect::from_two_pos(Pos2::ZERO, (rect.max - rect.min).to_pos2());
            document.scene_rect = real_rect;
        }
    }
}
