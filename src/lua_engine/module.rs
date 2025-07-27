use mlua::prelude::*;

//TODO
pub fn mod_engine(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set(
        "game_info",
        lua.create_function(|_, ()| Ok("simple_warfare"))?,
    )?;

    Ok(exports)
}
