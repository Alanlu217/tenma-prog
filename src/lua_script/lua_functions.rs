use core::time;
use std::{rc::Rc, sync::Arc, thread, time::Duration};

use mlua::{Error as LuaError, Lua};

use crate::tenma::tenma_commands::{TenmaCommand, TenmaCommandTrait};

pub fn add_delay_func(lua: &Lua) -> Result<(), LuaError> {
    lua.globals().set(
        "delay",
        lua.create_function(move |_, a: f64| {
            if a < 0.0 {
                return Err(LuaError::BadArgument {
                    to: Some("delay".to_string()),
                    pos: 1,
                    name: Some("time".to_string()),
                    cause: Arc::new(LuaError::RuntimeError("delay must be positive".to_string())),
                });
            }

            thread::sleep(time::Duration::from_millis((a * 1000.0) as u64));
            Ok(())
        })?,
    )?;
    Ok(())
}

pub fn add_set_voltage(lua: &Lua, ser: Rc<Box<dyn TenmaCommandTrait>>) -> Result<(), LuaError> {
    lua.globals().set(
        "_v_delay",
        lua.create_function(move |_, a: (i64, f64, f64)| {
            if a.0 < 1 {
                return Err(LuaError::BadArgument {
                    to: Some("v".to_string()),
                    pos: 1,
                    name: Some("channel".to_string()),
                    cause: Arc::new(LuaError::RuntimeError(
                        "channel must be 1 or larger".to_string(),
                    )),
                });
            }
            if a.1 < 0.0 {
                return Err(LuaError::BadArgument {
                    to: Some("v".to_string()),
                    pos: 2,
                    name: Some("voltage".to_string()),
                    cause: Arc::new(LuaError::RuntimeError(
                        "voltage must be positive".to_string(),
                    )),
                });
            }
            if a.2 < 0.0 {
                return Err(LuaError::BadArgument {
                    to: Some("v".to_string()),
                    pos: 3,
                    name: Some("delay".to_string()),
                    cause: Arc::new(LuaError::RuntimeError("delay must be positive".to_string())),
                });
            }

            ser.run_command(TenmaCommand::VSet {
                channel: a.0 as u8,
                voltage: a.1,
            });

            thread::sleep(time::Duration::from_millis((a.2 * 1000.0) as u64));

            Ok(())
        })?,
    )?;

    lua.load(
        r#"
    v = function (volt, delay)
        if not delay then
            _v_delay(1, volt, 0.05)
        else
            _v_delay(1, volt, delay)
        end
    end
    "#,
    )
    .exec()?;

    Ok(())
}

pub fn add_set_current(lua: &Lua, ser: Rc<Box<dyn TenmaCommandTrait>>) -> Result<(), LuaError> {
    lua.globals().set(
        "_i_delay",
        lua.create_function(move |_, a: (i64, f64, f64)| {
            if a.0 < 1 {
                return Err(LuaError::BadArgument {
                    to: Some("i".to_string()),
                    pos: 1,
                    name: Some("channel".to_string()),
                    cause: Arc::new(LuaError::RuntimeError(
                        "channel must be 1 or larger".to_string(),
                    )),
                });
            }
            if a.1 < 0.0 {
                return Err(LuaError::BadArgument {
                    to: Some("i".to_string()),
                    pos: 2,
                    name: Some("voltage".to_string()),
                    cause: Arc::new(LuaError::RuntimeError(
                        "voltage must be positive".to_string(),
                    )),
                });
            }
            if a.2 < 0.0 {
                return Err(LuaError::BadArgument {
                    to: Some("i".to_string()),
                    pos: 3,
                    name: Some("delay".to_string()),
                    cause: Arc::new(LuaError::RuntimeError("delay must be positive".to_string())),
                });
            }

            ser.run_command(TenmaCommand::ISet {
                channel: a.0 as u8,
                current: a.1,
            });

            thread::sleep(time::Duration::from_millis((a.2 * 1000.0) as u64));

            Ok(())
        })?,
    )?;

    lua.load(
        r#"
    i = function (volt, delay)
        if not delay then
            _i_delay(1, volt, 0.05)
        else
            _i_delay(1, volt, delay)
        end
    end
    "#,
    )
    .exec()?;

    Ok(())
}

pub fn add_set_out(lua: &Lua, ser: Rc<Box<dyn TenmaCommandTrait>>) -> Result<(), LuaError> {
    lua.globals().set(
        "out",
        lua.create_function(move |_, a: bool| {
            ser.run_command(TenmaCommand::Out(a));
            thread::sleep(Duration::from_millis(50));
            Ok(())
        })?,
    )?;

    Ok(())
}

pub fn add_set_beep(lua: &Lua, ser: Rc<Box<dyn TenmaCommandTrait>>) -> Result<(), LuaError> {
    lua.globals().set(
        "beep",
        lua.create_function(move |_, a: bool| {
            ser.run_command(TenmaCommand::Beep(a));
            thread::sleep(Duration::from_millis(50));
            Ok(())
        })?,
    )?;

    Ok(())
}

pub fn add_get_voltage(lua: &Lua, ser: Rc<Box<dyn TenmaCommandTrait>>) -> Result<(), LuaError> {
    lua.globals().set(
        "getv",
        lua.create_function(move |_, _: ()| {
            if let Some(num) = ser.run_command(TenmaCommand::VGet { channel: 1 }) {
                return Ok(num);
            }
            Err(LuaError::RuntimeError(
                "Bad / No response from tenma".to_string(),
            ))
        })?,
    )?;

    Ok(())
}

pub fn add_get_current(lua: &Lua, ser: Rc<Box<dyn TenmaCommandTrait>>) -> Result<(), LuaError> {
    lua.globals().set(
        "geti",
        lua.create_function(move |_, _: ()| {
            if let Some(num) = ser.run_command(TenmaCommand::IGet { channel: 1 }) {
                return Ok(num);
            }
            Err(LuaError::RuntimeError(
                "Bad / No response from tenma".to_string(),
            ))
        })?,
    )?;

    Ok(())
}
