//! SVG editor

mod path;
mod rect;

use bladvak::{ErrorManager, app::BladvakPanel, eframe::egui};
use resvg::usvg::Options;

use crate::GalagoApp;
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
}

impl InnerShape {
    /// show the shape
    #[allow(clippy::too_many_lines)]
    fn show_shape(&mut self, ui: &mut egui::Ui, idx: usize, error_manager: &mut ErrorManager) {
        let size_point = 5.0;
        match self {
            Self::Rect(shape) => {
                shape.show(ui, idx, size_point, error_manager);
            }
            Self::Path(shape) => {
                shape.show(ui, idx, size_point, error_manager);
            }
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
            "p" => {
                // TODO
                // let rect = p.abs_bounding_box();
                // let min = Pos2::new(rect.left(), rect.top());
                // let max = Pos2::new(rect.right(), rect.bottom());
                // let shape = Shape {
                //     inner: InnerShape::Rect(Rect::from_min_max(min, max)),
                //     is_visible: true,
                // };
                // rects.push(shape);
            }
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
                    continue;
                }
            },
            "polygon" => {
                // TODO
                // let rect = p.abs_bounding_box();
                // let min = Pos2::new(rect.left(), rect.top());
                // let max = Pos2::new(rect.right(), rect.bottom());
            }
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
        if ui.button("Add path").clicked() {
            let shape = Shape {
                inner: InnerShape::Path(SvgPath::default()),
                is_visible: true,
            };
            self.shapes.push(shape);
        }
        let mut to_remove = None;
        for (idx, shape) in self.shapes.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("{shape:?}"));
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
