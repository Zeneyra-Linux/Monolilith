use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::execute;

/// Cargo Build
/// 
/// Runs the `cargo build --release` command.
pub fn build(path: impl AsRef<Path>, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // cargo build --release in project subdir
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--release"]).current_dir(path.as_ref());

    // Run Build
    execute(cmd, verbose)?;

    // Copy resulting binary into output folder
    let buildfile = path.as_ref().join("target/release/").join(binname);
    fs::copy(buildfile, outfile)?;
    Ok(())
}

/// Cargo Zigbuild
/// 
/// Runs `cargo zigbuild --release`.
pub fn zigbuild(path: impl AsRef<Path>, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // cargo zigbuild --release in project subdir
    let mut cmd = Command::new("cargo");
    cmd.args(["zigbuild", "--release"]).current_dir(path.as_ref());

    // Run Build
    execute(cmd, verbose)?;

    // Copy resulting binary into output folder
    let buildfile = path.as_ref().join("target/release/").join(binname);
    fs::copy(buildfile, outfile)?;
    Ok(())
}