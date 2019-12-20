use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config
{
    pub query: String,
    pub filename: String
}

impl Config{
    pub fn new(args:&[String]) -> Result<Config, &'static str>
    {
        if args.len() < 3{
            return Err("Not Enough Arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config{query, filename});
    }
}

pub fn run(config:Config) -> Result<(), Box< dyn Error>>
{
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed");
    println!{"{}", contents};
    return Ok(());
}