use std::path::{Path, PathBuf};

use glob::glob;

pub fn find_images(dir: &Path) -> Vec<PathBuf> {
    glob(&format!("{}/*.JPG", dir.display()))
        .expect("Failed globbing files")
        .filter_map(|ok| ok.ok())
        .collect::<Vec<PathBuf>>()
}
