use std::io::prelude::*;
use std::{fs::File, io::BufWriter, path::PathBuf};

use colored::Colorize;

pub struct Summary<'a> {
    pub img_paths: &'a [PathBuf],
}

impl<'a> Summary<'a> {
    pub fn new(img_paths: &'a [PathBuf]) -> Self {
        Self { img_paths }
    }

    pub fn summarize(&self) {
        let mut extensions = self.parse_extension(self.img_paths);
        extensions.sort();
        extensions.dedup();
        self.print_summary(&extensions);
        self.write_summary();
    }

    fn parse_extension(&self, image_paths: &[PathBuf]) -> Vec<String> {
        image_paths
            .iter()
            .map(|img| {
                img.extension()
                    .expect("Error passing extension")
                    .to_string_lossy()
                    .to_lowercase()
                    .to_string()
            })
            .collect()
    }

    fn write_summary(&self) {
        let file = File::create("summary.txt").expect("Could not open file");
        let mut writer = BufWriter::new(file);
        self.img_paths.iter().for_each(|img_path| {
            writeln!(writer, "{}", img_path.display()).expect("Could not write to file");
        });
    }

    fn print_summary(&self, exts: &[String]) {
        log::info!("{}", "Summary".yellow().bold());
        log::info!("{:18}: {}", "File counts", self.img_paths.len());
        log::info!("{:18}: {}", "Filetype counts", exts.len());
        log::info!("");
        log::info!("{}", "Filetypes".yellow());
        exts.iter().for_each(|ext| log::info!("{}", ext));
    }
}
