mod keywords;
mod tenma_script_parser;

use core::time;
use std::{ fmt::Display, fs, iter::Peekable, thread };

use crate::tenma_serial::{ tenma_commands::TenmaCommand, TenmaSerial };

use self::{
    keywords::TenmaScriptCommand,
    tenma_script_parser::{
        parse_current,
        parse_delay,
        parse_loop_start,
        parse_voltage,
        ParseError,
    },
};

pub struct TenmaScript {
    contents: Vec<TenmaScriptCommand>,
    output: TenmaSerial,
}

impl TenmaScript {
    pub fn open(path: &str, output: TenmaSerial) -> Result<Self, std::io::Error> {
        let file_string = fs::read_to_string(path)?;
        let mut tokens = file_string
            .split(['\n', '\r', ' '])
            .filter(|x| x.len() > 0)
            .map(|x| x.to_string())
            .peekable();

        let result = Self::parse_file(&mut tokens);

        Ok(TenmaScript {
            contents: result.unwrap(),
            output,
        })
    }

    fn parse_file(
        tokens: &mut Peekable<impl Iterator<Item = String>>
    ) -> Result<Vec<TenmaScriptCommand>, ParseError> {
        fn internal(
            tokens: &mut Peekable<impl Iterator<Item = String>>,
            is_loop: bool
        ) -> Result<Vec<TenmaScriptCommand>, ParseError> {
            let mut x: Vec<TenmaScriptCommand> = vec![];
            loop {
                match tokens.next() {
                    Some(s) => {
                        match s.as_str() {
                            keywords::VOLTAGE_KEY => {
                                x.push(parse_voltage(tokens)?);
                                if let Some(s) = tokens.peek() {
                                    if s != keywords::DELAY_KEY {
                                        x.push(TenmaScriptCommand::Delay { milliseconds: 50 });
                                    }
                                }
                            }
                            keywords::CURRENT_KEY => {
                                x.push(parse_current(tokens)?);
                                if let Some(s) = tokens.peek() {
                                    if s != keywords::DELAY_KEY {
                                        x.push(TenmaScriptCommand::Delay { milliseconds: 50 });
                                    }
                                }
                            }
                            keywords::ON_KEY => {
                                x.push(TenmaScriptCommand::On);
                                x.push(TenmaScriptCommand::Delay { milliseconds: 50 });
                            }
                            keywords::OFF_KEY => {
                                x.push(TenmaScriptCommand::Off);
                                x.push(TenmaScriptCommand::Delay { milliseconds: 50 });
                            }
                            keywords::DELAY_KEY => {
                                x.push(parse_delay(tokens)?);
                            }
                            keywords::LOOP_START_KEY => {
                                let times = parse_loop_start(tokens)?;

                                let commands = internal(tokens, true)?;
                                for _ in 0..times {
                                    for command in commands.iter() {
                                        x.push(command.clone());
                                    }
                                }
                            }
                            keywords::LOOP_END_KEY => {
                                return Ok(x);
                            }

                            _ => {
                                return Err(ParseError::InvalidSymbol { symbol: s });
                            }
                        }
                    }
                    None => {
                        if is_loop {
                            return Err(ParseError::LoopEndNotFound);
                        }
                        return Ok(x);
                    }
                }
            }
        }

        internal(tokens, false)
    }

    pub fn run_script(&self) {
        for command in self.contents.iter() {
            println!("{}", TenmaScript::command_to_string(command));

            match command {
                TenmaScriptCommand::I { current } =>
                    self.output.run_command(TenmaCommand::ISet { channel: 1, current: *current }),
                TenmaScriptCommand::V { voltage } =>
                    self.output.run_command(TenmaCommand::VSet { channel: 1, voltage: *voltage }),
                TenmaScriptCommand::On => self.output.run_command(TenmaCommand::Out(true)),
                TenmaScriptCommand::Off => self.output.run_command(TenmaCommand::Out(false)),
                TenmaScriptCommand::Delay { milliseconds } => {
                    thread::sleep(time::Duration::from_millis(*milliseconds));
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn display_tenma_scripts(&self) {
        println!("{}", self.to_string());
    }

    pub fn command_to_string(command: &TenmaScriptCommand) -> String {
        match command {
            TenmaScriptCommand::I { current } => {
                return format!("Set current to: {}\n", current);
            }
            TenmaScriptCommand::V { voltage } => {
                return format!("Set voltage to: {}\n", voltage);
            }
            TenmaScriptCommand::On => {
                return format!("Output On\n");
            }
            TenmaScriptCommand::Off => {
                return format!("Output Off\n");
            }
            TenmaScriptCommand::Delay { milliseconds } => {
                return format!("Delay for: {} ms\n", milliseconds);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        for line in self.contents.iter() {
            out.push_str(TenmaScript::command_to_string(line).as_str());
        }

        out
    }
}

impl Display for TenmaScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn test() {
    println!("\n\n\n\n-----------------------------STDOUT----------------------------\n\n\n");
    let x = TenmaScript::open(
        "tenma_scripts/test.tms",
        TenmaSerial::new("/dev/tty.Bluetooth-Incoming-Port").unwrap()
    );

    x.unwrap().display_tenma_scripts();

    println!("\n\n\n\n-----------------------------STDOUT----------------------------\n\n\n");
    panic!()
}
