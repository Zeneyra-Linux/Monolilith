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
/// Note that this function will only return a [HashMap<String, String>]. The [Projects] must be made manually from the values.
pub fn parse_config(data: &str) -> Result<HashMap<String, String>, serde_json::Error> {
    let config: HashMap<String, String> = serde_json::from_str(data)?;
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

impl ToString for Projects {
    fn to_string(&self) -> String {
        match self {
            Projects::Zig => "zig".to_string(),
            Projects::ZigCC => "zigcc".to_string(),
            Projects::ZigCPP => "zigcpp".to_string(),
            Projects::Cargo => "cargo".to_string(),
            Projects::CargoZigbuild => "cargo-zigbuild".to_string(),
            Projects::Go => "go".to_string(),
            Projects::GCC => "gcc".to_string(),
            Projects::GPP => "g++".to_string(),
            Projects::Clang => "clang".to_string(),
            Projects::ClangPP => "clang++".to_string(),
            Projects::Unknown(ref s) => s.to_string()
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