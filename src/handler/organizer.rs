use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
use csv::Reader;
use fs_extra::file::{move_file_with_progress, CopyOptions, TransitProcess};
use serde::Deserialize;

use crate::io::{finder::Finder, spinner};

pub struct Organizer {
    img_records: HashMap<String, PathBuf>,
    records: Vec<ImgRecords>,
    taxa: HashSet<String>,
}

impl Organizer {
    pub fn new() -> Self {
        Self {
            img_records: HashMap::default(),
            records: Vec::new(),
            taxa: HashSet::default(),
        }
    }

    pub fn organize(&mut self, input_dir: &Path, cfg_path: &Path, output_dir: &Path) {
        let img_paths = Finder::new(input_dir).scan_directory();
        self.parse_config_csv(cfg_path);
        self.parse_img_records();
        self.parse_taxa();
        self.print_input(input_dir, &cfg_path, &img_paths);
        self.organize_by_taxa(&img_paths, output_dir);
    }

    fn organize_by_taxa(&self, img_paths: &[PathBuf], output_dir: &Path) {
        let spin = spinner::set_spinner();
        let mut counts = 0;
        let mut skipped = 0;
        spin.set_message("Organizing images by taxa");

        img_paths.iter().for_each(|img_path| {
            let output = match self.img_records.get(
                &img_path
                    .file_stem()
                    .expect("Error passing file stem")
                    .to_string_lossy()
                    .to_uppercase(),
            ) {
                Some(path) => Path::new(path),
                None => Path::new("unknown"),
            };

            let output_path = output_dir
                .join(output)
                .join(img_path.file_name().expect("Failed parsing filenames"));
            fs::create_dir_all(output_path.parent().expect("Could not get parent path"))
                .expect("Could not create directory");
            let options = CopyOptions::new();
            let handle = |process_info: TransitProcess| {
                spin.set_message(format!(
                    "Organizing images by taxa. Byte moved {}",
                    process_info.total_bytes
                ));
            };

            match move_file_with_progress(&img_path, &output_path, &options, handle) {
                Ok(_) => {
                    let dir = img_path.parent().expect("Could not get parent path");
                    match fs::remove_dir(dir) {
                        Ok(_) => (),
                        Err(e) => {
                            log::info!(
                                "Failed removing original directory for {}: {}",
                                dir.display(),
                                e
                            )
                        }
                    }

                    counts += 1;
                }
                Err(e) => {
                    spin.set_message(format!(
                        "Could not move image: {}. Error: {}",
                        output_path.display(),
                        e,
                    ));
                    skipped += 1;
                    return;
                }
            }
        });

        spin.finish_with_message("Done organizing images by taxa!");

        log::info!("\nMoved {} images", counts);
        log::info!("Skipped {} images", skipped);
    }

    fn parse_taxa(&mut self) {
        self.records.iter().for_each(|record| {
            self.taxa.insert(record.scientific_id.clone());
        });
    }

    fn parse_img_records(&mut self) {
        self.records.iter().for_each(|rec| {
            let taxon_name = rec.scientific_id.trim().replace(" ", "_").replace(".", "");
            let img_id = Path::new(rec.image_id.trim());
            let img_name = match img_id.file_stem() {
                Some(name) => name.to_string_lossy().to_uppercase().to_string(),
                None => img_id.to_string_lossy().to_uppercase().to_string(),
            };

            let path = Path::new(&taxon_name)
                .join(&rec.locality_id)
                .join(&rec.station);
            self.img_records.insert(img_name, path);
        })
    }

    fn parse_config_csv(&mut self, cfg_path: &Path) {
        let mut reader = Reader::from_path(cfg_path).expect("Failed to read config file");

        reader.deserialize().for_each(|record| {
            let recs: ImgRecords = record.expect("Failed to deserialize record");
            self.records.push(recs);
        });
    }

    fn print_input(&self, input_dir: &Path, cfg_path: &Path, img_paths: &[PathBuf]) {
        log::info!("{}", "Input".yellow().bold());
        log::info!("{:18}: {}", "Config file", cfg_path.display());
        log::info!("{:18}: {}", "Input directory", input_dir.display());
        log::info!("{:18}: {}", "File counts", img_paths.len());
        log::info!("{:18}: {}", "Record counts", self.records.len());
        log::info!("{:18}: {}", "Taxon counts", self.taxa.len());
    }
}

#[derive(Debug, Deserialize)]
struct ImgRecords {
    locality_id: String,
    station: String,
    image_id: String,
    scientific_id: String,
}
