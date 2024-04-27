use super::tenma_commands::{command_to_str, TenmaCommand, TenmaCommandTrait};

pub struct TenmaTester;

impl TenmaCommandTrait for TenmaTester {
    fn run_command(&self, cmd: TenmaCommand) {
        println!("{}", command_to_str(cmd));
    }
}
