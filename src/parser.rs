//! Command line argument parser.
//!
//! Expected to receive a positive natural number that can be parsed to a 128
//! bit unsigned integer. Numbers are allowed to contain underscores, e.g. 10_000
//! would be accepted and interpreted as 10000. Parser checks also whether a
//! "pretty" print option `--pretty` or `-p` is included in the arguments.
//!

pub fn parse_arguments(args: &mut [String]) -> Result<(u128, bool), &str> {
    match args.len() {
        0 => Err("no argument, nothing to factorize."),
        1 if &args[0] == "--help" || &args[0] == "-h" => {
            show_help();
            Err("help")
        },
        1 => match parse_to_int(&mut args[0]) {
            Some(num) => Ok((num, false)),
            _ => Err("cannot parse argument to a 128 bit unsigned integer."),
        },
        2 if &args[0] == "--pretty" || &args[0] == "-p" => {
            match parse_to_int(&mut args[1]) {
                Some(num) => Ok((num, true)),
                _ => Err("cannot parse 2nd argument to a 128 bit unsigned integer."),
            }
        },
        2 if &args[1] == "--pretty" || &args[1] == "-p" => {
            match parse_to_int(&mut args[0]) {
                Some(num) => Ok((num, true)),
                _ => Err("cannot parse 1st argument to a 128 bit unsigned integer."),
            }
        },
        _ => Err(
            "unable to parse args, please check instructions or pass `--help` as the only argument."
        ),
    }
}

fn parse_to_int(arg: &mut String) -> Option<u128> {
    match arg.parse::<u128>() {
        Ok(num) => Some(num),
        Err(_) => {
            arg.retain(|c| c != '_');

            if let Ok(num) = arg.parse::<u128>() {
                Some(num)
            } else {
                None
            }
        }
    }
}

fn show_help() {
    println!(
        "Decompose a natural number to its prime factors\n\nUSAGE:\n   ./target/release/prime_factorization <num; positive integer>"
    );
}
