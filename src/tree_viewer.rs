//! Tree Viewer

use egui::{Ui, Window};
use egui_extras::{Column, TableBuilder};
use xmltree::Element;

use crate::path::PathParsed;

/// TreeViewer Struct
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct TreeViewer {
    /// is windows mode
    pub is_windows: bool,

    /// Index of the group to edit
    #[serde(skip)]
    pub ref_group: Option<usize>,
}

impl TreeViewer {
    /// Create a new Tree Viewer
    pub fn new() -> Self {
        Default::default()
    }

    /// Tree Viewer title
    pub fn title(&self) -> &'static str {
        "SVG Tree"
    }

    /// Settings
    pub fn show_settings(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.is_windows, "Tree as windows");
    }

    /// Show Tree Viewer
    pub fn show(&mut self, ui: &mut Ui, svg_str: &mut String) {
        egui::ScrollArea::vertical().id_salt("tree_viewer").show(
            ui,
            |ui| match &mut Element::parse(svg_str.as_bytes()) {
                Ok(e) => {
                    self.show_group(ui, &mut e.children, None);
                    let mut buf = Vec::new();
                    if e.write(&mut buf).is_ok() {
                        if let Ok(s) = String::from_utf8(buf) {
                            *svg_str = s;
                        }
                    };
                }
                Err(e) => {
                    ui.label(format!("Error: {e:?}"));
                }
            },
        );
    }

    /// Show the svg groups
    fn show_group(&mut self, ui: &mut Ui, nodes: &mut [xmltree::XMLNode], curr_idx: Option<usize>) {
        let current_idx = curr_idx.unwrap_or(0);
        for (idx, node) in nodes.iter_mut().enumerate() {
            match node {
                xmltree::XMLNode::Element(g) => match g.name.clone().as_str() {
                    "g" => {
                        egui::CollapsingHeader::new("Group")
                            .id_salt(format!("group_{idx}"))
                            .show(ui, |ui| {
                                self.show_group(ui, &mut g.children, Some(current_idx + idx));
                            });
                    }
                    e => {
                        egui::CollapsingHeader::new(format!("Element: {e}"))
                            .id_salt(format!("element_{idx}"))
                            .show(ui, |ui| {
                                TableBuilder::new(ui)
                                    .column(Column::auto())
                                    .column(Column::remainder())
                                    .header(20.0, |mut header| {
                                        header.col(|ui| {
                                            ui.heading("attribute");
                                        });
                                        header.col(|ui| {
                                            ui.heading("value");
                                            if ui.button("edit").clicked() {
                                                self.ref_group = Some(idx);
                                            }
                                        });
                                    })
                                    .body(|mut body| {
                                        for (key, value) in g.attributes.iter_mut() {
                                            body.row(0.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.label(key);
                                                });
                                                row.col(|ui| {
                                                    ui.text_edit_multiline(value);
                                                });
                                            });
                                        }
                                    });
                            });
                        if let Some(index) = self.ref_group {
                            if *e == *"path" && index == idx {
                                self.show_current_edition(ui.ctx(), g);
                            }
                        }
                    }
                },
                xmltree::XMLNode::Text(t) => {
                    egui::CollapsingHeader::new("Path")
                        .id_salt(format!("text_{idx}"))
                        .show(ui, |ui| {
                            ui.label(format!("Text: {t}"));
                        });
                }
                _ => {
                    ui.label(format!("Unknown node: {node:?}"));
                }
            }
        }
    }

    /// Show current edition of the path
    fn show_current_edition(&mut self, ctx: &egui::Context, g: &mut Element) {
        let mut is_open = self.ref_group.is_some();
        Window::new("Edition")
            .resizable(true)
            .open(&mut is_open)
            .min_width(500.0)
            .min_height(100.0)
            .show(ctx, |ui| {
                if let Some(path) = g.attributes.get_mut("d") {
                    let mut parsed_path = PathParsed::from(path).unwrap();
                    if ui
                        .button("Convert to relative")
                        .on_hover_text("Convert path to relative coordinates")
                        .clicked()
                    {
                        parsed_path = parsed_path.to_relative();
                        *path = parsed_path.path_data();
                    }
                    if ui
                        .button("Convert to absolute")
                        .on_hover_text("Convert path to absolute coordinates")
                        .clicked()
                    {
                        parsed_path = parsed_path.to_absolute();
                        *path = parsed_path.path_data();
                    }
                    egui::ScrollArea::vertical()
                        .id_salt("string_viewer")
                        .show(ui, |ui| {
                            let segments = parsed_path.segments();
                            let mut idx_to_update = None;
                            for (idx, path_segment) in segments.iter_mut().enumerate() {
                                ui.horizontal_wrapped(|ui| {
                                    let letter = path_segment.get_letter();

                                    if ui.button(&letter).clicked() {
                                        idx_to_update = Some(idx);
                                    }
                                    let value = path_segment.value();

                                    // to do update
                                    ui.text_edit_singleline(value);
                                });
                            }
                            if let Some(idx) = idx_to_update {
                                parsed_path.toggle_segment(idx);
                            }
                            *path = parsed_path.path_data();
                        });
                } else {
                    ui.label("No path data found in the selected element.");
                }
            });
        if !is_open {
            self.ref_group = None; // Reset the reference group when the edition window is closed
        }
    }
}
