use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            path: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing the errors {err:?}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application Error: {e:?}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    println!("{contents:?}");
    Ok(())
}

// fn parse_config(args: &[String]) -> Config {
//     Config {
//         query: args[1].clone(),
//         path: args[2].clone(),
//     }
// }
