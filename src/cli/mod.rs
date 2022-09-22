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

use crate::handler::organizer::Organizer;
use crate::handler::summary::Summary;
use crate::io::finder::Finder;

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
        Some(("organize", org_matches)) => OrganizerCli::new(org_matches).organize(),
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

    fn organize(&self) {
        let input = self
            .matches
            .value_of("input")
            .expect("Failed parsing a config file");
        let dir = self
            .matches
            .value_of("dir")
            .expect("Failed parsing a config file");
        let output = self.matches.value_of("output").unwrap_or(dir);
        Organizer::new().organize(Path::new(dir), Path::new(input), Path::new(output));
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

        let images = Finder::new(dir).find_jpeg();
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
