//! Galago Settings

use egui::{Context, Id, Modal};

/// Settings of Galago
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    /// Is setting modal open
    pub open: bool,
}

impl Settings {
    /// Create new Settings struct
    pub fn new() -> Self {
        Default::default()
    }

    /// Show settings Ui
    pub fn show(&mut self, ctx: &Context, ui_fn: impl FnOnce(&mut egui::Ui)) {
        if self.open {
            let modal = Modal::new(Id::new("Modal A")).show(ctx, |ui| {
                ui.label(format!("{} settings", env!("CARGO_PKG_NAME")));
                ui.separator();
                ui_fn(ui);
                egui::Sides::new().show(
                    ui,
                    |_ui| {},
                    |modal_ui| {
                        if modal_ui.button("Close").clicked() {
                            modal_ui.close();
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
