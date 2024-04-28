mod lua_functions;

use std::rc::Rc;

use mlua::{Error, Lua};

use crate::tenma::tenma_commands::TenmaCommandTrait;

#[allow(unused)]

pub struct LuaScript {
    lua: Lua,
    script: String,
}

impl LuaScript {
    pub fn new(script: &str, serial: Box<dyn TenmaCommandTrait>) -> Result<Self, Error> {
        let lua = Lua::new();
        let script = script.to_string();
        let serial = Rc::new(serial);

        Self::setup(&lua, serial.clone())?;

        Ok(Self { lua, script })
    }

    fn setup(lua: &Lua, serial: Rc<Box<dyn TenmaCommandTrait>>) -> Result<(), Error> {
        lua_functions::add_delay_func(lua)?;
        lua_functions::add_set_voltage(lua, serial.clone())?;
        lua_functions::add_set_current(lua, serial.clone())?;
        lua_functions::add_set_out(lua, serial.clone())?;
        lua_functions::add_set_beep(lua, serial.clone())?;
        lua_functions::add_get_voltage(lua, serial.clone())?;
        lua_functions::add_get_current(lua, serial.clone())?;

        Ok(())
    }

    pub fn run(&self) -> Result<(), Error> {
        self.lua.load(self.script.clone()).exec()
    }
}

#[test]
fn lua_test() {
    use crate::tenma::tenma_command_tester::TenmaTester;
    use std::fs;

    let serial = TenmaTester {};

    let script = fs::read_to_string("lua/test.lua").unwrap();

    let lua = LuaScript::new(script.as_str(), Box::new(serial)).unwrap();

    lua.run().unwrap_or_else(|err| {
        println!("{err}");
    });
}
