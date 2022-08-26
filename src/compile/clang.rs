use std::io;
use std::path::{Path, PathBuf};
use super::{execute, c_command, cxx_command};

/// Clang
/// 
/// Compiles a C project with Clang.
pub fn cc(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get C build command
    let cmd = c_command("clang", path, outfile)?;

    // Run build command
    execute(&mut cmd, verbose)
}

/// Clang++
/// 
/// Compiles a C++ project with Clang.
pub fn cxx(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get C++ build command
    let cmd = cxx_command("clang++", path, outfile)?;

    // Run build command
    execute(&mut cmd, verbose)
}