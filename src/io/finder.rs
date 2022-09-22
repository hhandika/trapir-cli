use std::path::{Path, PathBuf};

use glob::{glob_with, MatchOptions};
use lazy_static::lazy_static;
use regex::Regex;
use walkdir::WalkDir;

pub struct Finder<'a> {
    input: &'a Path,
}

impl<'a> Finder<'a> {
    pub fn new(input: &'a Path) -> Self {
        Self { input }
    }

    pub fn scan_directory(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        WalkDir::new(&self.input)
            .into_iter()
            .filter_map(|ok| ok.ok())
            .filter(|entry| entry.file_type().is_file())
            .for_each(|file| {
                let ext = match file.path().extension() {
                    Some(ext) => ext.to_string_lossy(),
                    None => return,
                };
                if match_extension(&ext) {
                    paths.push(file.path().to_path_buf());
                }
            });
        paths
    }

    pub fn find_jpeg(&self) -> Vec<PathBuf> {
        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        glob_with(&format!("{}/*.JPG", self.input.display()), options)
            .expect("Failed globbing files")
            .filter_map(|ok| ok.ok())
            .collect::<Vec<PathBuf>>()
    }
}

fn match_extension(text: &str) -> bool {
    lazy_static! { // Match the first word in the block
        static ref RE: Regex = Regex::new(r"^((?i)jpg|jpeg|avi|m4a|m4v|mp4)").expect("Failed capturing file extension");
    }

    RE.is_match(text)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match_extension() {
        assert!(match_extension("jpg"));
        assert!(match_extension("JPG"));
        assert!(match_extension("jpeg"));
        assert!(match_extension("JPEG"));
        assert!(match_extension("avi"));
        assert!(match_extension("AVI"));
        assert!(match_extension("m4a"));
        assert!(match_extension("M4A"));
        assert!(match_extension("m4v"));
        assert!(match_extension("M4V"));
        assert!(match_extension("mp4"));
        assert!(match_extension("MP4"));
    }
}
