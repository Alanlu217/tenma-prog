mod lua_functions;

#[allow(unused)]
use std::{fs, rc::Rc};

use mlua::{Error, Lua};

use crate::tenma_serial::TenmaSerial;

pub struct LuaScript {
    lua: Lua,
    script: String,
}

impl LuaScript {
    pub fn new(script: &str, serial: TenmaSerial) -> Result<Self, Error> {
        let lua = Lua::new();
        let script = script.to_string();
        let serial = Rc::new(serial);

        Self::setup(&lua, serial.clone())?;

        Ok(Self { lua, script })
    }

    fn setup(lua: &Lua, serial: Rc<TenmaSerial>) -> Result<(), Error> {
        lua_functions::add_delay_func(lua)?;
        lua_functions::add_set_voltage(lua, serial.clone())?;
        lua_functions::add_set_current(lua, serial.clone())?;
        lua_functions::add_set_out(lua, serial.clone())?;
        lua_functions::add_set_beep(lua, serial.clone())?;

        Ok(())
    }

    pub fn run(&self) -> Result<(), Error> {
        self.lua.load(self.script.clone()).exec()
    }
}

#[ignore]
#[test]
fn lua_test() {
    let serial = TenmaSerial::new("/dev/tty.Bluetooth-Incoming-Port").unwrap();

    let script = fs::read_to_string("lua/test.lua").unwrap();

    let lua = LuaScript::new(script.as_str(), serial).unwrap();

    lua.run().unwrap();
}
