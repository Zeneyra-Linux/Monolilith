use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::{execute, cxx_command, c_command};

/// Zig CC
/// 
/// Compiles a Zig project with Zig's C compiler.
pub fn zigcc(path: impl AsRef<Path>, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get C build command
    let cmd = c_command("zig", Some("cc"), path, outfile)?;

    // Run build command
    execute(cmd, verbose)
}

/// Zig C++
/// 
/// Compiles a Zig project with Zig's C compiler.
pub fn zigcxx(path: impl AsRef<Path>, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get C++ build command
    let cmd = cxx_command("zig", Some("c++"), path, outfile)?;

    // Run build command
    execute(cmd, verbose)
}

/// Zig (Safe)
/// 
/// Builds a Zig project with the safe build mode
pub fn zigsafe(path: impl AsRef<Path>, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    zig("-Drelease-safe=true", path, binname, outfile, verbose)
}

/// Zig (Fast)
/// 
/// Builds a Zig project with the fast build mode
pub fn zigfast(path: impl AsRef<Path>, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    zig("-Drelease-fast=true", path, binname, outfile, verbose)
}

/// Zig (Small)
/// 
/// Builds a Zig project with the small build mode
pub fn zigsmall(path: impl AsRef<Path>, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    zig("-Drelease-small=true", path, binname, outfile, verbose)
}

/// Zig
/// 
/// Builds a Zig project with the set build mode
fn zig(releaseflag: &str, path: impl AsRef<Path>, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Create Zig Build command
    let mut cmd = Command::new("zig");
    cmd.arg("build").arg(releaseflag);
    
    // Run build command
    execute(cmd, verbose)?;
    
    // Copy built executable into output folder
    let buildfile = path.as_ref().join("zig-out/bin/").join(binname);
    fs::copy(buildfile, outfile)?;
    Ok(())
}