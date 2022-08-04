use serde_json;
use std::collections::HashMap;
use serde::Deserialize;

pub fn parse_config(data: &str) -> Result<HashMap<String, Projects>, serde_json::Error> {
    let config: HashMap<String, Projects> = serde_json::from_str(data)?;
    Ok(config)
}

#[derive(Deserialize)]
pub enum Projects {
    Zig,
    ZigCC,
    ZigCPP,
    Cargo,
    CargoZigbuild,
    Go,
    GCC,
    GPP,
    Clang,
    ClangPP,
    Unknown(String)
}

impl AsRef<str> for Projects {
    fn as_ref(&self) -> &str {
        match self {
            Projects::Zig => "zig",
            Projects::ZigCC => "zigcc",
            Projects::ZigCPP => "zigcpp",
            Projects::Cargo => "cargo",
            Projects::CargoZigbuild => "cargo-zigbuild",
            Projects::Go => "go",
            Projects::GCC => "gcc",
            Projects::GPP => "g++",
            Projects::Clang => "clang",
            Projects::ClangPP => "clang++",
            Projects::Unknown(ref s) => s.as_str()
        }
    }
}