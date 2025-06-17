use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments (needs at least 2)");
        }

        let query = args[1].clone();
        let path = args[2].clone();

        Ok(Config { query, path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(e) = run(config) {
        process::exit(1);
    }
}
