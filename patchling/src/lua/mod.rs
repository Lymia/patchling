use anyhow::*;
use mlua::{Lua, StdLib};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

mod paths;

fn chk<T, E: Display>(r: std::result::Result<T, E>) -> Result<T> {
    match r {
        Ok(v) => Ok(v),
        Err(e) => Err(Error::msg(format!("{}", e))),
    }
}

pub struct LuaContext(Lua);
impl LuaContext {
    pub fn new(source_path: impl AsRef<Path>) -> Result<LuaContext> {
        let lua_root = paths::get_lua_root_dir()?;
        let lua = unsafe {
            Lua::unsafe_new_with(
                StdLib::STRING
                    | StdLib::MATH
                    | StdLib::PACKAGE
                    | StdLib::TABLE
                    | StdLib::BIT
                    | StdLib::DEBUG
                    | StdLib::IO,
            )
        };
        let lua = LuaContext(lua);

        let libs_path = chk(lua.0.create_string(lua_root.display().to_string().as_bytes()))?;
        let source_path =
            chk(lua.0.create_string(source_path.as_ref().display().to_string().as_bytes()))?;

        let chunk = chk(lua
            .0
            .load(include_str!("bootstrap_privileged.lua"))
            .set_name("@<intrinsic>/bootstrap_privileged.lua"))?;
        chk(chunk.call::<_, ()>((libs_path, source_path)))?;

        let chunk = chk(lua
            .0
            .load(include_str!("bootstrap_metalua.lua"))
            .set_name("@<intrinsic>/bootstrap_metalua.lua"))?;
        chk(chunk.call::<_, ()>(()))?;

        Ok(lua)
    }
}

pub fn test_lua() -> Result<()> {
    LuaContext::new(PathBuf::new())?;
    Ok(())
}
