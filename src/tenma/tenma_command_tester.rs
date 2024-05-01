use super::tenma_commands::{command_to_str, TenmaCommand, TenmaCommandTrait};

pub struct TenmaTester {
    pub port: usize,
}

impl TenmaCommandTrait for TenmaTester {
    fn run_command(&self, cmd: TenmaCommand) -> Option<f64> {
        println!("{} on Port {}", command_to_str(&cmd), self.port + 1);

        match cmd {
            TenmaCommand::IGet { channel: _ } => Some(0.0),
            TenmaCommand::VGet { channel: _ } => Some(0.0),
            _ => None,
        }
    }
}
