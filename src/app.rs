//! Galago app

use bladvak::eframe::egui::{self};
use bladvak::utils::grid::Grid;
use bladvak::{AppError, BladvakApp, eframe};
use std::fmt::Debug;
use std::path::PathBuf;

use crate::{string_viewer::StringViewer, svg_render::SvgRender, tree_viewer::TreeViewer};

/// GalagoApp struct
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GalagoApp {
    /// Save of the SVG
    pub(crate) saved_svg: String,

    /// SVG Screen
    pub(crate) svg: String,

    /// Current scene zoom
    #[serde(skip)]
    pub(crate) scene_rect: egui::Rect,

    /// TreeViewer Ui
    pub(crate) tree_viewer: TreeViewer,

    /// StringViewer Ui
    pub(crate) string_viewer: StringViewer,

    /// Grid options
    pub(crate) grid: Grid,

    /// SvgRender
    pub(crate) svg_render: SvgRender,

    /// should reset the view
    pub(crate) should_reset_view: bool,

    /// Path to save the svg
    pub save_path: Option<PathBuf>,

    /// Svg is valid
    pub svg_is_valid: bool,
}

const BASE_SVG: &str = include_str!("../assets/galago.svg");

impl Default for GalagoApp {
    fn default() -> Self {
        Self {
            svg: BASE_SVG.to_string(),
            saved_svg: BASE_SVG.to_string(),
            scene_rect: egui::Rect::NAN,
            tree_viewer: TreeViewer::new(),
            string_viewer: StringViewer::new(),
            grid: Grid::default(),
            svg_render: SvgRender::new(),
            should_reset_view: false,
            save_path: None,
            svg_is_valid: true,
        }
    }
}

impl Debug for GalagoApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_fmt = f.debug_struct("GalagoApp");
        debug_fmt.finish()
    }
}
impl GalagoApp {
    /// New GalagoApp
    fn new_app(cc: &eframe::CreationContext<'_>) -> Self {
        bladvak::utils::get_saved_app_state::<Self>(cc)
    }
    /// Create a new Galago App with an svg
    pub fn new_app_with_svg(cc: &eframe::CreationContext<'_>, svg: String) -> Self {
        let mut app = Self::new_app(cc);
        app.saved_svg = svg.clone();
        app.svg = svg;
        app
    }
}

impl BladvakApp for GalagoApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Result<Self, AppError> {
        Ok(Self::new_app(cc))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn new_with_args(cc: &eframe::CreationContext<'_>, args: &[String]) -> Result<Self, AppError> {
        if args.len() > 1 {
            use std::fs;

            let path = &args[1];
            match fs::read_to_string(path) {
                Ok(svg) => Ok(Self::new_app_with_svg(cc, svg)),
                Err(e) => {
                    eprintln!("Failed to load svg '{path}': {e}");
                    Err((format!("Failed to load svg '{path}')"), e).into())
                }
            }
        } else {
            Ok(Self::new_app(cc))
        }
    }

    fn top_panel(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        self.app_top_panel(ui, error_manager)
    }
    fn settings(&mut self, ui: &mut egui::Ui, _error_manager: &mut bladvak::ErrorManager) {
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!("{} settings", self.svg_render.title()));
            if ui.button("⟳").clicked() {
                self.svg_render = SvgRender::new();
            }
        });
        self.svg_render.show_settings(ui);

        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!("{} settings", self.tree_viewer.title()));
            ui.button("⟳").clicked().then(|| {
                self.tree_viewer = TreeViewer::new();
            });
        });
        self.tree_viewer.show_settings(ui);

        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!("{} settings", self.string_viewer.title()));
            ui.button("⟳").clicked().then(|| {
                self.string_viewer = StringViewer::new();
            });
        });
        self.string_viewer.show_settings(ui);

        ui.separator();
        ui.horizontal(|ui| {
            ui.label(format!("{} settings", self.grid.title()));
            ui.button("⟳").clicked().then(|| {
                self.grid = Grid::default();
            });
        });
        self.grid.show_settings(ui);

        ui.separator();
        if ui.button("Default svg").clicked() {
            self.saved_svg = BASE_SVG.to_string();
            self.svg = BASE_SVG.to_string();
        }
    }

    fn name() -> String {
        env!("CARGO_PKG_NAME").to_string()
    }

    fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn repo_url() -> String {
        "https://github.com/Its-Just-Nans/galago".to_string()
    }

    fn central_panel(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        self.app_central_panel(ui, error_manager)
    }

    fn handle_file(&mut self, bytes: &[u8]) -> Result<(), AppError> {
        match String::from_utf8(bytes.to_vec()) {
            Ok(svg_str) => {
                self.saved_svg = svg_str.clone();
                self.svg = svg_str;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
    /// Check if the sidebar is needed
    fn is_side_panel(&self) -> bool {
        !self.tree_viewer.is_windows || !self.string_viewer.is_windows
    }

    fn side_panel(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        self.app_side_panel(ui, error_manager);
    }

    fn is_open_button(&self) -> bool {
        true
    }

    fn menu_file(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        if ui.button("Save").clicked() {
            ui.close();
            let save_path = bladvak::utils::get_save_path(Some(PathBuf::from("file.svg")));
            match save_path {
                Ok(save_p) => {
                    self.save_path = save_p.clone();
                    if let Some(path_to_save) = save_p
                        && let Err(err) = self.save_svg(&path_to_save)
                    {
                        error_manager.add_error(err);
                    }
                }
                Err(e) => {
                    error_manager.add_error(e);
                }
            }
        }
    }
}

impl GalagoApp {
    /// Side panel
    fn app_side_panel(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        if !self.string_viewer.is_windows {
            self.string_viewer
                .show(ui, &mut self.svg, self.svg_is_valid, error_manager);
        }
        if !self.tree_viewer.is_windows {
            if !self.string_viewer.is_windows {
                ui.separator();
            }
            self.tree_viewer.show(ui, &mut self.svg, error_manager);
        }
    }
}
