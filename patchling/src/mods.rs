use crate::Game;
use serde::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct ModInfo {
    pub id: String,
    pub game: Game,
    pub is_loaded: bool,
    pub copy_dirs: Vec<PathBuf>,
    pub source_dirs: Vec<PathBuf>,
    pub lib_dirs: Vec<PathBuf>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct LoadedMod {
    pub info: ModInfo,
    pub copy_files: Vec<(String, PathBuf)>,
    pub source_files: Vec<PathBuf>,
    pub lib_paths: Vec<PathBuf>,
}
