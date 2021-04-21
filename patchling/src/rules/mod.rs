mod resolve;

use crate::{pdx::PdxBlock, Game};
use anyhow::*;
use mlua::{
    prelude::LuaString, serde::LuaSerdeExt, Lua, RegistryKey, UserData, UserDataMethods, Value,
};
use serde::*;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResolverMode {
    Simple,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DataRoot {
    pub is_mod: bool,
    pub name: Arc<str>,
    pub root_dir: PathBuf,
}
impl DataRoot {
    pub fn vanilla(path: PathBuf) -> DataRoot {
        DataRoot { is_mod: false, name: "Vanilla Game Data".into(), root_dir: path }
    }

    pub fn mod_data(name: String, root_dir: PathBuf) -> DataRoot {
        DataRoot { is_mod: true, name: name.into(), root_dir }
    }
}

#[derive(Debug)]
pub struct RulesManager {
    game: Game,
    data_roots: Vec<DataRoot>,
    resolvers: HashMap<(String, String), RegistryKey>,
}
impl RulesManager {
    pub fn new(game: Game) -> RulesManager {
        RulesManager { game, data_roots: Vec::new(), resolvers: HashMap::new() }
    }

    pub fn add_data_root(&mut self, root: DataRoot) {
        self.data_roots.push(root);
    }
}
impl UserData for RulesManager {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_game", |lua, this, _: ()| Ok(lua.to_value(&this.game)));
        methods.add_method(
            "get_resolver",
            |lua, this, args: (LuaString<'_>, Option<LuaString<'_>>, Option<Value<'_>>)| {
                let (path, extension, resolver_mode) = args;
                // TODO: Build resolvers
                Ok(0)
            },
        );
    }
}
