mod config;
mod tenma_serial;

use tenma_serial::tenma_commands::Commands;

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

    serial.run_command(Commands::VSet { channel: 1, voltage: 10.0 })
}
