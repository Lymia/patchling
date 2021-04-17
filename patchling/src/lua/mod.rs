use anyhow::*;
use mlua::{Lua, StdLib};

mod paths;

pub fn test_lua() -> Result<()> {
    let lua_root = paths::get_lua_root_dir()?;

    let lua = Lua::new_with(StdLib::STRING | StdLib::MATH | StdLib::PACKAGE).unwrap();
    let chunk = lua.load(include_str!("bootstrap.lua"));
    let libs_path = lua.create_string(lua_root.display().to_string().as_bytes()).unwrap();
    let source_path = lua.create_string("").unwrap();

    match chunk.call::<_, ()>((libs_path, source_path)) {
        Ok(()) => Ok(()),
        Err(e) => bail!("{}", e),
    }
}
