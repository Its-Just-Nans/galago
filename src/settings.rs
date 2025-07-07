use egui::{Context, Id, Modal};

use crate::grid::Grid;

#[derive(Default, serde::Deserialize, serde::Serialize)]

pub struct TreeViewerSettings {
    pub is_windows: bool,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct StringViewerSettings {
    pub is_windows: bool,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub open: bool,
    pub grid: Grid,
    pub tree_settings: TreeViewerSettings,
    pub string_viewer: StringViewerSettings,
}

impl Settings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn show(&mut self, ctx: &Context) {
        if self.open {
            let modal = Modal::new(Id::new("Modal A")).show(ctx, |ui| {
                ui.checkbox(&mut self.tree_settings.is_windows, "Tree as windows");
                ui.checkbox(&mut self.string_viewer.is_windows, "String as windows");
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |ui| {
                        if ui.button("Close").clicked() {
                            // ui.close();
                        }
                    },
                );
            });
            if modal.should_close() {
                self.open = false;
            }
        }
    }
}
