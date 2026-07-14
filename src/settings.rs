//! App settings

/// App settings
#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub(crate) struct AppSettings {
    /// auto scale
    pub(crate) auto_scale: bool,
    /// global scaler value
    pub(crate) global_scaler: u32,
}
