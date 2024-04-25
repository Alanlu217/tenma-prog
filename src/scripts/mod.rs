mod keywords;
mod TenmaScriptParser;

use std::{ fmt::Display, fs, iter::Peekable };

use crate::tenma_serial::TenmaSerial;

use self::{
    keywords::TenmaScriptCommand,
    TenmaScriptParser::{ parse_current, parse_delay, parse_voltage, ParseError },
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
        let mut x: Vec<TenmaScriptCommand> = vec![];

        loop {
            match tokens.next() {
                Some(s) => {
                    match s.as_str() {
                        keywords::VOLTAGE_KEY => {
                            match parse_voltage(tokens) {
                                Ok(c) => {
                                    x.push(c);
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                        keywords::CURRENT_KEY => {
                            match parse_current(tokens) {
                                Ok(c) => {
                                    x.push(c);
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                        keywords::DELAY_KEY => {
                            match parse_delay(tokens) {
                                Ok(c) => x.push(c),
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                        _ => {
                            return Err(ParseError::InvalidSymbol { symbol: s });
                        }
                    }
                }
                None => {
                    return Ok(x);
                }
            }
        }
    }

    pub fn display_tenma_scripts(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        for line in self.contents.iter() {
            match line {
                TenmaScriptCommand::I { current } => {
                    out.push_str(format!("Set current to: {}\n", current).as_str());
                }
                TenmaScriptCommand::V { voltage } => {
                    out.push_str(format!("Set voltage to: {}\n", voltage).as_str());
                }
                TenmaScriptCommand::Off => {
                    out.push_str(format!("Set voltage to 0\n").as_str());
                }
                TenmaScriptCommand::Delay { milliseconds } => {
                    out.push_str(format!("Delay for: {} ms\n", milliseconds).as_str());
                }
            }
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
