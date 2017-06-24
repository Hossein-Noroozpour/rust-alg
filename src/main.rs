extern crate clap;

mod genetic;
mod hillclimbing;

const SQUARS_NUMBER_NAME: &'static str = "squars";
const GENETIC_NAME: &'static str = "genetic";
const HILL_NAME: &'static str = "hill-climbing";
const WISE_NAME: &'static str = "wise";
const TEMP_REDUCTION_NAME: &'static str = "temp-rate";

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
                .long(WISE_NAME)
                .help("Use wise approach in genetic algorithm.")))
        .subcommand(clap::SubCommand::with_name(HILL_NAME)
            .about("runs the hill-climbing algorithm")
            .version("1.0")
            .author("Fatemeh Rangraz Jeddi")
            .arg(clap::Arg::with_name(TEMP_REDUCTION_NAME)
                .short("t")
                .long(TEMP_REDUCTION_NAME)
                .help("Specify the temporaure reduction ratio of the algorithm.")))
        .get_matches();
    let squars: i32 = matches.value_of(SQUARS_NUMBER_NAME).unwrap_or("8").parse().unwrap_or(8);
    if let Some(matches) = matches.subcommand_matches(HILL_NAME) {
        let tepm_rate: f64 = matches.value_of(TEMP_REDUCTION_NAME)
            .unwrap_or("0.9999990").parse().unwrap_or(0.999f64);
        hillclimbing::execute(squars as u64, squars as u64, tepm_rate);
    } else if let Some(matches) = matches.subcommand_matches(GENETIC_NAME) {
        let wise_genetic: bool = matches.value_of(WISE_NAME)
            .unwrap_or("false").parse().unwrap_or(false);
        if !wise_genetic {
            genetic::execute(squars);
        }
    }

}
