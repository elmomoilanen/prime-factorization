//! Command line argument parser.
//!
//! Expected to receive positive natural number that can be parsed to 128 bit unsigned integer.
//!

pub fn parse_arguments(args: &[String]) -> Result<u128, &str> {
    match args.len() {
        0 => Err("No argument, nothing to factorize."),
        1 => {
            if &args[0] == "--help" || &args[0] == "-h" {
                show_help();
                return Err("help");
            }

            match (*args[0]).parse::<u128>() {
                Ok(num) => Ok(num),
                Err(_) => {
                    let mut arg = String::from(&args[0]);
                    arg.retain(|c| c != '_');

                    if let Ok(num) = arg.parse::<u128>() {
                        Ok(num)
                    } else {
                        Err("Cannot parse argument to 128 bit unsigned integer.")
                    }
                }
            }
        }
        _ => Err("Unable to parse multiple arguments, either pass only the number or `--help`."),
    }
}

fn show_help() {
    println!("\nusage: prime_factorization <num; positive integer>");
}
