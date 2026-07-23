//! IdleScreen Studio / Director TUI (scaffold).
//!
//! Will orchestrate idle-render jobs: queue, segments, music, channel presets.

use clap::Parser;
use std::process::ExitCode;

#[derive(Debug, Parser)]
#[command(
    name = "idle-studio",
    about = "IdleScreen Director (scaffold)",
    long_about = "TUI and job control for offline renders. Encoding lives in idle-render."
)]
struct Args {
    /// Open the interactive director (not implemented)
    #[arg(long)]
    tui: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    eprintln!("idle-studio scaffold — Director TUI not implemented yet");
    if args.tui {
        eprintln!("--tui requested; will host effect/seed/duration queue UI");
    }
    eprintln!("engine: https://github.com/idlescreen/idle-render");
    eprintln!("product: https://github.com/idlescreen/idle-pro");
    ExitCode::from(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let a = Args::parse_from(["idle-studio"]);
        assert!(!a.tui);
    }
}
