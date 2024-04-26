use core::time;
use std::iter::Peekable;

use super::keywords::{ delay_unit_to_duration, TenmaScriptCommand };

#[derive(Debug)]
pub enum ParseError {
    InvalidSymbol {
        symbol: String,
    },
    LoopEndNotFound,
    IntParseError {
        symbol: String,
    },
    MissingValue,
}

pub fn parse_voltage(
    tokens: &mut impl Iterator<Item = String>
) -> Result<TenmaScriptCommand, ParseError> {
    match tokens.next() {
        Some(s) => {
            if let Ok(num) = s.parse::<f32>() {
                return Ok(TenmaScriptCommand::V { voltage: num });
            } else {
                return Err(ParseError::IntParseError { symbol: s });
            }
        }
        None => {
            return Err(ParseError::MissingValue);
        }
    }
}

pub fn parse_current(
    tokens: &mut impl Iterator<Item = String>
) -> Result<TenmaScriptCommand, ParseError> {
    match tokens.next() {
        Some(s) => {
            if let Ok(num) = s.parse::<f32>() {
                return Ok(TenmaScriptCommand::I { current: num });
            } else {
                return Err(ParseError::IntParseError { symbol: s });
            }
        }
        None => {
            return Err(ParseError::MissingValue);
        }
    }
}

pub fn parse_delay(
    peekable_tokens: &mut Peekable<impl Iterator<Item = String>>
) -> Result<TenmaScriptCommand, ParseError> {
    let mut time: time::Duration;
    match peekable_tokens.next() {
        Some(s) => {
            if let Ok(num) = s.parse::<u64>() {
                time = time::Duration::from_secs(num);
            } else {
                return Err(ParseError::IntParseError { symbol: s.clone() });
            }
        }

        None => {
            return Err(ParseError::MissingValue);
        }
    }

    if let Some(s) = peekable_tokens.peek() {
        if let Ok(dur) = delay_unit_to_duration(s) {
            time = dur * (time.as_secs() as u32);
            peekable_tokens.next();
        }
    }

    Ok(TenmaScriptCommand::Delay { milliseconds: time.as_millis() as u64 })
}

pub fn parse_loop_start(tokens: &mut impl Iterator<Item = String>) -> Result<u32, ParseError> {
    match tokens.next() {
        Some(s) => {
            if let Ok(num) = s.parse::<u32>() {
                return Ok(num);
            } else {
                return Err(ParseError::IntParseError { symbol: s });
            }
        }
        None => {
            return Err(ParseError::MissingValue);
        }
    }
}
