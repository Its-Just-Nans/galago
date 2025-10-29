//! Galago app

use std::path::PathBuf;

use egui::{Pos2, Rect, Window};

use crate::{
    errors::ErrorManager, file_handler::FileHandler, grid::Grid, settings::Settings,
    string_viewer::StringViewer, svg_render::SvgRender, tree_viewer::TreeViewer,
};

/// GalagoApp struct
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GalagoApp {
    /// Save SVG
    base_svg: String,

    /// SVG Screen
    pub(crate) svg: String,

    /// Current scene zoom
    #[serde(skip)]
    scene_rect: egui::Rect,

    /// Settings Ui
    pub(crate) settings: Settings,

    /// TreeViewer Ui
    tree_viewer: TreeViewer,

    /// StringViewer Ui
    string_viewer: StringViewer,

    /// Grid options
    grid: Grid,

    /// SvgRender
    svg_render: SvgRender,

    /// should reset the view
    pub(crate) should_reset_view: bool,

    /// Error_manager
    #[serde(skip)]
    pub error_manager: ErrorManager,

    /// File handler
    pub(crate) file_handler: FileHandler,

    /// Debug and inspection toggle
    #[serde(skip)]
    show_inspection: bool,

    /// Path to save the svg
    pub save_path: Option<PathBuf>,
}

const BASE_SVG: &str = include_str!("../assets/galago.svg");

impl Default for GalagoApp {
    fn default() -> Self {
        Self {
            svg: BASE_SVG.to_string(),
            base_svg: BASE_SVG.to_string(),
            scene_rect: egui::Rect::NAN,
            settings: Settings::default(),
            tree_viewer: TreeViewer::new(),
            string_viewer: StringViewer::new(),
            grid: Grid::default(),
            svg_render: SvgRender::new(),
            should_reset_view: false,
            file_handler: Default::default(),
            error_manager: Default::default(),
            save_path: None,
            show_inspection: false,
        }
    }
}

impl GalagoApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    /// Create a new Galago App with an svg
    pub fn new_with_svg(cc: &eframe::CreationContext<'_>, svg: String) -> Self {
        let mut app = Self::new(cc);
        app.base_svg = svg.clone();
        app.svg = svg;
        app
    }

    /// Check if the sidebar is needed
    fn is_sidebar(&self) -> bool {
        !self.tree_viewer.is_windows || !self.string_viewer.is_windows
    }
}

impl eframe::App for GalagoApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);

        let color = self.svg_render.update(ctx, &self.svg).is_ok();

        if self.string_viewer.is_windows {
            let mut current_open = true;
            Window::new(self.string_viewer.title())
                .min_width(500.0)
                .min_height(100.0)
                .open(&mut current_open)
                .resizable(true)
                .show(ctx, |ui| {
                    self.string_viewer.show(ui, &mut self.svg, color);
                });
            self.string_viewer.is_windows = current_open;
        }
        if self.tree_viewer.is_windows {
            let mut current_open = true;

            Window::new(self.tree_viewer.title())
                .resizable(true)
                .min_width(500.0)
                .min_height(100.0)
                .open(&mut current_open)
                .show(ctx, |ui| {
                    self.tree_viewer.show(ui, &mut self.svg);
                });
            self.tree_viewer.is_windows = current_open;
        }
        if self.show_inspection {
            egui::Window::new("Inspection")
                .open(&mut self.show_inspection)
                .vscroll(true)
                .show(ctx, |ui| {
                    ctx.inspection_ui(ui);
                });
        }
        if self.is_sidebar() {
            egui::panel::SidePanel::right("conf_panel")
                .frame(
                    egui::Frame::central_panel(&ctx.style())
                        .inner_margin(0)
                        .outer_margin(0),
                )
                .min_width(self.settings.min_width_sidebar)
                .show(ctx, |ui_sidebar| {
                    egui::ScrollArea::vertical()
                        .id_salt("right_sidebar")
                        .show(ui_sidebar, |ui| {
                            if !self.string_viewer.is_windows {
                                self.string_viewer.show(ui, &mut self.svg, color);
                            }
                            if !self.tree_viewer.is_windows {
                                if !self.string_viewer.is_windows {
                                    ui.separator();
                                }
                                self.tree_viewer.show(ui, &mut self.svg);
                            }
                        });
                });
        }
        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(0)
                    .outer_margin(0),
            )
            .show(ctx, |parent_ui| {
                let rect = parent_ui.available_rect_before_wrap();
                let response = egui::Scene::new()
                    .max_inner_size([350.0, 1000.0])
                    .zoom_range(0.1..=50.0)
                    .show(parent_ui, &mut self.scene_rect, |ui| {
                        let painter = ui.painter();
                        let bg_r: egui::Response = ui.response();
                        if bg_r.rect.is_finite() {
                            self.grid.draw(&bg_r.rect, painter);
                        }
                        let _response = self.svg_render.show(ui);
                        // if response.clicked() {
                        //     println!("SVG clicked!");
                        // }
                    })
                    .response;

                if self.should_reset_view || response.double_clicked() {
                    let real_rect = Rect::from_two_pos(Pos2::ZERO, (rect.max - rect.min).to_pos2());
                    self.scene_rect = real_rect;
                }
            });
        match self.file_handler.handle_files(ctx) {
            Ok(Some(svg_str)) => {
                self.base_svg = svg_str.clone();
                self.svg = svg_str;
            }
            Ok(None) => {}
            Err(err) => {
                self.error_manager.add_error(err);
            }
        }
        self.error_manager.show(ctx);
        self.settings.show(ctx, |ui| {
            ui.checkbox(&mut self.show_inspection, "Debug panel");

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
            ui.horizontal(|ui| {
                ui.label(format!("{} settings", self.error_manager.title()));
                ui.button("⟳").clicked().then(|| {
                    self.error_manager = Default::default();
                });
            });
            self.error_manager.show_settings(ui);

            ui.separator();
            if ui.button("Default svg").clicked() {
                self.svg = BASE_SVG.to_owned();
            }
        });
    }
}
