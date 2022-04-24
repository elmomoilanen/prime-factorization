//! Command line argument parser.
//!
//! Expected to receive a positive natural number that can be parsed to a 128 bit unsigned integer.
//! Number are allowed to contain underscores, e.g. 10_000.
//!

pub fn parse_arguments(args: &[String]) -> Result<u128, &str> {
    match args.len() {
        0 => Err("no arg, nothing to factorize."),
        1 if &args[0] == "--help" || &args[0] == "-h" => {
            show_help();
            Err("help")
        }
        1 => match (*args[0]).parse::<u128>() {
            Ok(num) => Ok(num),
            Err(_) => {
                let mut arg = String::from(&args[0]);
                arg.retain(|c| c != '_');

                if let Ok(num) = arg.parse::<u128>() {
                    Ok(num)
                } else {
                    Err("cannot parse arg to a 128 bit unsigned integer.")
                }
            }
        },
        _ => Err("cannot parse multiple args, either pass one number or `--help`."),
    }
}

fn show_help() {
    println!("Decompose natural number to its prime factors\n\nUSAGE:\n   ./prime_factorization <num; positive integer>");
}
