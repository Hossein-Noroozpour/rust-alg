extern crate clap;

mod genetic;

const SQUARS_NUMBER_NAME: &'static str = "squars";
const GENETIC_NAME: &'static str = "genetic";
const HILL_NAME: &'static str = "hill-climbing";
const WISE_NAME: &'static str = "wise";

fn main() {
    let matches = clap::App::new("Hill Gen Queens")
        .version("1.0")
        .author("Fatemeh Rangraz Jeddi")
        .about("It is a comparison between genetic and hill climbing algorithms.")
        .arg(clap::Arg::with_name(SQUARS_NUMBER_NAME)
            .short("s")
            .long("squars")
            .value_name("SQUARS")
            .help("Sets a custom number of squars")
            .takes_value(true))
        .subcommand(clap::SubCommand::with_name(GENETIC_NAME)
            .about("runs the genetic algorithm")
            .version("1.0")
            .author("Fatemeh Rangraz Jeddi")
            .arg(clap::Arg::with_name(WISE_NAME)
                .short("w")
                .long("wise")
                .help("Use wise approach in genetic algorithm.")))
        .subcommand(clap::SubCommand::with_name(HILL_NAME)
            .about("runs the hill-climbing algorithm")
            .version("1.0")
            .author("Fatemeh Rangraz Jeddi")
            .arg(clap::Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .get_matches();
    let squars: i32 = matches.value_of(SQUARS_NUMBER_NAME).unwrap_or("8").parse().unwrap_or(8);
    if let Some(matches) = matches.subcommand_matches(HILL_NAME) {

    } else if let Some(matches) = matches.subcommand_matches(GENETIC_NAME) {
        let wise_genetic: bool = matches.value_of(WISE_NAME)
            .unwrap_or("false").parse().unwrap_or(false);
        if !wise_genetic {
            genetic::execute(squars);
        }
    }

}
