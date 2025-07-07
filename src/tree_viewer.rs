use egui::Ui;
use egui_extras::{Column, TableBuilder};
use xmltree::Element;

use crate::{
    path::{get_letter, to_string, PathParsed},
    settings::TreeViewerSettings,
};

fn show_group(ui: &mut Ui, nodes: &mut [xmltree::XMLNode]) {
    for (idx, node) in nodes.iter_mut().enumerate() {
        match node {
            xmltree::XMLNode::Element(g) => match g.name.as_str() {
                "g" => {
                    egui::CollapsingHeader::new("Group")
                        .id_salt(format!("group_{}", idx))
                        .show(ui, |ui| {
                            show_group(ui, &mut g.children);
                        });
                }
                e => {
                    egui::CollapsingHeader::new(e)
                        .id_salt(format!("element_{}", idx))
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
                            if e == "path" {
                                if let Some(path) = g.attributes.get("d") {
                                    TableBuilder::new(ui)
                                        .id_salt(format!("element_path_{}", idx))
                                        .column(Column::auto())
                                        .column(Column::remainder())
                                        .header(20.0, |mut header| {
                                            header.col(|ui| {
                                                ui.heading("type");
                                            });
                                            header.col(|ui| {
                                                ui.heading("value");
                                            });
                                        })
                                        .body(|mut body| {
                                            let path =
                                                PathParsed::from("M10-20l30.1.5.1-20z").unwrap();
                                            let segments = path.segments();
                                            for path_segment in segments {
                                                let letter = get_letter(path_segment);
                                                let mut value =
                                                    to_string(path_segment)[1..].to_string();
                                                body.row(30.0, |mut row| {
                                                    row.col(|ui| {
                                                        ui.label(letter);
                                                    });
                                                    row.col(|ui| {
                                                        ui.text_edit_singleline(&mut value);
                                                    });
                                                });
                                            }
                                        });
                                }
                            }
                        });
                }
            },
            xmltree::XMLNode::Text(t) => {
                egui::CollapsingHeader::new("Path")
                    .id_salt(format!("text_{}", idx))
                    .show(ui, |ui| {
                        ui.label(format!("Text: {}", t));
                    });
            }
            _ => {
                ui.label(format!("Unknown node: {:?}", node));
            }
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct TreeViewer {}

impl TreeViewer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn title(&self) -> &'static str {
        "SVG Tree"
    }

    pub fn show(&self, ui: &mut Ui, svg_str: &mut String, _settings: &TreeViewerSettings) {
        egui::ScrollArea::vertical().id_salt("tree_viewer").show(
            ui,
            |ui| match &mut Element::parse(svg_str.as_bytes()) {
                Ok(e) => {
                    show_group(ui, &mut e.children);
                    let mut buf = Vec::new();
                    e.write(&mut buf).unwrap();
                    let s = String::from_utf8(buf).unwrap();
                    *svg_str = s;
                }
                Err(e) => {
                    ui.label(format!("Error: {:?}", e));
                }
            },
        );
    }
}
