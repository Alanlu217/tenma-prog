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
    fn run_command(&self, cmd: TenmaCommand) {
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
        }
    }
}
