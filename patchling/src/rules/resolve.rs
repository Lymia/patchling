use crate::rules::{DataRoot, ResolverMode};
use anyhow::*;
use mlua::{Lua, Value};
use std::{fs, path::PathBuf, sync::Arc};

fn check_name_safe(dir: &str) -> Result<()> {
    for ch in dir.chars() {
        match ch {
            'a'..='z' | '0'..='9' | '_' | '.' => {}
            'A'..='Z' => bail!("Please use lowercase path names, as this is required on Linux."),
            _ => bail!("Invalid character in filename: {:?}", ch),
        }
    }
    Ok(())
}

struct ResolvedFile {
    source_mod: Option<Arc<str>>,
    file_name: String,
    path: PathBuf,
}
fn resolve_files(
    roots: &[DataRoot],
    directory: &str,
    extension: &str,
) -> Result<Vec<ResolvedFile>> {
    let mut resolved = Vec::new();
    for root in roots {
        let source_mod = if root.is_mod { Some(root.name.clone()) } else { None };

        let mut root_path = root.root_dir.clone();
        root_path.push(directory);
        for file in fs::read_dir(&root_path)? {
            let file = file?;
            let file_name = file.file_name().to_string_lossy().to_string();
            if file_name.ends_with(extension) {
                resolved.push(ResolvedFile {
                    source_mod: source_mod.clone(),
                    file_name,
                    path: file.path(),
                });
            }
        }
    }
    Ok(resolved)
}

pub fn load_rules<'a>(
    lua: &'a Lua,
    roots: &[DataRoot],
    mode: ResolverMode,
    directory: &str,
    extension: &str,
) -> Result<Value<'a>> {
    check_name_safe(directory)?;
    check_name_safe(extension)?;

    Ok(Value::Nil)
}
