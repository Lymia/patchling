use crate::{mods::LoadedMod, paths};
use anyhow::*;
use mlua::{
    prelude::LuaResult, FromLua, FromLuaMulti, Function, Lua, LuaSerdeExt, StdLib, ToLua,
    ToLuaMulti, UserData,
};
use std::path::{Path, PathBuf};

// TODO: Logging.

pub struct LuaContext(Lua);
impl LuaContext {
    pub fn new(lua_root: impl AsRef<Path>, mod_paths: &[LoadedMod]) -> Result<LuaContext> {
        let lua_root = lua_root.as_ref().to_path_buf();
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
        let mod_paths = lua.0.to_value(mod_paths);

        lua.0
            .load(include_str!("bootstrap_privileged.lua"))
            .set_name("@<intrinsic>/bootstrap_privileged.lua")?
            .call::<_, ()>((libs_path, mod_paths))?;
        lua.0
            .load(include_str!("bootstrap_metalua.lua"))
            .set_name("@<intrinsic>/bootstrap_metalua.lua")?
            .call::<_, ()>(())?;
        Ok(lua)
    }

    fn wrapped_execute<
        'lua: 'callback,
        'callback,
        P: ToLuaMulti<'callback>,
        R: FromLua<'callback>,
    >(
        &'lua self,
        func_module: &str,
        args: P,
    ) -> Result<R> {
        let require = self.0.globals().get::<_, Function<'_>>("require")?;
        let check_error: Function<'_> = require.call("patchling_private.check_error")?;
        let func: Function<'_> = require.call(func_module)?;
        let (res, err): (Option<R>, Option<String>) = check_error.bind(func)?.call(args)?;
        if let Some(e) = err { bail!("{}", e) } else { Ok(res.unwrap()) }
    }

    pub fn execute_script(&self, path: PathBuf) -> Result<()> {
        todo!()
    }

    pub fn compile_and_minify(&self, source: &str, name: &str) -> Result<String> {
        Ok(self.wrapped_execute("patchling_private.compile_and_minify", (source, name))?)
    }

    pub fn register_module(
        &self,
        name: &str,
        userdata: impl UserData + Send + 'static,
    ) -> Result<()> {
        self.0.globals().set(self.0.create_string(name)?, self.0.create_userdata(userdata)?)?;
        Ok(())
    }
}
