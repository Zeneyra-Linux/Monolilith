use std::io;
use std::path::{Path, PathBuf};
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

/// Zig
/// 
/// Builds a Zig project.
pub fn zig(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    Ok(())
}