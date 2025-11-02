//! Top panel of Galago. Handling the menu and save function
use std::path::PathBuf;

use egui::{Context, ThemePreference};

use crate::GalagoApp;

impl GalagoApp {
    /// Show the top panel of galago
    pub(crate) fn top_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let is_web = cfg!(target_arch = "wasm32");
                    if !is_web && ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Open").clicked() {
                        ui.close();
                        self.file_handler.handle_file_open();
                    }
                    if ui.button("Save").clicked() {
                        ui.close();
                        let save_path = self.get_save_path();
                        if let Some(save_path) = self.error_manager.handle_error(save_path) {
                            let res = self.save_svg(&save_path);
                            self.error_manager.handle_error(res);
                        }
                    }
                    if ui.button("Settings").clicked() {
                        self.settings.open = true;
                    }
                    ui.menu_button("Theme", |ui| {
                        let mut theme_preference = ui.ctx().options(|opt| opt.theme_preference);
                        ui.selectable_value(
                            &mut theme_preference,
                            ThemePreference::Light,
                            "☀ Light",
                        );
                        ui.selectable_value(
                            &mut theme_preference,
                            ThemePreference::Dark,
                            "🌙 Dark",
                        );
                        ui.selectable_value(
                            &mut theme_preference,
                            ThemePreference::System,
                            "💻 System",
                        );
                        ui.ctx().set_theme(theme_preference);
                    });
                    ui.add(
                        egui::Hyperlink::from_label_and_url(
                            "Github repo",
                            "https://github.com/Its-Just-Nans/galago",
                        )
                        .open_in_new_tab(true),
                    );
                    egui::warn_if_debug_build(ui);
                });
                ui.separator();
                self.should_reset_view = ui.button("Double click to Reset view").clicked();
                ui.separator();
                if ui.button("Copy").clicked() {
                    ctx.copy_text(self.svg.clone());
                }
            });
        });
    }

    /// Get the save path
    /// # Errors
    /// Failed if the input is wrong
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_save_path(&mut self) -> Result<PathBuf, String> {
        use rfd::FileDialog;
        use std::path::Path;
        let path = FileDialog::new()
            .set_directory(match &self.save_path {
                Some(path) => path.parent().ok_or("Cannot get parent in the path")?,
                None => std::path::Path::new("."),
            })
            .set_file_name(match &self.save_path {
                Some(path) => path
                    .file_name()
                    .ok_or("Cannot get file name")?
                    .to_string_lossy(),
                None => std::path::Path::new("file").to_string_lossy(),
            })
            .save_file();
        let res = if let Some(path) = path {
            self.save_path = Some(path.clone());
            path
        } else {
            Path::new(".").to_path_buf()
        };
        Ok(res)
    }
    /// Get a new path
    /// # Errors
    /// No error in wasm
    #[cfg(target_arch = "wasm32")]
    pub fn get_save_path(&mut self) -> Result<PathBuf, String> {
        let filename = format!("file.svg");
        Ok(PathBuf::from(filename))
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
