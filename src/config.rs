use serde_json;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub fn parse_config(data: &str) -> Result<HashMap<String, Projects>, serde_json::Error> {
    let config: HashMap<String, Projects> = serde_json::from_str(data)?;
    Ok(config)
}

#[derive(Deserialize, Serialize)]
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

impl Projects {
    pub fn from_str(nametype: &str) -> Projects {
        match nametype {
            "zig" => Projects::Zig,
            "zigcc" => Projects::ZigCC,
            "zigcpp" => Projects::ZigCPP,
            "cargo" => Projects::Cargo,
            "cargo-zigbuild" => Projects::CargoZigbuild,
            "go" => Projects::Go,
            "gcc" => Projects::GCC,
            "g++" => Projects::GPP,
            "clang" => Projects::Clang,
            "clang++" => Projects::ClangPP,
            _ => Projects::Unknown(nametype.to_string())
        }
    }

    pub fn valid(&self) -> bool {
        match self {
            Projects::Zig => true,
            Projects::ZigCC => true,
            Projects::ZigCPP => true,
            Projects::Cargo => true,
            Projects::CargoZigbuild => true,
            Projects::Go => true,
            Projects::GCC => true,
            Projects::GPP => true,
            Projects::Clang => true,
            Projects::ClangPP => true,
            Projects::Unknown(_) => false
        }
    }
}