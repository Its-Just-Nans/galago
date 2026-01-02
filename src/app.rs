//! Galago app

use bladvak::eframe::egui::{self};
use bladvak::utils::grid::Grid;
use bladvak::utils::is_native;
use bladvak::{AppError, BladvakApp, File, eframe};
use resvg::usvg;
use std::fmt::Debug;
use std::path::PathBuf;

use crate::string_viewer::StringViewerPanel;
use crate::svg_render::SvgViewerPanel;
use crate::tree_viewer::TreeViewerPanel;
use crate::{string_viewer::StringViewer, svg_render::SvgRender, tree_viewer::TreeViewer};

/// `GalagoApp` struct
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

    /// `TreeViewer` Ui
    pub(crate) tree_viewer: TreeViewer,

    /// `StringViewer` Ui
    pub(crate) string_viewer: StringViewer,

    /// Grid options
    pub(crate) grid: Grid,

    /// `SvgRender`
    pub(crate) svg_render: SvgRender,

    /// should reset the view
    pub(crate) should_reset_view: bool,

    /// Path to save the svg
    pub(crate) save_path: Option<PathBuf>,

    /// Svg is valid
    pub(crate) svg_is_valid: bool,

    /// usvg options
    #[serde(skip)]
    pub(crate) usvg_options: usvg::Options<'static>,
}

impl Default for GalagoApp {
    fn default() -> Self {
        let mut usvg_options = usvg::Options::default();
        // sadly, no wasm support
        // see https://github.com/RazrFalcon/fontdb/issues/83
        // or maybe do https://github.com/RazrFalcon/fontdb/issues/83#issuecomment-3677330841
        usvg_options.fontdb_mut().load_system_fonts();
        Self {
            svg: Self::BASE_SVG.to_string(),
            saved_svg: Self::BASE_SVG.to_string(),
            scene_rect: egui::Rect::NAN,
            tree_viewer: TreeViewer::default(),
            string_viewer: StringViewer::default(),
            grid: Grid::default(),
            svg_render: SvgRender::default(),
            should_reset_view: false,
            save_path: None,
            svg_is_valid: true,
            usvg_options,
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
    /// Base default svg (galago logo)
    pub(crate) const BASE_SVG: &str = include_str!("../assets/galago.svg");
}

impl BladvakApp<'_> for GalagoApp {
    fn panel_list(&self) -> Vec<Box<dyn bladvak::app::BladvakPanel<App = Self>>> {
        vec![
            Box::new(StringViewerPanel),
            Box::new(TreeViewerPanel),
            Box::new(SvgViewerPanel),
        ]
    }

    fn try_new_with_args(
        saved_state: Self,
        _cc: &eframe::CreationContext<'_>,
        args: &[String],
    ) -> Result<Self, AppError> {
        if is_native() && args.len() > 1 {
            use std::fs;

            let path = &args[1];
            match fs::read_to_string(path) {
                Ok(svg) => {
                    let mut app = saved_state;
                    app.saved_svg.clone_from(&svg);
                    app.svg = svg;
                    Ok(app)
                }
                Err(e) => {
                    eprintln!("Failed to load svg '{path}': {e}");
                    Err((format!("Failed to load svg '{path}')"), e).into())
                }
            }
        } else {
            Ok(saved_state)
        }
    }

    fn top_panel(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        self.app_top_panel(ui, error_manager);
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

    fn icon() -> &'static [u8] {
        &include_bytes!("../assets/icon-256.png")[..]
    }

    fn central_panel(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        self.app_central_panel(ui, error_manager);
    }

    fn handle_file(&mut self, file: File) -> Result<(), AppError> {
        let most_likely_font = file
            .path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| {
                matches!(
                    e.to_ascii_lowercase().as_str(),
                    "ttf" | "otf" | "woff" | "woff2" | "eot" | "ttc"
                )
            });
        if most_likely_font {
            self.usvg_options.fontdb_mut().load_font_data(file.data);
            self.svg_render.stale_render();
            Ok(())
        } else {
            match String::from_utf8(file.data) {
                Ok(svg_str) => {
                    self.saved_svg = svg_str.clone();
                    self.svg = svg_str;
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
        }
    }
    /// Check if the sidebar is needed
    fn is_side_panel(&self) -> bool {
        true
    }

    fn side_panel(&mut self, ui: &mut egui::Ui, func_ui: impl FnOnce(&mut egui::Ui, &mut Self)) {
        func_ui(ui, self);
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
                    self.save_path.clone_from(&save_p);
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
