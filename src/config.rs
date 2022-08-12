use serde_json;
use std::io;
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Config Wrapper
/// 
/// Wrapper for [read_config] and [parse_config]
pub fn config() -> Result<HashMap<String, String>, io::Error> {
    let data = read_config()?;
    let config = parse_config(data.as_str())?;
    Ok(config)
}

/// Config Reader
/// 
/// Simple Wrapper to read the `monolilith.json` file.
pub fn read_config() -> Result<String, io::Error> {
    match fs::read_to_string("monolilith.json") {
        Ok(data) => Ok(data),
        Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, "Could not find monolilith.json"))
    }
}

/// Config Parser
/// 
/// Simple Wrapper for the `serde_json::from_str` function.
/// Note that this function will only return a [HashMap<String, String>]. The [Project] must be made manually from the values.
pub fn parse_config(data: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    let config: HashMap<String, String> = serde_json::from_str(data)?;
    Ok(config)
}

#[derive(Deserialize, Serialize)]
pub enum Project {
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

impl AsRef<str> for Project {
    fn as_ref(&self) -> &str {
        match self {
            Project::Zig => "zig",
            Project::ZigCC => "zigcc",
            Project::ZigCPP => "zigcpp",
            Project::Cargo => "cargo",
            Project::CargoZigbuild => "cargo-zigbuild",
            Project::Go => "go",
            Project::GCC => "gcc",
            Project::GPP => "g++",
            Project::Clang => "clang",
            Project::ClangPP => "clang++",
            Project::Unknown(ref s) => s.as_str()
        }
    }
}

impl ToString for Project {
    fn to_string(&self) -> String {
        match self {
            Project::Zig => "zig".to_string(),
            Project::ZigCC => "zigcc".to_string(),
            Project::ZigCPP => "zigcpp".to_string(),
            Project::Cargo => "cargo".to_string(),
            Project::CargoZigbuild => "cargo-zigbuild".to_string(),
            Project::Go => "go".to_string(),
            Project::GCC => "gcc".to_string(),
            Project::GPP => "g++".to_string(),
            Project::Clang => "clang".to_string(),
            Project::ClangPP => "clang++".to_string(),
            Project::Unknown(ref s) => s.to_string()
        }
    }
}

impl Project {
    pub fn from_str(nametype: &str) -> Project {
        match nametype {
            "zig" => Project::Zig,
            "zigcc" => Project::ZigCC,
            "zigcpp" => Project::ZigCPP,
            "cargo" => Project::Cargo,
            "cargo-zigbuild" => Project::CargoZigbuild,
            "go" => Project::Go,
            "gcc" => Project::GCC,
            "g++" => Project::GPP,
            "clang" => Project::Clang,
            "clang++" => Project::ClangPP,
            _ => Project::Unknown(nametype.to_string())
        }
    }

    pub fn valid(&self) -> bool {
        match self {
            Project::Zig => true,
            Project::ZigCC => true,
            Project::ZigCPP => true,
            Project::Cargo => true,
            Project::CargoZigbuild => true,
            Project::Go => true,
            Project::GCC => true,
            Project::GPP => true,
            Project::Clang => true,
            Project::ClangPP => true,
            Project::Unknown(_) => false
        }
    }

    pub fn valid_str(t: &str) -> bool {
        match t {
            "zig" => true,
            "zigcc" => true,
            "zigcpp" => true,
            "cargo" => true,
            "cargo-zigbuild" => true,
            "go" => true,
            "gcc" => true,
            "g++" => true,
            "clang" => true,
            "clang++" => true,
            _ => false
        }
    }
}