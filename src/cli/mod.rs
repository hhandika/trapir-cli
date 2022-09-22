mod args;

use std::io::Result;
use std::path::Path;

use clap::ArgMatches;
use exif::{In, Tag};
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::handler::summary::Summary;
use crate::image::finder::Finder;
use crate::io::files;

const LOG_FILE: &str = "trapir.log";

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
    setup_logger().expect("Could not setup logger");
    match args.subcommand() {
        Some(("image", img_matches)) => ImageCli::new(img_matches).process(),
        Some(("organize", org_matches)) => OrganizerCli::new(org_matches).process(),
        Some(("summarize", sum_matches)) => SummaryCli::new(sum_matches).summarize(),
        _ => unreachable!("Unknown subcommand"),
    }
}

struct SummaryCli<'a> {
    matches: &'a ArgMatches,
}

impl<'a> SummaryCli<'a> {
    fn new(matches: &'a ArgMatches) -> Self {
        Self { matches }
    }

    fn summarize(&self) {
        let input = self.matches.value_of("dir").expect("No directory provided");
        let image_paths = Finder::new(Path::new(input)).scan_directory();
        Summary::new(&image_paths).summarize();
    }
}

struct OrganizerCli<'a> {
    matches: &'a ArgMatches,
}

impl<'a> OrganizerCli<'a> {
    fn new(matches: &'a ArgMatches) -> Self {
        Self { matches }
    }

    fn process(&self) {
        let input = self.matches.value_of("dir").expect("No directory provided");
        println!("Input directory: {}", input);
        // let mut ext_found = Vec::new();
        // // WalkDir::new(input)
        // //     .into_iter()
        // //     .filter_map(|ok| ok.ok())
        // //     .filter(|entry| entry.file_type().is_file())
        // //     .for_each(|file| {
        // //         let ext = match file.path().extension() {
        // //             Some(ext) => ext.to_string_lossy().to_string(),
        // //             None => return,
        // //         };
        // //         if match_extension(&ext) {
        // //             ext_found.push(ext);
        // //         }
        // //     });

        // ext_found.sort();
        // ext_found.dedup();
        // println!("Found {} matched extensions", ext_found.len());
        // ext_found.iter().for_each(|ext| println!("{}", ext));
    }
}

struct ImageCli<'a> {
    matches: &'a ArgMatches,
}

impl<'a> ImageCli<'a> {
    pub fn new(matches: &'a ArgMatches) -> ImageCli<'a> {
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

fn setup_logger() -> Result<()> {
    let log_dir = std::env::current_dir()?;
    let target = log_dir.join(LOG_FILE);
    let tofile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)} - {l} - {m}\n",
        )))
        .build(target)?;

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("logfile", Box::new(tofile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(LevelFilter::Info),
        )
        .expect("Failed building log configuration");

    log4rs::init_config(config).expect("Cannot initiate log configuration");

    Ok(())
}
