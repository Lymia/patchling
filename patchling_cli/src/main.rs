use clap::Clap;
use patchling::pdx::PdxBlock;
use std::path::PathBuf;

/// Build utility for the Robopon randomizer.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Aurora Amissa <aurora@aura.moe>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    TestLua,
    TestParser(CmdTestParser),
}

#[derive(Clap)]
struct CmdTestParser {
    input: PathBuf,
}

fn main() {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::TestLua => patchling::test_load_lua(),
        SubCommand::TestParser(cmd) => {
            let file = std::fs::read(cmd.input).unwrap();
            let parsed = patchling::pdx::PdxBlock::parse_file("test.txt", &file).unwrap();
            let source = serde_json::to_string_pretty(&parsed).unwrap();
            let source_ugly = serde_json::to_string(&parsed).unwrap();
            println!("{}", source);
            println!("{}", parsed.display_file(false, false));

            // FIXME do this because we don't have eq. Fixed point maybe??
            let parsed2: PdxBlock<'static> = serde_json::from_str(&source).unwrap();
            let source2 = serde_json::to_string(&parsed2).unwrap();
            assert_eq!(source_ugly, source2);
        }
    }
}
