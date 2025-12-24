//! Tree Viewer

use bladvak::ErrorManager;
use bladvak::app::BladvakPanel;
use bladvak::eframe::egui::{self, Color32, DragValue, Frame, Ui, Window};
use bladvak::egui_extras::{Column, TableBuilder};
use std::collections::HashMap;
use svgtypes::PathSegment;
use xmltree::{Element, EmitterConfig};

use crate::GalagoApp;
use crate::path::{
    SvgPath, circle_to_path, ellipse_to_path, line_to_path, polygon_to_path, polyline_to_path,
    rect_to_path,
};

/// TreeViewer Struct
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TreeViewer {
    /// Is multi line
    is_multi_line: bool,

    /// Is editable
    is_editable: bool,

    /// Edit as inputs
    edit_path_as_input: bool,

    /// Index of the group to edit
    #[serde(skip)]
    ref_group: Option<usize>,

    /// Translate x
    translate_x: f64,
    /// Translate y
    translate_y: f64,
    /// Scale x
    scale_x: f64,
    /// Scale y
    scale_y: f64,
    /// Rotate x
    rotate_x: f64,
    /// Rotate y
    rotate_y: f64,
    /// Rotate angle
    rotate: f64,
    /// Round to value
    round_to: u64,
    /// Name of the tew element to add
    new_element_name: String,

    /// Attributes of the tree viewer
    #[serde(skip)]
    attributes_temp: HashMap<usize, String>,
}

impl Default for TreeViewer {
    fn default() -> Self {
        Self {
            is_multi_line: true,
            is_editable: false,
            edit_path_as_input: false,
            ref_group: None,
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotate_x: 0.0,
            rotate_y: 0.0,
            rotate: 0.0,
            round_to: 1,
            attributes_temp: HashMap::new(),
            new_element_name: String::new(),
        }
    }
}

impl TreeViewer {
    /// Show Tree Viewer
    pub fn show(&mut self, ui: &mut Ui, svg_str: &mut String, error_manager: &mut ErrorManager) {
        Frame::new()
            .inner_margin(egui::Margin {
                left: 5,
                right: 0, // scrollbar touches the edge
                top: 5,
                bottom: 5,
            })
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("tree_viewer")
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.checkbox(&mut self.is_editable, "Editable (auto-write)");
                        match &mut Element::parse(svg_str.as_bytes()) {
                            Ok(e) => {
                                // edit width and height and viewbox
                                ui.collapsing("SVG", |ui| {
                                    for (key, value) in e.attributes.iter_mut() {
                                        ui.horizontal(|ui| {
                                            ui.label(key);
                                            ui.text_edit_singleline(value);
                                        });
                                    }
                                });

                                self.show_group(
                                    ui,
                                    &mut e.children,
                                    None,
                                    error_manager,
                                    self.is_editable,
                                );
                                ui.add_enabled_ui(self.is_editable, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.text_edit_singleline(&mut self.new_element_name);
                                        if ui
                                            .button("Add key")
                                            .on_hover_text("Add new attribute")
                                            .clicked()
                                            && !self.new_element_name.is_empty()
                                        {
                                            e.children.push(xmltree::XMLNode::Element(
                                                Element::new(&self.new_element_name),
                                            ));
                                            self.new_element_name = Default::default();
                                        }
                                    });
                                });

                                if self.is_editable {
                                    let mut buf = Vec::new();
                                    let writer_config = EmitterConfig {
                                        perform_indent: true,
                                        ..EmitterConfig::new()
                                    };
                                    if e.write_with_config(&mut buf, writer_config).is_ok()
                                        && let Ok(s) = String::from_utf8(buf)
                                    {
                                        *svg_str = s;
                                    };
                                }
                            }
                            Err(e) => {
                                ui.label(format!("Error: {e}"));
                            }
                        }
                    });
            });
    }

    /// Show the svg groups
    fn show_group(
        &mut self,
        ui: &mut Ui,
        nodes: &mut [xmltree::XMLNode],
        curr_idx: Option<usize>,
        error_manager: &mut ErrorManager,
        is_editable: bool,
    ) {
        let current_idx = curr_idx.unwrap_or(0);
        for (idx, node) in nodes.iter_mut().enumerate() {
            match node {
                xmltree::XMLNode::Element(g) => match g.name.clone().as_str() {
                    "g" => {
                        egui::CollapsingHeader::new("Group")
                            .id_salt(format!("group_{idx}"))
                            .show(ui, |ui| {
                                self.show_group(
                                    ui,
                                    &mut g.children,
                                    Some(current_idx + idx),
                                    error_manager,
                                    is_editable,
                                );
                            });
                    }
                    e => {
                        let name = if let Some(id) = g.attributes.get("id") {
                            &format!("{e} ({id})")
                        } else {
                            e
                        };
                        egui::CollapsingHeader::new(format!("Element: {name}"))
                            .id_salt(format!("element_{idx}"))
                            .show(ui, |ui| {
                                ui.add_enabled_ui(is_editable, |ui| {
                                    if e == "path" {
                                        if ui.button("edit").clicked() {
                                            match self.ref_group {
                                                Some(i) if i == idx => {
                                                    self.ref_group = None; // Deselect if already selected
                                                }
                                                _ => {
                                                    // Select the current group
                                                    self.ref_group = Some(idx);
                                                }
                                            }
                                        }
                                    } else if e == "circle" {
                                        // Convert circle to path logic here
                                        // For example, you can create a path string based on circle attributes
                                        if let Some(cx) = g.attributes.get("cx")
                                            && let Some(cy) = g.attributes.get("cy")
                                            && let Some(r) = g.attributes.get("r")
                                            && ui.button("Convert to path").clicked()
                                        {
                                            match circle_to_path(cx, cy, r) {
                                                Ok(path_data) => {
                                                    g.name = "path".to_string();
                                                    g.attributes.insert("d".to_string(), path_data);
                                                    g.attributes.shift_remove("cx");
                                                    g.attributes.shift_remove("cy");
                                                    g.attributes.shift_remove("r");
                                                }
                                                Err(err) => {
                                                    error_manager.add_error(err);
                                                }
                                            }
                                        }
                                    } else if e == "ellipse" {
                                        // Convert ellipse to path logic here
                                        if let Some(cx) = g.attributes.get("cx")
                                            && let Some(cy) = g.attributes.get("cy")
                                            && let Some(rx) = g.attributes.get("rx")
                                            && let Some(ry) = g.attributes.get("ry")
                                            && ui.button("Convert to path").clicked()
                                        {
                                            match ellipse_to_path(cx, cy, rx, ry) {
                                                Ok(path_data) => {
                                                    g.name = "path".to_string();
                                                    g.attributes.insert("d".to_string(), path_data);
                                                    g.attributes.shift_remove("cx");
                                                    g.attributes.shift_remove("cy");
                                                    g.attributes.shift_remove("rx");
                                                    g.attributes.shift_remove("ry");
                                                }
                                                Err(err) => {
                                                    error_manager.add_error(err);
                                                }
                                            }
                                        }
                                    } else if e == "polyline"
                                        && ui.button("Convert to path").clicked()
                                    {
                                        // Convert polyline to path logic here
                                        if let Some(points) = g.attributes.get("points") {
                                            let path_data = polyline_to_path(points);
                                            g.name = "path".to_string();
                                            g.attributes.insert("d".to_string(), path_data);
                                            g.attributes.shift_remove("points");
                                        }
                                    } else if e == "line" {
                                        // Convert line to path logic here
                                        if let Some(x1) = g.attributes.get("x1")
                                            && let Some(y1) = g.attributes.get("y1")
                                            && let Some(x2) = g.attributes.get("x2")
                                            && let Some(y2) = g.attributes.get("y2")
                                            && ui.button("Convert to path").clicked()
                                        {
                                            let path_data = line_to_path(x1, y1, x2, y2);
                                            g.name = "path".to_string();
                                            g.attributes.insert("d".to_string(), path_data);
                                            g.attributes.shift_remove("x1");
                                            g.attributes.shift_remove("y1");
                                            g.attributes.shift_remove("x2");
                                            g.attributes.shift_remove("y2");
                                        }
                                    } else if e == "polygon"
                                        && ui.button("Convert to path").clicked()
                                    {
                                        // Convert polygon to path logic here
                                        if let Some(points) = g.attributes.get("points") {
                                            let path_data = polygon_to_path(points);
                                            g.name = "path".to_string();
                                            g.attributes.insert("d".to_string(), path_data);
                                            g.attributes.shift_remove("points");
                                        }
                                    } else if e == "rect" {
                                        // Convert rectangle to path logic here
                                        if let Some(x) = g.attributes.get("x")
                                            && let Some(y) = g.attributes.get("y")
                                            && let Some(width) = g.attributes.get("width")
                                            && let Some(height) = g.attributes.get("height")
                                            && ui.button("Convert to path").clicked()
                                        {
                                            {
                                                let path_data = rect_to_path(x, y, width, height);
                                                g.name = "path".to_string();
                                                g.attributes.insert("d".to_string(), path_data);
                                                g.attributes.shift_remove("x");
                                                g.attributes.shift_remove("y");
                                                g.attributes.shift_remove("width");
                                                g.attributes.shift_remove("height");
                                            }
                                        }
                                    }
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
                                            let mut remove_idx = None;
                                            for (key, value) in g.attributes.iter_mut() {
                                                body.row(0.0, |mut row| {
                                                    row.col(|ui| {
                                                        ui.scope(|ui| {
                                                            ui.style_mut()
                                                                .visuals
                                                                .widgets
                                                                .hovered
                                                                .weak_bg_fill = Color32::RED;

                                                            if ui
                                                                .button(key)
                                                                .on_hover_text(
                                                                    "Remove this attribute",
                                                                )
                                                                .clicked()
                                                            {
                                                                remove_idx = Some(key.clone());
                                                            }
                                                        });
                                                    });
                                                    row.col(|ui| {
                                                        if self.is_multi_line {
                                                            ui.text_edit_multiline(value);
                                                        } else {
                                                            ui.text_edit_singleline(value);
                                                        }
                                                    });
                                                });
                                            }
                                            let key_attr =
                                                self.attributes_temp.entry(idx).or_default();
                                            body.row(0.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.horizontal(|ui| {
                                                        ui.text_edit_singleline(key_attr);
                                                    });
                                                });
                                                row.col(|ui| {
                                                    if ui
                                                        .button("Add key")
                                                        .on_hover_text("Add new attribute")
                                                        .clicked()
                                                        && !key_attr.is_empty()
                                                    {
                                                        g.attributes.insert(
                                                            key_attr.clone(),
                                                            "".to_string(),
                                                        );
                                                        key_attr.clear();
                                                    }
                                                });
                                            });

                                            if let Some(idx) = remove_idx {
                                                g.attributes.shift_remove(&idx);
                                            }
                                        });
                                });
                            });
                        if let Some(index) = self.ref_group
                            && *e == *"path"
                            && index == idx
                        {
                            self.show_current_edition(ui.ctx(), g);
                        }
                    }
                },
                xmltree::XMLNode::Text(t) => {
                    egui::CollapsingHeader::new("Text")
                        .id_salt(format!("text_{idx}"))
                        .show(ui, |ui| {
                            ui.label(format!("Text: {t}"));
                        });
                }
                xmltree::XMLNode::Comment(comment_value) => {
                    egui::CollapsingHeader::new("Comment")
                        .id_salt(format!("comment_{idx}"))
                        .show(ui, |ui| {
                            ui.add_enabled_ui(is_editable, |ui| {
                                ui.text_edit_singleline(comment_value);
                            });
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
                    let mut parsed_path = match SvgPath::parse(path) {
                        Ok(path) => path,
                        Err(e) => {
                            ui.label(format!("Invalid path: {e}"));
                            return;
                        }
                    };
                    if ui
                        .button("Convert to relative")
                        .on_hover_text("Convert path to relative coordinates")
                        .clicked()
                    {
                        parsed_path.relative();
                        *path = parsed_path.to_string();
                    }
                    if ui
                        .button("Convert to absolute")
                        .on_hover_text("Convert path to absolute coordinates")
                        .clicked()
                    {
                        parsed_path.absolute();
                        *path = parsed_path.to_string();
                    }
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.translate_x)
                                .speed(0.1)
                                .prefix("x: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.translate_y)
                                .speed(0.1)
                                .prefix("y: "),
                        );
                        ui.button("Translate")
                            .on_hover_text("Translate path by the given values")
                            .clicked()
                            .then(|| {
                                parsed_path.translate(self.translate_x, self.translate_y);
                                *path = parsed_path.to_string();
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.scale_x)
                                .speed(0.1)
                                .prefix("x: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.scale_y)
                                .speed(0.1)
                                .prefix("y: "),
                        );
                        ui.button("Scale")
                            .on_hover_text("Scale path by the given values")
                            .clicked()
                            .then(|| {
                                parsed_path.scale(self.scale_x, self.scale_y);
                                *path = parsed_path.to_string();
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut self.rotate_x)
                                .speed(0.1)
                                .prefix("x: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.rotate_y)
                                .speed(0.1)
                                .prefix("y: "),
                        );
                        ui.add(
                            egui::DragValue::new(&mut self.rotate)
                                .speed(0.1)
                                .suffix("°"),
                        );
                        ui.button("Rotate")
                            .on_hover_text("Rotate path by the given values")
                            .clicked()
                            .then(|| {
                                parsed_path.rotate(self.rotate, self.rotate_x, self.rotate_y);
                                *path = parsed_path.to_string();
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut self.round_to).speed(1.0));
                        ui.button("Round")
                            .on_hover_text("Round path coordinates to the given value")
                            .clicked()
                            .then(|| {
                                parsed_path.round(self.round_to);
                                *path = parsed_path.to_string();
                            });
                    });

                    egui::ScrollArea::vertical()
                        .id_salt("string_viewer")
                        .show(ui, |ui| {
                            let segments = &mut parsed_path.items;
                            let mut idx_to_update = None;
                            let mut item_edit = None;
                            for (idx, path_segment) in segments.iter_mut().enumerate() {
                                ui.horizontal_wrapped(|ui| {
                                    let letter = path_segment.get_letter();

                                    if ui.button(letter.to_string()).clicked() {
                                        idx_to_update = Some(idx);
                                    }
                                    if let PathSegment::ClosePath { abs: _ } = path_segment.inner {
                                        // ClosePath don't have numbers
                                        return;
                                    }
                                    let curr_value = path_segment.to_string();

                                    if self.edit_path_as_input {
                                        // remove first character from the value
                                        let numbers_part = curr_value[1..].to_string();
                                        let mut vec_ret = vec![];
                                        for one_value in numbers_part.split(" ") {
                                            let mut value_float = one_value.parse().unwrap_or(0.0);
                                            ui.add(DragValue::new(&mut value_float));
                                            vec_ret.push(value_float);
                                        }
                                        let joined = vec_ret
                                            .iter()
                                            .map(|val| val.to_string())
                                            .collect::<Vec<String>>()
                                            .join(" ");
                                        if joined != curr_value[1..] {
                                            item_edit = Some((idx, format!("{letter}{joined}")))
                                        }
                                    } else {
                                        // remove first character from the value
                                        let mut numbers_part = curr_value[1..].to_string();
                                        ui.text_edit_singleline(&mut numbers_part);

                                        if numbers_part != curr_value[1..] {
                                            item_edit =
                                                Some((idx, format!("{letter}{numbers_part}")))
                                        }
                                    }
                                });
                            }
                            if let Some(idx) = idx_to_update {
                                parsed_path.toggle_coord_type_at(idx);
                                *path = parsed_path.to_string();
                            } else if let Some((idx, val)) = item_edit
                                && let Ok(new_path) = parsed_path.try_replace_element_at(idx, &val)
                            {
                                *path = new_path;
                            }
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

/// Tree viewer panel
#[derive(Debug)]
pub struct TreeViewerPanel;

impl BladvakPanel for TreeViewerPanel {
    type App = GalagoApp;

    fn name(&self) -> &str {
        "SVG tree"
    }

    fn has_settings(&self) -> bool {
        true
    }

    fn ui_settings(
        &self,
        app: &mut Self::App,
        ui: &mut egui::Ui,
        _error_manager: &mut ErrorManager,
    ) {
        ui.checkbox(&mut app.tree_viewer.is_multi_line, "Multi line attributes");
        ui.checkbox(
            &mut app.tree_viewer.edit_path_as_input,
            "Edit path as inputs",
        );
    }

    fn has_ui(&self) -> bool {
        true
    }

    fn ui(&self, app: &mut Self::App, ui: &mut egui::Ui, error_manager: &mut ErrorManager) {
        app.tree_viewer.show(ui, &mut app.svg, error_manager);
    }
}
