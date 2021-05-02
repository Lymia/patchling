use anyhow::*;
use clap::{AppSettings, Clap};
use patchling::{CompilerBuilder, Game};
use std::{env, path::PathBuf};
use tracing::Level;

/// A tool for making mods for Stellaris and other Paradox Interactive games.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Aurora Amissa <aurora@aura.moe>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Print additional debugging output.
    #[clap(short, long)]
    verbose: bool,
    /// The directory that contains the base game data.
    #[clap(long)]
    game_data: Option<PathBuf>,
}

fn main_res(opts: Opts) -> Result<()> {
    let mut builder = CompilerBuilder::new(Game::Stellaris);
    if let Some(data) = opts.game_data {
        builder = builder.game_data(data);
    }
    let mut compiler = builder.build()?;

    Ok(())
}

fn main() {
    let opts: Opts = Opts::parse();
    if opts.verbose {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let max_level = if opts.verbose { Level::TRACE } else { Level::INFO };
    tracing_subscriber::fmt().with_max_level(max_level).init();

    if let Err(e) = main_res(opts) {
        eprintln!("{}", e);
        let trace = e.backtrace().to_string();
        if !trace.is_empty() && trace != "disabled backtrace" {
            eprintln!();
            eprintln!("{}", trace);
        }
    }
}
