mod args;

use std::io::Result;
use std::path::Path;

use clap::ArgMatches;
use exif::{In, Tag};

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
        _ => unreachable!("Unknown subcommand"),
    }
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
