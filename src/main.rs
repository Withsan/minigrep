use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("fuck:{}", err);
        process::exit(1);
    });
    run(config).unwrap_or_else(|err| {
        eprintln!("fuck too:{}", err);
        process::exit(1);
    })
}


