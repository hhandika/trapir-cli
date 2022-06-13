mod args;

use clap::ArgMatches;

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
        let dir = self
            .matches
            .value_of("dir")
            .expect("Failed parsing dir input");
        println!("Processing image: {}", dir);
    }
}
