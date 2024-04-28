use core::time;
use std::{rc::Rc, sync::Arc, thread};

use mlua::{Error as LuaError, Lua};

use crate::tenma::tenma_commands::{TenmaCommand, TenmaCommandTrait};

pub fn add_serial_var(lua: &Lua) -> Result<(), LuaError> {
    lua.globals().set("_serial", 0)?;
    lua.load(
        r#"
    ser = function(a)
        _serial = a - 1
    end
    "#,
    )
    .exec()
}

pub fn add_channel_var(lua: &Lua) -> Result<(), LuaError> {
    lua.globals().set("_channel", 1)?;
    lua.load(
        r#"
    ch = function(a)
        _channel = a
    end
    "#,
    )
    .exec()
}

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

pub fn add_set_voltage(
    lua: &Lua,
    ser: Rc<Vec<Box<dyn TenmaCommandTrait>>>,
) -> Result<(), LuaError> {
    lua.globals().set(
        "_v_delay",
        lua.create_function(move |l, a: (i64, f64, f64)| {
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

            let port = l.globals().get("_serial")?;
            if (0..ser.len()).contains(&port) {
                ser[port].run_command(TenmaCommand::VSet {
                    channel: a.0 as u8,
                    voltage: a.1,
                });

                thread::sleep(time::Duration::from_millis((a.2 * 1000.0) as u64));

                return Ok(());
            }

            Err(LuaError::BadArgument {
                to: Some("v".to_string()),
                pos: 0,
                name: Some("serial port".to_string()),
                cause: Arc::new(LuaError::RuntimeError(
                    "Serial port does not exist".to_string(),
                )),
            })
        })?,
    )?;

    lua.load(
        r#"
    v = function (volt, delay)
        if not delay then
            _v_delay(_channel, volt, 0.05)
        else
            _v_delay(_channel, volt, delay)
        end
    end
    "#,
    )
    .exec()?;

    Ok(())
}

pub fn add_set_current(
    lua: &Lua,
    ser: Rc<Vec<Box<dyn TenmaCommandTrait>>>,
) -> Result<(), LuaError> {
    lua.globals().set(
        "_i_delay",
        lua.create_function(move |l, a: (i64, f64, f64)| {
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

            let port = l.globals().get("_serial")?;
            if (0..ser.len()).contains(&port) {
                ser[port].run_command(TenmaCommand::ISet {
                    channel: a.0 as u8,
                    current: a.1,
                });

                thread::sleep(time::Duration::from_millis((a.2 * 1000.0) as u64));

                return Ok(());
            }

            Err(LuaError::BadArgument {
                to: Some("i".to_string()),
                pos: 0,
                name: Some("serial port".to_string()),
                cause: Arc::new(LuaError::RuntimeError(
                    "Serial port does not exist".to_string(),
                )),
            })
        })?,
    )?;

    lua.load(
        r#"
    i = function (volt, delay)
        if not delay then
            _i_delay(_channel, volt, 0.05)
        else
            _i_delay(_channel, volt, delay)
        end
    end
    "#,
    )
    .exec()?;

    Ok(())
}

pub fn add_set_out(lua: &Lua, ser: Rc<Vec<Box<dyn TenmaCommandTrait>>>) -> Result<(), LuaError> {
    lua.globals().set(
        "out",
        lua.create_function(move |l, a: bool| {
            let port = l.globals().get("_serial")?;
            if (0..ser.len()).contains(&port) {
                ser[port].run_command(TenmaCommand::Out(a));
                thread::sleep(time::Duration::from_millis(50));

                return Ok(());
            }

            Err(LuaError::BadArgument {
                to: Some("out".to_string()),
                pos: 0,
                name: Some("serial port".to_string()),
                cause: Arc::new(LuaError::RuntimeError(
                    "Serial port does not exist".to_string(),
                )),
            })
        })?,
    )?;

    Ok(())
}

pub fn add_set_beep(lua: &Lua, ser: Rc<Vec<Box<dyn TenmaCommandTrait>>>) -> Result<(), LuaError> {
    lua.globals().set(
        "beep",
        lua.create_function(move |l, a: bool| {
            let port = l.globals().get("_serial")?;
            if (0..ser.len()).contains(&port) {
                ser[port].run_command(TenmaCommand::Beep(a));
                thread::sleep(time::Duration::from_millis(50));

                return Ok(());
            }

            Err(LuaError::BadArgument {
                to: Some("beep".to_string()),
                pos: 0,
                name: Some("serial port".to_string()),
                cause: Arc::new(LuaError::RuntimeError(
                    "Serial port does not exist".to_string(),
                )),
            })
        })?,
    )?;

    Ok(())
}

pub fn add_get_voltage(
    lua: &Lua,
    ser: Rc<Vec<Box<dyn TenmaCommandTrait>>>,
) -> Result<(), LuaError> {
    lua.globals().set(
        "getv",
        lua.create_function(move |l, _: ()| {
            let port = l.globals().get("_serial")?;
            if (0..ser.len()).contains(&port) {
                if let Some(num) = ser[port].run_command(TenmaCommand::VGet {
                    channel: l.globals().get("_channel")?,
                }) {
                    return Ok(num);
                }
                return Err(LuaError::RuntimeError(
                    "Bad / No response from tenma".to_string(),
                ));
            }
            Err(LuaError::BadArgument {
                to: Some("getv".to_string()),
                pos: 0,
                name: Some("serial port".to_string()),
                cause: Arc::new(LuaError::RuntimeError(
                    "Serial port does not exist".to_string(),
                )),
            })
        })?,
    )?;

    Ok(())
}

pub fn add_get_current(
    lua: &Lua,
    ser: Rc<Vec<Box<dyn TenmaCommandTrait>>>,
) -> Result<(), LuaError> {
    lua.globals().set(
        "geti",
        lua.create_function(move |l, _: ()| {
            let port = l.globals().get("_serial")?;
            if (0..ser.len()).contains(&port) {
                if let Some(num) = ser[port].run_command(TenmaCommand::IGet {
                    channel: l.globals().get("_channel")?,
                }) {
                    return Ok(num);
                }
                return Err(LuaError::RuntimeError(
                    "Bad / No response from tenma".to_string(),
                ));
            }
            Err(LuaError::BadArgument {
                to: Some("geti".to_string()),
                pos: 0,
                name: Some("serial port".to_string()),
                cause: Arc::new(LuaError::RuntimeError(
                    "Serial port does not exist".to_string(),
                )),
            })
        })?,
    )?;

    Ok(())
}
