pub enum TenmaCommand {
    ISet {
        channel: u8,
        current: f64,
    },
    VSet {
        channel: u8,
        voltage: f64,
    },
    #[allow(dead_code)]
    Beep(bool),
    Out(bool),
}

pub fn command_to_str(cmd: TenmaCommand) -> String {
    match cmd {
        TenmaCommand::ISet { channel, current } => {
            format!("Set Current to {current} on Channel {channel}")
        }
        TenmaCommand::VSet { channel, voltage } => {
            format!("Set Voltage to {voltage} on Channel {channel}")
        }
        TenmaCommand::Beep(x) => format!("Set Beep to {x}"),
        TenmaCommand::Out(x) => format!("Set Out to {x}"),
    }
}

pub trait TenmaCommandTrait {
    fn run_command(&self, cmd: TenmaCommand);
}
