//! Top panel of Galago. Handling the menu and save function
use bladvak::eframe::egui;
use std::path::PathBuf;

use crate::GalagoApp;

impl GalagoApp {
    /// Show the top panel of galago
    pub(crate) fn app_top_panel(
        &mut self,
        ui: &mut egui::Ui,
        _error_manager: &mut bladvak::ErrorManager,
    ) {
        ui.separator();
        self.should_reset_view = ui.button("Double click to Reset view").clicked();
        ui.separator();
        if ui.button("Copy").clicked() {
            ui.ctx().copy_text(self.svg.clone());
        }
    }

    /// Save the current svg
    pub(crate) fn save_svg(&mut self, path_file: &PathBuf) -> Result<(), String> {
        let bytes = self.svg.as_bytes();
        bladvak::utils::save_file(bytes, path_file)
    }
}
