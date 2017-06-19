extern crate clap;

const SQUARS_NUMBER_NAME: &'static str = "squars";

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
        .subcommand(clap::SubCommand::with_name("genetic")
            .about("runs the genetic algorithm")
            .version("1.0")
            .author("Fatemeh Rangraz Jeddi")
            .arg(clap::Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .subcommand(clap::SubCommand::with_name("hill-climbing")
            .about("runs the hill-climbing algorithm")
            .version("1.0")
            .author("Fatemeh Rangraz Jeddi")
            .arg(clap::Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .get_matches();
    let squars: u32 = matches.value_of(SQUARS_NUMBER_NAME).unwrap_or("8").parse().unwrap_or(8);
}
