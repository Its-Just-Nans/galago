//! Text

use resvg::usvg::roxmltree::Node;

/// svg text
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SvgText {
    /// x
    x: f32,
    /// y
    y: f32,
    /// inner text
    inner_text: String,
}

impl SvgText {
    /// Try to parse the value
    pub(crate) fn try_parse(node: Node<'_, '_>) -> Result<Self, String> {
        let x = node
            .attribute("x")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse x property")?;
        let y = node
            .attribute("y")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse y property")?;
        let t = if let Some(text) = node.text() {
            text.to_string()
        } else {
            String::new()
        };
        Ok(Self {
            x,
            y,
            inner_text: t,
        })
    }
}
