extern crate VGrep;
use std::env;
use std::process;
use VGrep::Config;

fn main()
{
    let args:Vec<String>=env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem in Parsing {}: ", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In File {}", config.filename);

    VGrep::run(config).unwrap_or_else(|err|{
        println!("Application Error {}: ", err);
        process::exit(1);
    });
}