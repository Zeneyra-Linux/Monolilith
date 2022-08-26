use std::io;
use std::path::{Path, PathBuf};
use super::{execute, c_command, cxx_command};

/// GCC
/// 
/// Compiles a C project with GCC.
pub fn cc(path: impl AsRef<Path>, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get C build command
    let cmd = c_command("gcc", None, path, outfile)?;

    // Run build command
    execute(cmd, verbose)
}

/// G++
/// 
/// Compiles a C++ project with GCC.
pub fn cxx(path: impl AsRef<Path>, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get C++ build command
    let cmd = cxx_command("gcc", None, path, outfile)?;

    // Run build command
    execute(cmd, verbose)
}