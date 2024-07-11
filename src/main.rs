use std::{env, process};
use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsin arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = grep::run(config){
        println!("Application error: {e}");
        process::exit(1);
    }
}
