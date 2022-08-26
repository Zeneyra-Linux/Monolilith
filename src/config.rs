use std::io;
use std::fs;
use std::path::{Path, PathBuf};
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


pub const ZIGSAFE: &str = "zigsafe";
pub const ZIGFAST: &str = "zigfast";
pub const ZIGSMALL: &str = "zigsmall";
pub const ZIGCC: &str = "zigcc";
pub const ZIGCXX: &str = "zigc++";
pub const CARGO: &str = "cargo";
pub const CARGO_ZIGBUILD: &str = "cargo-zigbuild";
pub const GO: &str = "go";
pub const GCC: &str = "gcc";
pub const GXX: &str = "g++";
pub const CLANG: &str = "clang";
pub const CLANGXX: &str = "clang++";


#[derive(Deserialize, Serialize, Debug)]
pub enum Project {
    ZigSafe,
    ZigFast,
    ZigSmall,
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
            Project::ZigSafe => ZIGSAFE,
            Project::ZigFast => ZIGFAST,
            Project::ZigSmall => ZIGSMALL,
            Project::ZigCC => ZIGCC,
            Project::ZigCXX => ZIGCXX,
            Project::Cargo => CARGO,
            Project::CargoZigbuild => CARGO_ZIGBUILD,
            Project::Go => GO,
            Project::GCC => GCC,
            Project::GXX => GXX,
            Project::Clang => CLANG,
            Project::ClangXX => CLANGXX,
        }
    }
}

impl ToString for Project {
    fn to_string(&self) -> String {
        match self {
            Project::ZigSafe => ZIGSAFE.to_string(),
            Project::ZigFast => ZIGFAST.to_string(),
            Project::ZigSmall => ZIGSMALL.to_string(),
            Project::ZigCC => ZIGCC.to_string(),
            Project::ZigCXX => ZIGCXX.to_string(),
            Project::Cargo => CARGO.to_string(),
            Project::CargoZigbuild => CARGO_ZIGBUILD.to_string(),
            Project::Go => GO.to_string(),
            Project::GCC => GCC.to_string(),
            Project::GXX => GXX.to_string(),
            Project::Clang => CLANG.to_string(),
            Project::ClangXX => CLANGXX.to_string(),
        }
    }
}

impl Project {
    pub fn rich(&self) -> &str {
        match self {
            Project::ZigCC => "C (Zig)",
            Project::ZigCXX => "C++ (Zig)",
            Project::Cargo => "Rust",
            Project::CargoZigbuild => "Rust (Zig)",
            Project::Go => "Go",
            Project::GCC => "C (GCC)",
            Project::GXX => "C++ (GCC)",
            Project::Clang => "C (Clang)",
            Project::ClangXX => "C++ (Clang)",
            Project::ZigSafe => "Zig (Safe)",
            Project::ZigFast => "Zig (Fast)",
            Project::ZigSmall => "Zig (Small)",
            
        }
    }

    pub fn from_str(nametype: &str) -> Option<Project> {
        match nametype {
            ZIGCC => Some(Project::ZigCC),
            ZIGCXX => Some(Project::ZigCXX),
            CARGO => Some(Project::Cargo),
            CARGO_ZIGBUILD => Some(Project::CargoZigbuild),
            GO => Some(Project::Go),
            GCC => Some(Project::GCC),
            GXX => Some(Project::GXX),
            CLANG => Some(Project::Clang),
            CLANGXX => Some(Project::ClangXX),
            ZIGSAFE => Some(Project::ZigSafe),
            ZIGSMALL => Some(Project::ZigSmall),
            ZIGFAST => Some(Project::ZigFast),
            _ => None
        }
    }

    pub fn build(&self, path: impl AsRef<Path>, cwd: PathBuf, verbose: bool) -> io::Result<()> {
        // Check if the binary name can be used
        if let Some(binname_os) = path.as_ref().file_name() {
            if let Some(fname) = binname_os.to_str().and_then(|x| Some(x.to_string())) {
                // Get outdir
                let outdir = cwd.join("build/");

                // Binname for Windows
                #[cfg(target_os = "windows")]
                let binname = fname.add(".exe");

                #[cfg(not(target_os = "windows"))]
                let binname = fname;

                // Set Outfile Path
                let outfile = outdir.join(binname.as_str());

                // Run build command
                return match self {
                    Project::ZigFast => zig::zigfast(path, binname, outfile, verbose),
                    Project::ZigSmall => zig::zigsmall(path, binname, outfile, verbose),
                    Project::ZigSafe => zig::zigsafe(path, binname, outfile, verbose),
                    Project::ZigCC => zig::zigcc(path, outfile, verbose),
                    Project::ZigCXX => zig::zigcxx(path, outfile, verbose),
                    Project::Cargo => cargo::build(path, binname, outfile, verbose),
                    Project::CargoZigbuild => cargo::zigbuild(path, binname, outfile, verbose),
                    Project::Go => go::build(path, outfile, verbose),
                    Project::GCC => gcc::cc(path, outfile, verbose),
                    Project::GXX => gcc::cxx(path, outfile, verbose),
                    Project::Clang => clang::cc(path, outfile, verbose),
                    Project::ClangXX => clang::cxx(path, outfile, verbose),
                }
            }
        }
        // Return Error
        Err(io::Error::new(io::ErrorKind::InvalidData, "Cannot read binary name"))
    }
}