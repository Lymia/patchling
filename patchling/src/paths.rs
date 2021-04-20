use anyhow::*;
use std::{env, path::PathBuf};

fn get_exe_dir() -> Result<PathBuf> {
    // for release bulids
    let mut path = env::current_exe()?;
    path.pop();
    Ok(path)
}
fn get_cargo_dir() -> Result<PathBuf> {
    // for debug builds
    match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => Ok(PathBuf::from(manifest_dir)),
        None => bail!("CARGO_MANIFEST_DIR not found."),
    }
}
fn get_cargo_parent_dir() -> Result<PathBuf> {
    // for workspaces
    let mut path = get_cargo_dir()?;
    path.push("..");
    Ok(path)
}
fn check_is_root_dir(buf: PathBuf) -> Result<PathBuf> {
    ensure!(buf.exists(), "Path does not exist.");

    let mut tmp = buf.clone();
    tmp.push("lua_modules/share/lua/5.1");
    ensure!(tmp.exists(), "Path does not contain a lua_modules directory.");

    Ok(buf)
}
fn get_root_dir() -> Result<PathBuf> {
    get_exe_dir()
        .and_then(check_is_root_dir)
        .or_else(|_| get_cargo_dir().and_then(check_is_root_dir))
        .or_else(|_| get_cargo_parent_dir().and_then(check_is_root_dir))
}

pub fn get_lua_root_dir() -> Result<PathBuf> {
    let mut buf = get_root_dir()?;
    buf.push("lua_modules");
    Ok(buf)
}
