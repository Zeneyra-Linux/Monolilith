use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::{list_files, execute};

/// Go
/// 
/// Builds a Go project.
/// Also automatically includes every Go file in the root of project folder.
pub fn build(path: impl AsRef<Path>, outfile: PathBuf, verbose: bool) -> io::Result<()> {
    // Get root Go files
    let files = list_files(path.as_ref(), "go")?;
    
    // Create Go Build command
    let mut cmd = Command::new("go");
    cmd.arg("build")
    // Set output and ldflags
    .arg("-ldflags=-s -w").arg("-o").arg(outfile)
    // Set root source files and working directory
    .args(files).current_dir(path);

    execute(cmd, verbose)
}
