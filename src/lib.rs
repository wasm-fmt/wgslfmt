mod config;

use wasm_bindgen::prelude::*;

pub use crate::config::Config;
use crate::config::WGSLConfig;

/// Formats a WGSL string with optional configuration.
#[wasm_bindgen]
pub fn format(
    #[wasm_bindgen(param_description = "The WGSL string to format")] input: &str,
    #[wasm_bindgen(param_description = "Optional formatter config")] config: Option<Config>,
) -> Result<String, String> {
    let config = config
        .map(|x| serde_wasm_bindgen::from_value::<WGSLConfig>(x.into()))
        .transpose()
        .map_err(|op| op.to_string())?
        .unwrap_or_default();

    Ok(wgsl_formatter::format_str(input, &config.into()))
}
