use std::io;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::compile::*;

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

#[derive(Deserialize, Serialize, Debug)]
pub enum Project {
    Zig,
    ZigCC,
    ZigCXX,
    Cargo,
    CargoZigbuild,
    Go,
    GCC,
    GXX,
    Clang,
    ClangXX
}

impl AsRef<str> for Project {
    fn as_ref(&self) -> &str {
        match self {
            Project::Zig => "zig",
            Project::ZigCC => "zigcc",
            Project::ZigCXX => "zigc++",
            Project::Cargo => "cargo",
            Project::CargoZigbuild => "cargo-zigbuild",
            Project::Go => "go",
            Project::GCC => "gcc",
            Project::GXX => "g++",
            Project::Clang => "clang",
            Project::ClangXX => "clang++",
        }
    }
}

impl ToString for Project {
    fn to_string(&self) -> String {
        match self {
            Project::Zig => "zig".to_string(),
            Project::ZigCC => "zigcc".to_string(),
            Project::ZigCXX => "ZigCXX".to_string(),
            Project::Cargo => "cargo".to_string(),
            Project::CargoZigbuild => "cargo-zigbuild".to_string(),
            Project::Go => "go".to_string(),
            Project::GCC => "gcc".to_string(),
            Project::GXX => "g++".to_string(),
            Project::Clang => "clang".to_string(),
            Project::ClangXX => "clang++".to_string(),
        }
    }
}

impl Project {
    pub fn from_str(nametype: &str) -> Option<Project> {
        match nametype {
            "zig" => Some(Project::Zig),
            "zigcc" => Some(Project::ZigCC),
            "ZigCXX" => Some(Project::ZigCXX),
            "cargo" => Some(Project::Cargo),
            "cargo-zigbuild" => Some(Project::CargoZigbuild),
            "go" => Some(Project::Go),
            "gcc" => Some(Project::GCC),
            "g++" => Some(Project::GXX),
            "clang" => Some(Project::Clang),
            "clang++" => Some(Project::ClangXX),
            _ => None
        }
    }

    pub fn build(&self, path: impl AsRef<Path>, verbose: bool) {
        match self {
            Project::Zig => zig::zig(path, verbose),
            Project::ZigCC => zig::zigcc(path, verbose),
            Project::ZigCXX => zig::zigcxx(path, verbose),
            Project::Cargo => cargo::zigbuild(path, verbose),
            Project::CargoZigbuild => cargo::zigbuild(path, verbose),
            Project::Go => go::build(path, verbose),
            Project::GCC => gcc::cc(path, verbose),
            Project::GXX => gcc::cxx(path, verbose),
            Project::Clang => clang::cc(path, verbose),
            Project::ClangXX => clang::cxx(path, verbose),
        }
    }
}