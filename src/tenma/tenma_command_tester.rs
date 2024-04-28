use super::tenma_commands::{command_to_str, TenmaCommand, TenmaCommandTrait};

pub struct TenmaTester;

impl TenmaCommandTrait for TenmaTester {
    fn run_command(&self, cmd: TenmaCommand) -> Option<f64> {
        println!("{}", command_to_str(&cmd));

        match cmd {
            TenmaCommand::IGet { channel: _ } => Some(0.0),
            TenmaCommand::VGet { channel: _ } => Some(0.0),
            _ => None,
        }
    }
}
