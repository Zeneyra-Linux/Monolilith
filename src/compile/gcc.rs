use std::io;
use std::path::{Path, PathBuf};
use super::{list_files_recursive, execute};

/// GCC
/// 
/// Compiles a C project with GCC.
pub fn cc(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {

}

/// G++
/// 
/// Compiles a C++ project with GCC.
pub fn cxx(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {

}