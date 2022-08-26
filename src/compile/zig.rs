use std::io;
use std::path::{Path, PathBuf};
use super::execute;

/// Zig CC
/// 
/// Compiles a Zig project with Zig's C compiler.
pub fn zigcc(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    Ok(())
}

/// Zig C++
/// 
/// Compiles a Zig project with Zig's C compiler.
pub fn zigcxx(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    Ok(())
}

/// Zig
/// 
/// Builds a Zig project.
pub fn zig(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    Ok(())
}