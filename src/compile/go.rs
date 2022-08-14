use std::path::Path;
use kagero::runner::{shell, result};
use kagero::printer::{Printer, Colors};

/// Go
/// 
/// Builds a Go project.
/// Also automatically includes every Go file in the root of project folder.
pub fn build(path: impl AsRef<Path>, verbose: bool) {
    
}