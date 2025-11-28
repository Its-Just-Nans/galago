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
        let bytes = self.svg.as_bytes().to_vec();
        self.save_file(&bytes, path_file)
    }

    /// Save the data to a file
    #[cfg(not(target_arch = "wasm32"))]
    fn save_file(&mut self, data: &[u8], path_file: &PathBuf) -> Result<(), String> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut file = File::create(path_file).map_err(|e| format!("Cannot create file: {e}"))?;
        file.write_all(data)
            .map_err(|e| format!("Cannot write file: {e}"))
    }

    #[cfg(target_arch = "wasm32")]
    fn save_file(&mut self, data: &[u8], path_file: &PathBuf) -> Result<(), String> {
        // create blob
        use eframe::wasm_bindgen::JsCast;
        use js_sys::Array;

        log::info!("Saving file to {:?}", path_file);
        let filename = match path_file.file_name() {
            Some(name) => name.to_str().ok_or("Cannot get filename")?,
            None => "file.svg",
        };

        let array_data = Array::new();
        array_data.push(&js_sys::Uint8Array::from(data));
        let blob = web_sys::Blob::new_with_u8_array_sequence(&array_data)
            .map_err(|_| "Cannot create svg data")?;
        let url = web_sys::Url::create_object_url_with_blob(&blob)
            .map_err(|_| "Cannot create svg url data")?;
        // create link
        let document = web_sys::window()
            .ok_or("Cannot get the website window")?
            .document()
            .ok_or("Cannot get the website document")?;
        let a = document
            .create_element("a")
            .map_err(|_| "Cannot create <a> element")?;
        a.set_attribute("href", &url)
            .map_err(|_| "Cannot create add href attribute")?;
        a.set_attribute("download", filename)
            .map_err(|_| "Cannot create add download attribute")?;

        // click link
        a.dyn_ref::<web_sys::HtmlElement>()
            .ok_or("Cannot simulate click")?
            .click();
        // revoke url
        web_sys::Url::revoke_object_url(&url)
            .map_err(|_| "Cannot remove object url with revoke_object_url".into())
    }
}
