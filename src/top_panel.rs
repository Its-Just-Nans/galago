//! Top panel of Galago. Handling the menu and save function
use bladvak::eframe::egui;
use std::path::Path;

use crate::GalagoApp;

impl GalagoApp {
    /// Show the top panel of galago
    pub(crate) fn app_top_panel(
        &mut self,
        ui: &mut egui::Ui,
        _error_manager: &mut bladvak::ErrorManager,
    ) {
        ui.separator();
        self.documents.show_file_list(ui);
    }

    /// Save the current svg
    pub(crate) fn save_svg(&mut self, path_file: &Path) -> Result<(), String> {
        let Some(document) = self.documents.get_current_doc_mut() else {
            return Err("No svg document".into());
        };
        let bytes = document.svg.as_bytes();
        bladvak::utils::save_file(bytes, path_file)
    }
}
