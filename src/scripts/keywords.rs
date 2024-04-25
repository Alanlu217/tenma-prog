use core::time;

pub const CURRENT_KEY: &str = "I";

pub const VOLTAGE_KEY: &str = "V";

pub const OFF_KEY: &str = "OFF";

pub const DELAY_KEY: &str = ":";

pub const LOOP_START_KEY: &str = "loop";

pub const LOOP_END_KEY: &str = "end";

pub fn delay_unit_to_duration(unit: &str) -> Result<time::Duration, String> {
    match unit {
        "min" => Ok(time::Duration::from_secs(60)),
        "s" => Ok(time::Duration::from_secs(1)),
        "ms" => Ok(time::Duration::from_millis(1)),
        _ => Err(format!("{} is not a valid unit", unit)),
    }
}

#[derive(Clone)]
pub enum TenmaScriptCommand {
    I {
        current: i32,
    },
    V {
        voltage: i32,
    },
    Off,
    Delay {
        milliseconds: u32,
    },
}
