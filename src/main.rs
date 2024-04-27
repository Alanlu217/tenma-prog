mod config;
mod lua_script;
mod tenma;

use std::process::{self, exit};
use std::{env, fs};

use tenma::tenma_serial;

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

    // let script = TenmaScript::open(config.file_path.as_str(), serial);

    // println!("{}", script.unwrap());
    // script.unwrap().run_script();

    let script = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    let lua = lua_script::LuaScript::new(script.as_str(), Box::new(serial)).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    lua.run().unwrap_or_else(|err| {
        println!("{err}");
    });
}
