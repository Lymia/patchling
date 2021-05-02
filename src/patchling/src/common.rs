use crate::{lua::LuaContext, paths, rules::RulesManager};
use anyhow::*;
use serde::*;
use std::path::PathBuf;

/// Used to define the game that's being compiled for.
#[derive(Serialize, Deserialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Game {
    Stellaris,
}
impl Game {
    /// Returns the display name of this game.
    pub fn display_name(&self) -> &str {
        match self {
            Game::Stellaris => "Stellaris",
        }
    }

    /// Returns the name of the `common/*` directory Steam uses for this game.
    pub fn steam_name(&self) -> &str {
        self.display_name() // technically different, but should mostly be good.
    }
}

/// A compiler for Patchling mod definitions.
pub struct Compiler {
    lua_ctx: LuaContext,
}
impl Compiler {
    /// Initializes a compiler with default settings.
    pub fn new(game: Game) -> Result<Compiler> {
        CompilerBuilder::new(game).build()
    }

    /// Returns a builder for a compiler.
    pub fn builder(game: Game) -> CompilerBuilder {
        CompilerBuilder::new(game)
    }
}

/// A builder for compiler objects.
#[derive(Debug)]
pub struct CompilerBuilder {
    game: Game,
    game_data: Option<PathBuf>,
}
impl CompilerBuilder {
    /// Creates a new compiler builder.
    pub fn new(game: Game) -> Self {
        CompilerBuilder { game, game_data: None }
    }

    pub fn game_data(mut self, path: impl Into<PathBuf>) -> Self {
        self.game_data = Some(path.into());
        self
    }

    pub fn build(self) -> Result<Compiler> {
        let root_path = paths::get_lua_root_dir()?;

        // Find the game data directory
        let game_data = self.game_data.clone();
        let game_data = if let Some(data) = game_data {
            debug!("Game data: {} (explicitly set)", data.display());
            data
        } else {
            let paths = paths::find_game_data(self.game)?;
            if paths.is_empty() {
                bail!(
                    "Could not find game data directory. \
                     Please explicitly set it using --game-data."
                );
            } else {
                let game_data = paths.into_iter().next().unwrap();
                debug!("Game data: {}", game_data.display());
                game_data
            }
        };

        // Create the Lua context.
        debug!("Initializing Lua context...");
        let lua_ctx = LuaContext::new(root_path, &[])?;
        let rules = RulesManager::new(self.game);
        lua_ctx.register_module("rules", rules)?;

        // TODO: Testing
        println!(
            "{}",
            lua_ctx.compile_and_minify(
                include_str!("../../patchling_rt/patchling_private/mlua/ast_to_src.mlua"),
                "ast_to_src.mlua",
            )?
        );

        debug!("Compiler initialized!");
        Ok(Compiler { lua_ctx })
    }
}
