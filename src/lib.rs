use std::string::String;
use std::fs;
use std::error::Error;
use std::thread;
use std::time::{SystemTime, Duration};
use std::io;
use std::io::ErrorKind;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(filename) = config.filename {
        println!("Reading from {}", filename);
    }

    let total_time = Duration::new(config.seconds, 0);
    println!("{:?}", total_time);
    

    let start = SystemTime::now();
    for i in 0..config.seconds {
        let seconds_left = config.seconds - i;
        println!("{}", String::from("#").repeat(seconds_left as usize));

        let wait_time = Duration::new(i + 1, 0);
        let elapsed = start.elapsed()?;
        if elapsed < wait_time {
            thread::sleep(wait_time - start.elapsed()?);
        }
    }

    Ok(())
}

pub struct Config {
    seconds: u64,
    filename: Option<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, Box<dyn Error>> {
        let mut time_str = String::from("0");
        let mut args = args.iter();

        loop {
            let arg = args.next();
            if arg.is_none() {
                break;
            }

            let arg = arg.unwrap();
            if arg == "-f" {
                return read_file_flag(args.next());
            } else {
                time_str = arg.clone();
            }
        }

        let seconds = parse_int(&time_str)?;

        Ok(Config { filename: None, seconds })
    }
}

fn read_file_flag(filename: Option<&String>) -> Result<Config, Box<dyn Error>> {
    let result: Result<Config, Box<dyn Error>>;

    if let Some(path) = filename {
      let filename = Some(path.clone());
    
      let time_str = fs::read_to_string(path)?;
      let seconds = parse_int(time_str.trim())?;

      result = Ok(Config { seconds, filename })
    } else {
        result = Err(Box::new(io::Error::new(ErrorKind::NotFound, "Filename must not be empty")))
    }

    result
}

fn parse_int(input_str: &str) -> Result<u64, Box<dyn Error>> {
    Ok(input_str.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_single_number() {
        let args: Vec<String> = vec![String::from("5")];
        let config = Config::new(args).unwrap();
        assert_eq!(config.seconds, 5);

        assert_eq!(config.filename, None);
    }
}