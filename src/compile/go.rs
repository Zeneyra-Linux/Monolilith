use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use kagero::runner::{shell, result};
use super::list_files;

/// Go
/// 
/// Builds a Go project.
/// Also automatically includes every Go file in the root of project folder.
pub fn build(path: impl AsRef<Path>, cwd: PathBuf, binname: String, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get root Go files
    let files = list_files(path, "go")?;
    
    // Create Go Build command
    let cmd = Command::new("go").arg("build")
    // Set root source files and working directory
    .args(files).current_dir(path)
    // Set output and ldflags
    .arg("-o").arg(outfile).arg("-ldflags=\"-s -w\"");

    if verbose {
        return shell(*cmd);
    }
    // Use result runner, but ignore output
    Ok(_ = result(*cmd)?)
}