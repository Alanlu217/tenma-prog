use serial2::SerialPort;

pub mod tenma_commands;

use tenma_commands::Commands;

pub struct TenmaSerial {
    port: SerialPort,
}

impl TenmaSerial {
    pub fn new(port: &str) -> Result<Self, std::io::Error> {
        let port = SerialPort::open(port, 9600)?;

        Ok(TenmaSerial { port })
    }

    pub fn run_command(&self, cmd: Commands) {
        match cmd {
            Commands::ISet { channel, current } => {
                let _ = self.port.write(format!("ISET{}:{}", channel, current).as_bytes());
            }
            Commands::VSet { channel, voltage } => {
                let _ = self.port.write(format!("VSET{}:{}", channel, voltage).as_bytes());
            }
            Commands::Beep(on) => {
                let ch = if on { '1' } else { '0' };
                let _ = self.port.write(format!("BEEP{}", ch).as_bytes());
            }
        }
    }
}
