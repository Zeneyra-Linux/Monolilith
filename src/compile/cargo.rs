use std::io;
use std::path::{Path, PathBuf};
use kagero::runner::{shell, result};

/// Cargo Build
/// 
/// Runs the `cargo build --release` command.
pub fn build(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {

}

/// Cargo Zigbuild
/// 
/// Runs `cargo zigbuild --release`.
pub fn zigbuild(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {

}