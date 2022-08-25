use std::io;
use std::path::{Path, PathBuf};
use kagero::runner::{shell, result};

/// Clang
/// 
/// Compiles a C project with Clang.
pub fn cc(path: impl AsRef<Path>, cwd: PathBuf, binname: String, verbose: bool) -> io::Result<()> {

}

/// Clang++
/// 
/// Compiles a C++ project with Clang.
pub fn cxx(path: impl AsRef<Path>, cwd: PathBuf, binname: String, verbose: bool) -> io::Result<()> {

}