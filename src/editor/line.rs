//! svg line

use resvg::usvg::roxmltree::Node;

/// svg line
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub(crate) struct SvgLine {
    /// x1
    x1: f32,
    /// y1
    y1: f32,
    /// x2
    x2: f32,
    /// y2
    y2: f32,
}

impl SvgLine {
    /// Try to parse the value
    pub(crate) fn try_parse(node: Node<'_, '_>) -> Result<Self, String> {
        let x1 = node
            .attribute("x1")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse x property")?;
        let y1 = node
            .attribute("y1")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse y property")?;
        let x2 = node
            .attribute("x2")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse x property")?;
        let y2 = node
            .attribute("y2")
            .unwrap_or("0")
            .parse()
            .map_err(|_e| "Cannot parse y property")?;
        Ok(Self { x1, y1, x2, y2 })
    }
}
