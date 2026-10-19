//! SVG editor

mod path;
mod polygon;
mod rect;

use bladvak::{
    ErrorManager,
    app::BladvakPanel,
    eframe::egui::{self, CollapsingHeader},
};
use resvg::usvg::Options;

use crate::{GalagoApp, editor::polygon::SvgPolygon};
use path::SvgPath;
use rect::SvgRect;

/// Data
#[derive(serde::Deserialize, serde::Serialize, Default, Debug, Clone)]
pub struct ShapeEditor {
    /// shapes
    shapes: Vec<Shape>,
}

/// Shape
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Shape {
    /// inner
    inner: InnerShape,
    /// visibility
    is_visible: bool,
}

/// Shape type
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
enum InnerShape {
    /// rect
    Rect(SvgRect),
    /// path
    Path(SvgPath),
    /// polygon
    Polygon(SvgPolygon),
}

impl InnerShape {
    /// show the shape
    fn show_shape(&mut self, ui: &mut egui::Ui, idx: usize, error_manager: &mut ErrorManager) {
        let size_point = 5.0;
        match self {
            Self::Rect(shape) => {
                shape.show(ui, idx, size_point, error_manager);
            }
            Self::Path(shape) => {
                shape.show(ui, idx, size_point, error_manager);
            }
            Self::Polygon(shape) => {
                shape.show(ui, idx, size_point, error_manager);
            }
        }
    }
    /// name
    fn name(&self) -> &str {
        match self {
            Self::Rect(_) => "rect",
            Self::Path(_) => "path",
            Self::Polygon(_) => "polygon",
        }
    }
}

/// recursivily add shape
fn recursive_add(
    rects: &mut Vec<Shape>,
    group: &resvg::usvg::roxmltree::Node<'_, '_>,
    error_manager: &mut ErrorManager,
) {
    for one_node in group.children() {
        match one_node.tag_name().name() {
            "path" => match SvgPath::try_parse(one_node) {
                Ok(s) => {
                    let shape = Shape {
                        inner: InnerShape::Path(s),
                        is_visible: true,
                    };
                    rects.push(shape);
                }
                Err(err) => {
                    error_manager.add_error(err);
                }
            },
            "rect" => match SvgRect::try_parse(one_node) {
                Ok(s) => {
                    let shape = Shape {
                        inner: InnerShape::Rect(s),
                        is_visible: true,
                    };
                    rects.push(shape);
                }
                Err(err) => {
                    error_manager.add_error(err);
                }
            },
            "polygon" => match SvgPolygon::try_parse(one_node) {
                Ok(s) => {
                    let shape = Shape {
                        inner: InnerShape::Polygon(s),
                        is_visible: true,
                    };
                    rects.push(shape);
                }
                Err(err) => {
                    error_manager.add_error(err);
                }
            },
            "g" | "svg" => {
                recursive_add(rects, &one_node, error_manager);
            }
            _ => {}
        }
    }
}

impl ShapeEditor {
    /// Show ui
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        svg: &str,
        _usvg_options: &Options<'_>, // TODO
        error_manager: &mut ErrorManager,
    ) {
        if ui.button("Convert to shape").clicked() {
            let xml_opt = resvg::usvg::roxmltree::ParsingOptions {
                allow_dtd: true,
                ..Default::default()
            };

            match resvg::usvg::roxmltree::Document::parse_with_options(svg, xml_opt) {
                Ok(tree) => {
                    recursive_add(&mut self.shapes, &tree.root(), error_manager);
                }
                Err(_e) => {
                    error_manager.add_error("Cannot convert the svg");
                }
            }
        }
        let mut to_remove = None;
        for (idx, shape) in self.shapes.iter().enumerate() {
            ui.horizontal(|ui| {
                CollapsingHeader::new(shape.inner.name())
                    .id_salt(("collapsing", shape.inner.name(), idx))
                    .show(ui, |ui| {
                        ui.label(format!("{shape:#?}"));
                    });
                if ui.button("Delete").clicked() {
                    to_remove = Some(idx);
                }
            });
        }
        if let Some(idx) = to_remove {
            self.shapes.remove(idx);
        }
    }

    /// show in scene
    pub(crate) fn show_in_scene(&mut self, ui: &mut egui::Ui, error_manager: &mut ErrorManager) {
        for (idx, shape) in self.shapes.iter_mut().enumerate() {
            if shape.is_visible {
                shape.inner.show_shape(ui, idx, error_manager);
            }
        }
    }
}

/// Panel
#[derive(Debug)]
pub struct ShapeEditorPanel;

impl BladvakPanel for ShapeEditorPanel {
    type App = GalagoApp;

    fn name(&self) -> &'static str {
        "Shape editor"
    }

    fn has_settings(&self) -> bool {
        false
    }

    fn ui_settings(
        &self,
        _app: &mut Self::App,
        _ui: &mut egui::Ui,
        _error_manager: &mut ErrorManager,
    ) {
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&self, app: &mut Self::App, ui: &mut egui::Ui, error_manager: &mut ErrorManager) {
        let Some(document) = app.documents.get_current_doc_mut() else {
            return;
        };
        document
            .shape_editor
            .show(ui, &document.svg, &app.usvg_options, error_manager);
    }
}
