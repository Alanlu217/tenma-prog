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
    pub fn new(script: &str, serials: Vec<Box<dyn TenmaCommandTrait>>) -> Result<Self, Error> {
        let lua = Lua::new();
        let script = script.to_string();
        let serials = Rc::new(serials);

        Self::setup(&lua, serials.clone())?;

        Ok(Self { lua, script })
    }

    fn setup(lua: &Lua, serials: Rc<Vec<Box<dyn TenmaCommandTrait>>>) -> Result<(), Error> {
        lua_functions::add_serial_var(lua)?; // Must go before others
        lua_functions::add_channel_var(lua)?; // Must go before others
        lua_functions::add_delay_func(lua)?;
        lua_functions::add_set_voltage(lua, serials.clone())?;
        lua_functions::add_set_current(lua, serials.clone())?;
        lua_functions::add_set_out(lua, serials.clone())?;
        lua_functions::add_set_beep(lua, serials.clone())?;
        lua_functions::add_get_voltage(lua, serials.clone())?;
        lua_functions::add_get_current(lua, serials.clone())?;

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

    let serial = TenmaTester { port: 1 };

    let script = fs::read_to_string("lua/test.lua").unwrap();

    let lua = LuaScript::new(script.as_str(), vec![Box::new(serial)]).unwrap();

    lua.run().unwrap_or_else(|err| {
        println!("{err}");
    });
}
