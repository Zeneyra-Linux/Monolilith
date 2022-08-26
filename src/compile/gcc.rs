use std::io;
use std::path::{Path, PathBuf};
use super::{execute, c_command, cxx_command};

/// GCC
/// 
/// Compiles a C project with GCC.
pub fn cc(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get C build command
    let cmd = c_command("gcc", path, outfile)?;

    // Run build command
    execute(&mut cmd, verbose)
}

/// G++
/// 
/// Compiles a C++ project with GCC.
pub fn cxx(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get C++ build command
    let cmd = cxx_command("gcc", path, outfile)?;

    // Run build command
    execute(&mut cmd, verbose)
}