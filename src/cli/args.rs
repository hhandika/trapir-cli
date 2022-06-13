use clap::{crate_description, crate_name, Arg, ArgMatches, Command};

pub fn get_args(version: &str) -> ArgMatches {
    Command::new(crate_name!())
        .version(version)
        .about(crate_description!())
        .author("Heru Handika")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("image").about("Get image metadata").arg(
                Arg::new("dir")
                    .short('d')
                    .long("dir")
                    .help("Directory to scan")
                    .takes_value(true)
                    .required(true)
                    .value_name("DIR"),
            ),
        )
        .get_matches()
}
