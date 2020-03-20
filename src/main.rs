use std::{env, env::Args};
use std::string::String;
use std::fs;
use std::process;
use std::num::ParseIntError;

fn main() {
    let mut args: Args = env::args();
    args.next();

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Invalid argurments: {}", err);
        process::exit(1);
    });

    if config.filename.is_some() {
        println!("{}", config.filename.as_ref().unwrap());
    }
    println!("{:?}", config.seconds);
}

struct Config {
    seconds: u32,
    filename: Option<String>,
}

impl Config {
    fn new(mut args: Args) -> Result<Config, &'static str> {
        let mut time_str = String::new();

        loop {
            let arg = args.next();
            if arg.is_none() {
                break;
            }

            let arg = arg.unwrap();
            if arg == "-f" {
                return read_file_flag(&mut args);
            } else {
                time_str = arg;
            }
        }

        let seconds = parse_int(&time_str);

        if seconds.is_err() {
            return Err(seconds.err().unwrap());
        }
        let seconds = seconds.unwrap();

        Ok(Config { filename: None, seconds })
    }
}

fn read_file_flag(args: &mut Args) -> Result<Config, &'static str> {
    let path = args.next();
    
    if path.is_none() {
        return Err("Must Provide a filename with the '-f' flag");
    }
    let path = path.unwrap();
    let filename = Some(path.clone());
    
    let time_str = fs::read_to_string(path).unwrap();
    let seconds = parse_int(time_str.trim());
    if seconds.is_err() {
        return Err(seconds.err().unwrap());
    }
    let seconds = seconds.unwrap();

    Ok(Config { seconds, filename })
}

fn parse_int(input_str: &str) -> Result<u32, &'static str> {
    let seconds: Result<u32, ParseIntError> = input_str.parse();
    if seconds.is_err() {
        return Err("Must provide a positive integer");
    }
    Ok(seconds.unwrap())
}