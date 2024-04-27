use core::time;
use std::{rc::Rc, thread, time::Duration};

use mlua::{Error as LuaError, Lua};

use crate::tenma_serial::{tenma_commands::TenmaCommand, TenmaSerial};

pub fn add_range_func(lua: &Lua) -> Result<(), LuaError> {
    lua.load(r#"
    range = function (i, to, inc)
                if i == nil then return end -- range(--[[ no args ]]) -> return "nothing" to fail the loop in the caller

                if not to then
                    to = i
                    i  = to == 0 and 0 or (to > 0 and 1 or -1)
                end

                -- we don't have to do the to == 0 check
                -- 0 -> 0 with any inc would never iterate
                inc = inc or (i < to and 1 or -1)

                -- step back (once) before we start
                i = i - inc

                return function () if i == to then return nil end i = i + inc return i, i end
            end
        "#).exec().unwrap();
    Ok(())
}

pub fn add_delay_func(lua: &Lua) -> Result<(), LuaError> {
    lua.globals().set(
        "delay",
        lua.create_function(move |_, a: f64| {
            thread::sleep(time::Duration::from_millis((a * 1000.0) as u64));
            Ok(())
        })?,
    )?;
    Ok(())
}

pub fn add_set_voltage(lua: &Lua, ser: Rc<TenmaSerial>) -> Result<(), LuaError> {
    lua.globals().set(
        "_v_delay",
        lua.create_function(move |_, a: (i64, f64, f64)| {
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

pub fn add_set_current(lua: &Lua, ser: Rc<TenmaSerial>) -> Result<(), LuaError> {
    lua.globals().set(
        "_i_delay",
        lua.create_function(move |_, a: (i64, f64, f64)| {
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

pub fn add_set_out(lua: &Lua, ser: Rc<TenmaSerial>) -> Result<(), LuaError> {
    let ser1 = ser.clone();
    lua.globals().set(
        "out",
        lua.create_function(move |_, a: bool| {
            ser1.run_command(TenmaCommand::Out(a));
            thread::sleep(Duration::from_millis(50));
            Ok(())
        })?,
    )?;

    Ok(())
}

pub fn add_set_beep(lua: &Lua, ser: Rc<TenmaSerial>) -> Result<(), LuaError> {
    let ser1 = ser.clone();
    lua.globals().set(
        "beep",
        lua.create_function(move |_, a: bool| {
            ser1.run_command(TenmaCommand::Beep(a));
            thread::sleep(Duration::from_millis(50));
            Ok(())
        })?,
    )?;

    Ok(())
}
