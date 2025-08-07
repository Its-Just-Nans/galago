//! Galago app

use egui::{Pos2, Rect, Window};

use crate::{
    errors::ErrorManager, file_handler::FileHandler, grid::Grid, settings::Settings,
    string_viewer::StringViewer, svg_render::SvgRender, tree_viewer::TreeViewer,
};
use egui::ThemePreference;

/// GalagoApp struct
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GalagoApp {
    /// Save SVG
    base_svg: String,

    /// SVG Screen
    svg: String,

    /// Current scene zoom
    #[serde(skip)]
    scene_rect: egui::Rect,

    /// Settings Ui
    settings: Settings,

    /// TreeViewer Ui
    tree_viewer: TreeViewer,

    /// StringViewer Ui
    string_viewer: StringViewer,

    /// Grid options
    grid: Grid,

    /// SvgRender
    svg_render: SvgRender,

    /// should reset the view
    should_reset_view: bool,

    /// Error_manager
    #[serde(skip)]
    pub error_manager: ErrorManager,

    /// File handler
    file_handler: FileHandler,
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
            });
        });

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
            Ok(Some(img)) => {
                self.base_svg = img.clone();
                self.svg = img;
            }
            Ok(None) => {}
            Err(err) => {
                self.error_manager.add_error(err);
            }
        }
        self.error_manager.show(ctx);
        self.settings.show(ctx, |ui| {
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
                self.svg = BASE_SVG.to_owned();
            }
        });
    }
}
