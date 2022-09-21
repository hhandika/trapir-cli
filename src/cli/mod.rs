mod args;

use std::io::Result;
use std::path::Path;

use clap::ArgMatches;
use exif::{In, Tag};
use lazy_static::lazy_static;
use regex::Regex;
use walkdir::WalkDir;

use crate::io::files;

macro_rules! print_exif {
    ($exif: ident, $name: ident, $tag: ident, $msg: expr) => {
        let $name = $exif.get_field(Tag::$tag, In::PRIMARY);
        match $name {
            Some($name) => println!("{}", $name.display_value().with_unit(&$exif)),
            None => println!($msg),
        }
    };
}

pub fn parse_args(version: &str) {
    let args = args::get_args(version);
    match args.subcommand() {
        Some(("image", img_matches)) => ImageProcessor::new(img_matches).process(),
        Some(("organize", org_matches)) => FileOrganizer::new(org_matches).process(),
        _ => unreachable!("Unknown subcommand"),
    }
}

struct FileOrganizer<'a> {
    matches: &'a ArgMatches,
}

impl<'a> FileOrganizer<'a> {
    fn new(matches: &'a ArgMatches) -> Self {
        Self { matches }
    }

    fn process(&self) {
        let input = self.matches.value_of("dir").expect("No directory provided");
        let mut ext_found = Vec::new();
        WalkDir::new(input)
            .into_iter()
            .filter_map(|ok| ok.ok())
            .filter(|entry| entry.file_type().is_file())
            .for_each(|file| {
                let ext = match file.path().extension() {
                    Some(ext) => ext.to_string_lossy().to_string(),
                    None => return,
                };
                if match_extension(&ext) {
                    ext_found.push(ext);
                }
            });

        ext_found.sort();
        ext_found.dedup();
        println!("Found {} matched extensions", ext_found.len());
        ext_found.iter().for_each(|ext| println!("{}", ext));
    }
}

fn match_extension(text: &str) -> bool {
    lazy_static! { // Match the first word in the block
        static ref RE: Regex = Regex::new(r"^((?i)jpg|jpeg|avi|m4a|m4v|mp4)").expect("Failed capturing file extension");
    }

    RE.is_match(text)
}

struct ImageProcessor<'a> {
    matches: &'a ArgMatches,
}

impl<'a> ImageProcessor<'a> {
    pub fn new(matches: &'a ArgMatches) -> ImageProcessor<'a> {
        Self { matches: matches }
    }

    pub fn process(&self) {
        let dir = Path::new(
            self.matches
                .value_of("dir")
                .expect("Failed parsing dir input"),
        );

        let images = files::find_images(dir);
        images.iter().for_each(|image| {
            println!("{}", image.display());
            // self.print_file_metadata(image)
            //     .expect("Failed printing file metadata");
            self.print_exif(image)
                .expect("Failed printing exif metadata");
        });
    }

    // fn print_file_metadata(&self, file: &Path) -> Result<()> {
    //
    //     let metadata = fs::metadata(file)?;
    //     println!("Filetype: {:?}", metadata.file_type());
    //     Ok(())
    // }

    fn print_exif(&self, file: &Path) -> Result<()> {
        let file = std::fs::File::open(file)?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader
            .read_from_container(&mut bufreader)
            .expect("Failed reading exif");
        // for f in exif.fields() {
        //     println!("{}", f.display_value().with_unit(&exif));
        // }

        print_exif!(exif, date, DateTime, "Failed reading width from exif");
        print_exif!(
            exif,
            temperature,
            Temperature,
            "Failed reading temperature from exif"
        );
        print_exif!(
            exif,
            latitude,
            GPSLatitude,
            "Failed reading latitude from exif"
        );
        print_exif!(
            exif,
            longitude,
            GPSLongitude,
            "Failed reading longitude from exif"
        );
        print_exif!(
            exif,
            altitude,
            GPSAltitude,
            "Failed reading altitude from exif"
        );

        Ok(())
    }
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
