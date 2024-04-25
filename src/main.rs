mod config;
mod scripts;
mod tenma_serial;

use scripts::TenmaScript;

use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect();

    let config = config::Config::from_args(&args).unwrap_or_else(|err| {
        println!("Could not load arguments: {}", err);
        exit(1);
    });

    let serial = tenma_serial::TenmaSerial::new(&config.port).unwrap_or_else(|err| {
        println!("Could not open serial port: {}", err);
        exit(1);
    });

    let script = TenmaScript::open(config.file_path.as_str(), serial);

    // println!("{}", script.unwrap());
    script.unwrap().run_script();
}
