//! Galago app

use bladvak::eframe::egui::{self};
use bladvak::utils::grid::Grid;
use bladvak::utils::is_native;
use bladvak::{AppError, BladvakApp, ErrorManager, File, eframe, utils::Documents};
use resvg::usvg;
use std::fmt::Debug;
use std::path::PathBuf;

use crate::document::Document;
use crate::settings::AppSettings;
use crate::string_viewer::StringViewerPanel;
use crate::svg_render::SvgViewerPanel;
use crate::tree_viewer::TreeViewerPanel;
use crate::{string_viewer::StringViewer, tree_viewer::TreeViewer};

/// `GalagoApp` struct
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GalagoApp {
    /// documents
    pub(crate) documents: Documents<Document>,
    /// settings
    pub(crate) settings: AppSettings,
    /// `TreeViewer` Ui
    pub(crate) tree_viewer: TreeViewer,
    /// `StringViewer` Ui
    pub(crate) string_viewer: StringViewer,
    /// Grid options
    pub(crate) grid: Grid,
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
        let mut documents = Documents::default();
        documents.push(Self::default_document());
        Self {
            documents,
            settings: AppSettings::default(),
            tree_viewer: TreeViewer::default(),
            string_viewer: StringViewer::default(),
            grid: Grid::default(),
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

    /// Default document
    pub(crate) fn default_document() -> Document {
        Document {
            svg: Self::BASE_SVG.to_string(),
            saved_svg: Self::BASE_SVG.to_string(),
            scene_rect: egui::Rect::NAN,
            svg_is_valid: true,
            filename: PathBuf::from("galago.svg"),
            ..Default::default()
        }
    }

    /// Handle file a string
    pub(crate) fn handle_file_string(&mut self, filename: PathBuf, svg_str: String) {
        let document = Document {
            svg: svg_str.clone(),
            saved_svg: svg_str,
            filename,
            ..Default::default()
        };
        self.documents.push(document);
    }
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
        _error_manager: &mut ErrorManager,
    ) -> Result<Self, AppError> {
        let mut app = saved_state;
        if is_native() && args.len() > 1 {
            use std::fs;
            // do not clear the documents since we save them
            // app.documents.clear();
            for one_path in &args[1..] {
                let absolute_path = fs::canonicalize(one_path)?;
                match fs::read_to_string(&absolute_path) {
                    Ok(svg) => {
                        let document = Document {
                            saved_svg: svg.clone(),
                            svg,
                            filename: absolute_path,
                            ..Default::default()
                        };
                        app.documents.push(document);
                    }
                    Err(e) => {
                        eprintln!("Failed to load svg '{}': {e}", absolute_path.display());
                        return Err((
                            format!("Failed to load svg '{}')", absolute_path.display()),
                            e,
                        )
                            .into());
                    }
                }
            }
            Ok(app)
        } else {
            if !app.documents.is_some() {
                app.documents.push(Self::default_document());
            }
            Ok(app)
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
            for one_document in &mut self.documents {
                one_document.svg_render.stale_render();
            }
            Ok(())
        } else {
            match String::from_utf8(file.data) {
                Ok(svg_str) => {
                    self.handle_file_string(file.path, svg_str);
                    Ok(())
                }
                Err(e) => Err(e.into()),
            }
        }
    }

    /// Check if the sidebar is needed
    fn is_side_panel(&self) -> bool {
        self.documents.is_some()
    }

    fn side_panel(&mut self, ui: &mut egui::Ui, func_ui: impl FnOnce(&mut egui::Ui, &mut Self)) {
        func_ui(ui, self);
    }
    fn window_panel_frame(&mut self, ui: &mut egui::Ui) -> egui::Frame {
        egui::Frame::window(&ui.ctx().global_style()).inner_margin(0.0)
    }

    fn is_open_button(&self) -> bool {
        true
    }

    fn menu_file(&mut self, ui: &mut egui::Ui, error_manager: &mut bladvak::ErrorManager) {
        if self.documents.is_some() && ui.button("Save").clicked() {
            ui.close();
            let Some(document) = self.documents.get_current_doc_mut() else {
                error_manager.add_error("No document to save");
                return;
            };

            let current_save_path = document.filename.clone();
            let save_path = bladvak::utils::get_save_path(Some(&current_save_path));
            match save_path {
                Ok(save_p) => {
                    if let Some(path_to_save) = save_p {
                        document.filename.clone_from(&path_to_save);
                        if let Err(err) = self.save_svg(&path_to_save) {
                            error_manager.add_error(err);
                        }
                    }
                }
                Err(e) => {
                    error_manager.add_error(e);
                }
            }
        }
    }
}
