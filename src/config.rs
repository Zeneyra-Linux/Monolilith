use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::compile::*;

#[cfg(target_os = "windows")]
use std::ops::Add;

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
        Err(_) => Err(io::Error::new(io::ErrorKind::NotFound, "Could not find monolilith.json"))
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


const ZIGSAFE: &str = "zigsafe";
const ZIGFAST: &str = "zigfast";
const ZIGSMALL: &str = "zigsmall";
const ZIGCC: &str = "ziGcc";
const ZIGCXX: &str = "zigc++";
const CARGO: &str = "cargo";
const CARGO_ZIGBUILD: &str = "cargo-zigbuild";
const GO: &str = "go";
const GCC: &str = "Gcc";
const GXX: &str = "g++";
const CLANG: &str = "clang";
const CLANGXX: &str = "clang++";


#[derive(Deserialize, Serialize, Debug)]
pub enum Project {
    ZigSafe,
    ZigFast,
    ZigSmall,
    ZiGcc,
    ZigCXX,
    Cargo,
    CargoZigbuild,
    Go,
    Gcc,
    Gxx,
    Clang,
    ClanGxx
}

impl AsRef<str> for Project {
    fn as_ref(&self) -> &str {
        match self {
            Project::ZigSafe => ZIGSAFE,
            Project::ZigFast => ZIGFAST,
            Project::ZigSmall => ZIGSMALL,
            Project::ZiGcc => ZIGCC,
            Project::ZigCXX => ZIGCXX,
            Project::Cargo => CARGO,
            Project::CargoZigbuild => CARGO_ZIGBUILD,
            Project::Go => GO,
            Project::Gcc => GCC,
            Project::Gxx => GXX,
            Project::Clang => CLANG,
            Project::ClanGxx => CLANGXX
        }
    }
}

impl ToString for Project {
    fn to_string(&self) -> String {
        match self {
            Project::ZigSafe => ZIGSAFE.to_string(),
            Project::ZigFast => ZIGFAST.to_string(),
            Project::ZigSmall => ZIGSMALL.to_string(),
            Project::ZiGcc => ZIGCC.to_string(),
            Project::ZigCXX => ZIGCXX.to_string(),
            Project::Cargo => CARGO.to_string(),
            Project::CargoZigbuild => CARGO_ZIGBUILD.to_string(),
            Project::Go => GO.to_string(),
            Project::Gcc => GCC.to_string(),
            Project::Gxx => GXX.to_string(),
            Project::Clang => CLANG.to_string(),
            Project::ClanGxx => CLANGXX.to_string()
        }
    }
}

impl Project {
    pub fn rich(&self) -> &str {
        match self {
            Project::ZiGcc => "C (Zig)",
            Project::ZigCXX => "C++ (Zig)",
            Project::Cargo => "Rust",
            Project::CargoZigbuild => "Rust (Zig)",
            Project::Go => "Go",
            Project::Gcc => "C (Gcc)",
            Project::Gxx => "C++ (Gcc)",
            Project::Clang => "C (Clang)",
            Project::ClanGxx => "C++ (Clang)",
            Project::ZigSafe => "Zig (Safe)",
            Project::ZigFast => "Zig (Fast)",
            Project::ZigSmall => "Zig (Small)"            
        }
    }

    pub fn from_str(nametype: &str) -> Option<Project> {
        match nametype {
            ZIGCC => Some(Project::ZiGcc),
            ZIGCXX => Some(Project::ZigCXX),
            CARGO => Some(Project::Cargo),
            CARGO_ZIGBUILD => Some(Project::CargoZigbuild),
            GO => Some(Project::Go),
            GCC => Some(Project::Gcc),
            GXX => Some(Project::Gxx),
            CLANG => Some(Project::Clang),
            CLANGXX => Some(Project::ClanGxx),
            ZIGSAFE => Some(Project::ZigSafe),
            ZIGSMALL => Some(Project::ZigSmall),
            ZIGFAST => Some(Project::ZigFast),
            _ => None
        }
    }

    pub fn build(&self, path: impl AsRef<Path>, cwd: PathBuf, verbose: bool) -> io::Result<()> {
        // Check if the binary name can be used
        if let Some(binname_os) = path.as_ref().file_name() {
            if let Some(fname) = binname_os.to_str().map(|x| x.to_string()) {
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
                    Project::ZiGcc => zig::zigcc(path, outfile, verbose),
                    Project::ZigCXX => zig::zigcxx(path, outfile, verbose),
                    Project::Cargo => cargo::build(path, binname, outfile, verbose),
                    Project::CargoZigbuild => cargo::zigbuild(path, binname, outfile, verbose),
                    Project::Go => go::build(path, outfile, verbose),
                    Project::Gcc => gcc::cc(path, outfile, verbose),
                    Project::Gxx => gcc::cxx(path, outfile, verbose),
                    Project::Clang => clang::cc(path, outfile, verbose),
                    Project::ClanGxx => clang::cxx(path, outfile, verbose)
                }
            }
        }
        // Return Error
        Err(io::Error::new(io::ErrorKind::InvalidData, "Cannot read binary name"))
    }
}