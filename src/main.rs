use std::env;
use std::process;

use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem in parsing aurgument: {}", err);
        process::exit(1);
    });

    println!("{}",config.query);
    println!("{}",config.filename);

    if let Err(e) = mini_grep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}