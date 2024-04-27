mod config;
mod lua_script;
mod tenma;

use std::process::{self, exit};
use std::{env, fs};

use tenma::tenma_commands::TenmaCommandTrait;
use tenma::{tenma_command_tester, tenma_serial};

fn main() {
    let args = env::args().collect();

    let config = config::Config::from_args(&args).unwrap_or_else(|err| {
        println!("Could not load arguments: {}", err);
        exit(1);
    });

    let command_runner: Box<dyn TenmaCommandTrait> = match config.port {
        Some(port) => Box::new(tenma_serial::TenmaSerial::new(&port).unwrap_or_else(|err| {
            println!("Could not open serial port: {}", err);
            exit(1);
        })),
        None => Box::new(tenma_command_tester::TenmaTester {}),
    };

    let script = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    let lua = lua_script::LuaScript::new(script.as_str(), command_runner).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    lua.run().unwrap_or_else(|err| {
        println!("{err}");
    });
}
