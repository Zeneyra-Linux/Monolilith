use std::io;
use std::path::{Path, PathBuf};
use kagero::runner::{shell, result};
use super::list_files;

/// Go
/// 
/// Builds a Go project.
/// Also automatically includes every Go file in the root of project folder.
pub fn build(path: impl AsRef<Path>, cwd: PathBuf, binname: String, verbose: bool) -> io::Result<()> {
    let files = list_files(path, "go")?;

    if verbose {
        // Run shell
    } else {
        // Run result
    }
}