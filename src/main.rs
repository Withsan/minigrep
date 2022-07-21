use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("fuck:{}", err);
        process::exit(1);
    });
    run(config).unwrap_or_else(|err| {
        println!("fuck too:{}", err);
        process::exit(1);
    })
}


