use clap::{crate_description, crate_name, Arg, ArgMatches, Command};

pub fn get_args(version: &str) -> ArgMatches {
    Command::new(crate_name!())
        .version(version)
        .about(crate_description!())
        .author("Heru Handika")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("metadata").about("Parse image metadata").arg(
                Arg::new("dir")
                    .short('d')
                    .long("dir")
                    .help("Directory to scan")
                    .takes_value(true)
                    .required(true)
                    .value_name("DIR"),
            ),
        )
        .subcommand(
            Command::new("organize")
                .about("Organize image based species name")
                .arg(
                    Arg::new("dir")
                        .short('d')
                        .long("dir")
                        .help("Directory to scan")
                        .takes_value(true)
                        .required(true)
                        .value_name("DIR"),
                )
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .help("Input a config file")
                        .takes_value(true)
                        .required(true)
                        .value_name("DIR"),
                ),
        )
        .subcommand(
            Command::new("summarize")
                .about("Summarize all camera traps images recursively in a directory")
                .arg(
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
