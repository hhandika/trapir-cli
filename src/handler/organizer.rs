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
        let img_paths = Finder::new(input_dir).scan_directory();
        self.parse_config_csv(cfg_path);
        self.parse_img_records();
        self.print_input(input_dir, &cfg_path);
        self.organize_by_taxa(&img_paths, output_dir);
    }

    fn organize_by_taxa(&self, img_paths: &[PathBuf], output_dir: &Path) {
        let spin = spinner::set_spinner();
        spin.set_message("Organizing images by taxa");

        img_paths.iter().for_each(|img_path| {
            let img_name = img_path
                .file_name()
                .expect("Failed parsing filenames")
                .to_string_lossy()
                .to_uppercase()
                .to_string();

            let output = match self.img_records.get(&img_name) {
                Some(path) => Path::new(path),
                None => Path::new("unknown"),
            };

            let output_path = output_dir.join(output).join(img_name);
            fs::create_dir_all(output_path.parent().expect("Could not get parent path"))
                .expect("Could not create directory");
            match fs::rename(img_path, &output_path) {
                Ok(_) => (),
                Err(e) => {
                    spin.set_message(format!(
                        "Could not move image: {} for {}",
                        e,
                        output_path.display()
                    ));
                    return;
                }
            }
        });

        spin.finish_with_message("Done organizing images by taxa!");
    }

    fn parse_img_records(&mut self) {
        self.records.iter().for_each(|rec| {
            let taxon_name = rec.scientific_id.trim().replace(" ", "_").replace(".", "");
            let path = Path::new(&taxon_name)
                .join(&rec.locality)
                .join(&rec.station);
            self.img_records.insert(rec.image_id.to_uppercase(), path);
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
    locality: String,
    station: String,
    image_id: String,
    scientific_id: String,
}
