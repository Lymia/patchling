mod resolve;
mod rules_parser;

use crate::{
    pdx::{PdxBlock, PdxRelation, PdxRelationType, PdxRelationValue},
    Game,
};
use anyhow::*;
use indexmap::IndexMap;
use mlua::{
    prelude::{LuaResult, LuaString},
    serde::LuaSerdeExt,
    Lua, RegistryKey, UserData, UserDataMethods, Value,
};
use serde::*;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use twox_hash::RandomXxh3HashBuilder64;

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
enum DefaultRuleType {
    RuleEquals,
}

#[derive(Debug)]
struct RuleInfo {
    origin_mod: u32,
    original: Option<PdxRelation>,
    lua_mirror: Option<RegistryKey>,
}
impl RuleInfo {
    fn get_lua_mirror<'lua>(
        &mut self,
        rule_name: &str,
        default: &DefaultRuleType,
        lua: &'lua Lua,
    ) -> LuaResult<Value<'lua>> {
        if self.lua_mirror.is_none() {
            let new_value = match &self.original {
                Some(relation) => lua.to_value(relation)?,
                None => match default {
                    DefaultRuleType::RuleEquals => lua.to_value(&PdxRelation {
                        tag: rule_name.into(), // TODO: Pass an interner here.
                        relation: PdxRelationType::Normal,
                        value: PdxRelationValue::Block(PdxBlock { contents: Vec::new() }),
                    })?,
                },
            };

            let key = lua.create_registry_value(new_value)?;
            self.lua_mirror = Some(key);
        }
        lua.registry_value(self.lua_mirror.as_ref().unwrap())
    }
}

#[derive(Debug)]
pub struct ResolvedRules {
    default: DefaultRuleType,
    path: String,
    map: IndexMap<String, RuleInfo, RandomXxh3HashBuilder64>,
    initialized: bool,
}
impl ResolvedRules {
    fn new(character: DefaultRuleType, path: &str) -> Self {
        ResolvedRules {
            default: character,
            path: path.to_string(),
            map: Default::default(),
            initialized: false,
        }
    }

    fn add_rule_from_sources(&mut self, origin_mod: u32, name: &str, rule: PdxRelation) {
        assert!(!self.initialized, "Cannot add rule from sources after initialization.");
        if self.map.contains_key(name) {
            self.map.insert(name.to_string(), RuleInfo {
                origin_mod,
                original: Some(rule),
                lua_mirror: None,
            });
        } else {
            trace!("Ignoring rule {}. (Already defined.)", name);
        }
    }

    fn finish_init(&mut self) {
        assert!(!self.initialized, "Cannot initialize an already initialized rules set.");
        self.initialized = true;
    }

    fn get_rule(&mut self, origin_mod: u32, name: &str) -> &mut RuleInfo {
        assert!(self.initialized, "Cannot get rules in an uninitialized rules set.");
        if !self.map.contains_key(name) {
            self.map.insert(name.to_string(), RuleInfo {
                origin_mod,
                original: None,
                lua_mirror: None,
            });
        }
        self.map.get_mut(name).unwrap()
    }
}
impl UserData for ResolvedRules {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {

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
