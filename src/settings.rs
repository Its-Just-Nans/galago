//! App settings

/// App settings
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub(crate) struct AppSettings {
    /// auto scale
    pub(crate) auto_scale: bool,
    /// global scaler value
    pub(crate) global_scaler: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            auto_scale: true,
            global_scaler: 1,
        }
    }
}
