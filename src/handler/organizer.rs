use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
use csv::Reader;
use serde::Deserialize;

use crate::io::{finder::Finder, spinner};

pub struct Organizer {
    img_records: HashMap<String, PathBuf>,
    records: Vec<ImgRecords>,
    taxa: Vec<String>,
}

impl Organizer {
    pub fn new() -> Self {
        Self {
            img_records: HashMap::default(),
            records: Vec::new(),
            taxa: Vec::new(),
        }
    }

    pub fn organize(&mut self, input_dir: &Path, cfg_path: &Path, output_dir: &Path) {
        self.parse_config_csv(cfg_path);
        self.print_input(input_dir, &cfg_path);
        let img_paths = Finder::new(input_dir).scan_directory();
        self.parse_img_records(&img_paths);
        self.organize_by_taxa(output_dir);
    }

    fn organize_by_taxa(&self, output_dir: &Path) {
        let spin = spinner::set_spinner();
        spin.set_message("Organizing images by taxa");
        self.records.iter().for_each(|rec| {
            let taxon_name = rec.scientific_id.trim().replace(" ", "_").replace(".", "");
            let img_path = match self.img_records.get(&rec.image_id) {
                Some(path) => path,
                None => {
                    log::error!("Could not find image path for {}", rec.image_id);
                    return;
                }
            };

            let taxon_path = Path::new(&taxon_name);
            let output_path = output_dir.join(taxon_path).join(img_path);
            fs::create_dir_all(output_path.parent().expect("Could not get parent path"))
                .expect("Could not create directory");
            fs::rename(img_path, output_path).expect("Could not move file");
        });
        spin.finish_with_message("Done organizing images by taxa!");
    }

    fn parse_img_records(&mut self, img_paths: &[PathBuf]) {
        img_paths.iter().for_each(|img_path| {
            let img_name = img_path
                .file_name()
                .expect("Error parsing image name")
                .to_string_lossy()
                .to_uppercase()
                .to_string();
            self.img_records.insert(img_name, img_path.clone());
        })
    }

    fn parse_config_csv(&mut self, cfg_path: &Path) {
        let mut reader = Reader::from_path(cfg_path).expect("Failed to read config file");

        reader.deserialize().for_each(|record| {
            let recs: ImgRecords = record.expect("Failed to deserialize record");
            self.records.push(recs);
        });
    }

    fn print_input(&self, input_dir: &Path, cfg_path: &Path) {
        log::info!("{}", "Input".yellow().bold());
        log::info!("{:18}: {}", "Config file", cfg_path.display());
        log::info!("{:18}: {}", "Input directory", input_dir.display());
        log::info!("{:18}: {}", "File counts", self.img_records.len());
        log::info!("{:18}: {}", "Record counts", self.records.len());
        log::info!("{:18}: {}", "Taxon counts", self.taxa.len());
    }
}

#[derive(Debug, Deserialize)]
struct ImgRecords {
    image_id: String,
    scientific_id: String,
}
