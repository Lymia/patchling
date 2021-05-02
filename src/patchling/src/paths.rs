use crate::Game;
use anyhow::*;
use std::{env, fs, path::PathBuf, str::FromStr};

pub fn get_lua_root_dir() -> Result<PathBuf> {
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
    fn get_appimage_dir() -> Result<PathBuf> {
        // for AppImage builds
        match env::var_os("APPDIR") {
            Some(app_dir) => {
                let mut dir = PathBuf::from(app_dir);
                dir.push("usr/share/patchling");
                Ok(dir)
            }
            None => bail!("APPDIR not found."),
        }
    }
    fn get_cargo_parent_dir() -> Result<PathBuf> {
        // for workspaces
        let mut path = get_cargo_dir()?;
        path = path.canonicalize()?;
        path.pop();
        Ok(path)
    }
    fn check_is_root_dir(buf: PathBuf) -> Result<PathBuf> {
        ensure!(buf.exists(), "Path does not exist.");

        let mut tmp = buf.clone();
        tmp.push("lua_modules/share/lua/5.1");
        ensure!(tmp.exists(), "Path does not contain a lua_modules directory.");

        Ok(buf)
    }

    let mut buf = get_exe_dir()
        .and_then(check_is_root_dir)
        .or_else(|_| get_appimage_dir().and_then(check_is_root_dir))
        .or_else(|_| get_cargo_dir().and_then(check_is_root_dir))
        .or_else(|_| get_cargo_parent_dir().and_then(check_is_root_dir))
        .map_err(|_| Error::msg("Could not find Lua modules path. Are all the files present?"))?;
    buf.push("lua_modules");
    Ok(buf)
}

pub fn find_game_data(game: Game) -> Result<Vec<PathBuf>> {
    fn load_library_folders(mut root_path: PathBuf) -> Result<Vec<PathBuf>> {
        root_path.push("steamapps/libraryfolders.vdf");
        let mut paths = Vec::new();
        if root_path.exists() {
            let file_data = fs::read_to_string(&root_path)?;
            for line in file_data.split('\n') {
                let mut fields = line
                    .split('\t')
                    .filter(|x| x.starts_with("\"") && x.ends_with("\""))
                    .map(|x| &x[1..x.len() - 1]);
                if let Some(key) = fields.next() {
                    if let Some(value) = fields.next() {
                        if fields.next().is_none() {
                            if u32::from_str(&key).is_ok() {
                                paths.push(PathBuf::from(value));
                            }
                        }
                    }
                }
            }
        }
        root_path.pop();
        root_path.pop();
        paths.push(root_path);
        Ok(paths)
    }

    #[cfg(target_os = "linux")]
    fn steam_path() -> Result<PathBuf> {
        if let Some(mut home) = dirs::home_dir() {
            home.push(".steam/steam");
            Ok(home)
        } else {
            bail!("No home directory?");
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn steam_path() -> Result<PathBuf> {
        bail!("Platform not currently supported.")
    }

    debug!("Finding game data directory...");
    let steam_path = steam_path()?;
    debug!("- Steam path: {}", steam_path.display());
    let mut paths = Vec::new();
    for mut path in load_library_folders(steam_path)? {
        debug!("- Checking library path: {}", path.display());
        path.push("steamapps/common");
        path.push(game.steam_name());
        if path.exists() {
            let mut tmp = path.clone();
            macro_rules! check_exists {
                ($path:literal) => {
                    tmp.push($path);
                    if !tmp.exists() {
                        continue;
                    }
                    tmp.pop();
                };
            }

            // Checks a few paths unique to PDX games.
            check_exists!("checksum_manifest.txt");
            check_exists!("tweakergui_assets");
            check_exists!("common");

            paths.push(path);
        }
    }
    Ok(paths)
}
