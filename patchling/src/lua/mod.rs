use anyhow::*;
use mlua::{Function, Lua, StdLib};
use std::path::{Path, PathBuf};

mod paths;

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

        let libs_path = lua.0.create_string(lua_root.display().to_string().as_bytes())?;
        let source_path =
            lua.0.create_string(source_path.as_ref().display().to_string().as_bytes())?;

        lua.0
            .load(include_str!("bootstrap_privileged.lua"))
            .set_name("@<intrinsic>/bootstrap_privileged.lua")?
            .call::<_, ()>((libs_path, source_path))?;
        lua.0
            .load(include_str!("bootstrap_metalua.lua"))
            .set_name("@<intrinsic>/bootstrap_metalua.lua")?
            .call::<_, ()>(())?;
        Ok(lua)
    }

    pub fn compile_and_minify(&self, source: &str, name: &str) -> Result<String> {
        let func: Function<'_> = self
            .0
            .globals()
            .get::<_, Function<'_>>("require")?
            .call("patchling_private.compile_and_minify")?;
        Ok(func.call((source, name))?)
    }
}

pub fn test_lua() -> Result<()> {
    let lua = LuaContext::new(PathBuf::new())?;
    println!(
        "{}",
        lua.compile_and_minify(
            include_str!("../../../patchling_rt/patchling_private/ast_to_src.mlua"),
            "ast_to_src.mlua"
        )?
    );
    Ok(())
}
