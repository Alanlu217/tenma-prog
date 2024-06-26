mod config;
mod lua_script;
mod tenma;
mod util;

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

    let mut command_runners: Vec<Box<dyn TenmaCommandTrait>> = vec![];

    for (idx, port) in config.ports.iter().enumerate() {
        match port {
            Some(s) => command_runners.push(Box::new(
                tenma_serial::TenmaSerial::new(&s).unwrap_or_else(|err| {
                    println!("Could not open serial port: {}", err);
                    exit(1);
                }),
            )),
            None => command_runners.push(Box::new(tenma_command_tester::TenmaTester { port: idx })),
        }
    }

    let script = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    let lua = lua_script::LuaScript::new(script.as_str(), command_runners).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    lua.run().unwrap_or_else(|err| {
        println!("{err}");
    });
}
