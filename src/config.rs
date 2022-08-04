use serde_json;
use std::path::Path;
use std::collections::HashMap;

pub fn parse_config(data: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    let config: HashMap<String, String> = serde_json::from_str(data)?;
    Ok(config)
}

pub fn read_config(path: impl AsRef<Path>) {

}