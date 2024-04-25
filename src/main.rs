mod config;

use std::env;

fn main() {
    let args = env::args().collect();

    let config = config::Config::from_args(&args);

    dbg!("{}", config.unwrap());
}
