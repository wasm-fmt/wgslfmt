use serde::{Deserialize, Deserializer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_Config: &'static str = r#"
import type { Config } from "./wgslfmt_config.d.ts";
export type * from "./wgslfmt_config.d.ts";
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Config")]
    pub type Config;
}

fn deserialize_policy<'de, D>(deserializer: D) -> Result<Option<wgsl_formatter::Policy>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s.as_deref() {
        Some("ignore") => Ok(Some(wgsl_formatter::Policy::Ignore)),
        Some("insert") => Ok(Some(wgsl_formatter::Policy::Insert)),
        Some("remove") => Ok(Some(wgsl_formatter::Policy::Remove)),
        Some(_) => Err(serde::de::Error::custom("invalid policy")),
        None => Ok(None),
    }
}

/// Configuration for the WGSL formatter
#[derive(Clone, Debug, Default, Deserialize)]
pub struct WGSLConfig {
    /// How to handle trailing commas
    #[serde(alias = "trailingCommas")]
    #[serde(deserialize_with = "deserialize_policy")]
    pub trailing_commas: Option<wgsl_formatter::Policy>,
    /// The string used for indentation (e.g., "\t" or "    ")
    #[serde(alias = "indentSymbol")]
    pub indent_symbol: Option<String>,
}

impl From<WGSLConfig> for wgsl_formatter::FormattingOptions {
    fn from(config: WGSLConfig) -> Self {
        let mut options = wgsl_formatter::FormattingOptions::default();

        if let Some(trailing_commas) = config.trailing_commas {
            options.trailing_commas = trailing_commas;
        }

        if let Some(indent_symbol) = config.indent_symbol {
            options.indent_symbol = indent_symbol;
        }

        options
    }
}
