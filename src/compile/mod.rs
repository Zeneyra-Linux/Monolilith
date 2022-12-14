use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::process::Command;
use kagero::runner::{shell, silent};

/// Cargo
/// 
/// Module for Cargo-specific commands
pub mod cargo;

/// Zig
/// 
/// Module for Zig-specific commands
pub mod zig;

/// GCC
/// 
/// Module for GCC-specific commands
pub mod gcc;

/// Clang
/// 
/// Module for Clang-specific commands
pub mod clang;

/// Go
/// 
/// Module for Go-specific commands
pub mod go;


/// Execute Command
/// 
/// Executes a Command, optionally in a verbose manner
pub fn execute(cmd: Command, verbose: bool) -> io::Result<()> {
    // Bind stdout and stderr to shell if verbose flag is set
    if verbose {
        return shell(cmd);
    }
    // Use result runner, but ignore output
    silent(cmd)
}

/// C Command
/// 
/// Function to build a C command because I don't want to repeat myself...
pub fn c_command(cc: &str, arg1: Option<&str>, path: impl AsRef<Path>, outfile: PathBuf) -> io::Result<Command> {
    // Get C source files
    let cfiles = list_files_recursive(path.as_ref(), "c")?;
    let hfiles = list_files_recursive(path.as_ref(), "h")?;
        
    // Create gcc command
    let mut cmd = Command::new(cc);

    // Append arg1, for Zig only I guess
    if let Some(viparg) = arg1 {
        cmd.arg(viparg);
    }

    // Set output and cwd
    cmd.arg("-o").arg(outfile).current_dir(path)
    // Set source files and Opt-Level
    .arg("-O3").arg("-s").arg("-w").args(cfiles).args(hfiles);

    Ok(cmd)
}

/// C++ Command
/// 
/// Function to build a C++ command because I don't want to repeat myself... The irony...
pub fn cxx_command(cc: &str, arg1: Option<&str>, path: impl AsRef<Path>, outfile: PathBuf) -> io::Result<Command> {
    // Get C++ source files
    let cppfiles = list_files_recursive(path.as_ref(), "cpp")?;
    let hppfiles = list_files_recursive(path.as_ref(), "hpp")?;

    // Get C build command
    let mut cmd = c_command(cc, arg1, path, outfile)?;
    // Append C++ and H++ source files
    cmd.args(cppfiles).args(hppfiles);

    Ok(cmd)
}

/// List Directory Entries
/// 
/// List Directory Entries as a tuple of [DirEntry](fs::DirEntry) and [Metadata](fs::Metadata) in an [Iterator]
fn list_meta(dir: impl AsRef<Path>) -> io::Result<impl Iterator<Item = (fs::DirEntry, fs::Metadata)>> {
    // Read Directory and Iterate over Files
    let entries = fs::read_dir(dir)?.into_iter()
    // Filter Out unavailable entries
    .filter_map(|entry| {
        // Check if entry is available
        if let Ok(direntry) = entry {
            // Check if entry's metadata is available
            if let Ok(filemeta) = direntry.metadata() {
                return Some((direntry, filemeta));
            }
        }
        None
    });
    Ok(entries)
}

/// List Files by Extension
/// 
/// Lists all files in a directory by extension.
/// Returns a [Vec<PathBuf>] containing [PathBuf]s.
/// Used for things like C and Go compiling where all root files have to be set.
/// 
/// `ext` should be the file extension WITHOUT the dot, e.g. `rs`, `go`, `c`, etc.
/// 
/// # Example
/// ```
/// for ele in compile::list_files("./src", "rs").unwrap() {
///     println!("{}", ele);
/// }
/// ```
/// 
/// You can get the file name only by using `.file_name()` for the [PathBuf] elements
pub fn list_files(dir: impl AsRef<Path>, ext: &str) -> io::Result<Vec<PathBuf>> {
    let files_iter = list_meta(dir)?
    // Filter out non-files
    .filter(|x| x.1.is_file())
    // Filter by extension
    .filter(|x| {
        if let Some(extension) = x.0.path().extension() {
            if let Ok(extend) = extension.to_owned().into_string() {
                return extend == ext;
            }
        }
        false
    });

    // Collect file entries to an array of their paths
    Ok(files_iter.map(|x| x.0.path()).collect())
}

/// List Directories
/// 
/// Lists all Directories in the set folder.
/// Returns a [Vec<PathBuf>] containing [PathBuf]s.
fn list_dirs(dir: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let entries = list_meta(dir)?
    // Filter out non-directories
    .filter(|x| x.1.is_dir());

    // Return Paths of Directories.
    Ok(entries.map(|x| x.0.path()).collect())
}

/// List Files Recursively by Extension
/// 
/// Lists all files in a directory and subdirectories by extensions.
/// Returns a [Vec<PathBuf>] containing [PathBuf]s.
pub fn list_files_recursive(dir: impl AsRef<Path>, ext: &str) -> io::Result<Vec<PathBuf>> {
    // Get files for current directory
    let mut files = list_files(&dir, ext)?;

    // Get directories of the current dir
    for folder in list_dirs(&dir)? {
        // Iterate over subfolder files
        list_files_recursive(folder, ext)?.into_iter()
        // Append the new files
        .for_each(|x| files.push(x));
    }

    // Return total files
    Ok(files)
}
