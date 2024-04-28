use std::{io::Read, num::ParseFloatError};

use serial2::SerialPort;

use super::tenma_commands::{TenmaCommand, TenmaCommandTrait};

pub struct TenmaSerial {
    port: SerialPort,
}

impl TenmaSerial {
    pub fn new(port: &str) -> Result<Self, std::io::Error> {
        let port = SerialPort::open(port, 9600)?;

        Ok(TenmaSerial { port })
    }
}

impl TenmaCommandTrait for TenmaSerial {
    fn run_command(&self, cmd: TenmaCommand) -> Option<f64> {
        match cmd {
            TenmaCommand::ISet { channel, current } => {
                let _ = self
                    .port
                    .write(format!("ISET{}:{}", channel, current).as_bytes());
            }
            TenmaCommand::VSet { channel, voltage } => {
                let _ = self
                    .port
                    .write(format!("VSET{}:{}", channel, voltage).as_bytes());
            }
            TenmaCommand::Beep(on) => {
                let ch = if on { '1' } else { '0' };
                let _ = self.port.write(format!("BEEP{}", ch).as_bytes());
            }
            TenmaCommand::Out(on) => {
                let ch = if on { '1' } else { '0' };
                let _ = self.port.write(format!("OUT{}", ch).as_bytes());
            }
            TenmaCommand::IGet { channel } => {
                let _ = self.port.write(format!("IOUT{channel}?").as_bytes());

                let mut buffer: [u8; 5] = [0; 5];
                if let Err(err) = self.port.read_exact(&mut buffer) {
                    println!("{err}");
                    let _ = self.port.discard_input_buffer();
                    return None;
                }
                let _ = self.port.discard_input_buffer();

                if let Ok(num) = parse_buffer(buffer.into()) {
                    return Some(num);
                }
            }
            TenmaCommand::VGet { channel } => {
                let _ = self.port.write(format!("VOUT{channel}?").as_bytes());

                let mut buffer: [u8; 5] = [0; 5];
                if let Err(err) = self.port.read_exact(&mut buffer) {
                    println!("{err}");
                    let _ = self.port.discard_input_buffer();
                    return None;
                }
                let _ = self.port.discard_input_buffer();

                if let Ok(num) = parse_buffer(buffer.into()) {
                    return Some(num);
                }
            }
        }

        None
    }
}

fn parse_buffer(buffer: Vec<u8>) -> Result<f64, ParseFloatError> {
    let mut s = String::new();
    buffer.iter().for_each(|x| s.push(*x as char));
    Ok(s.parse::<f64>()?)
}
