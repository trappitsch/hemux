//! Hemux
//!
//! TODO:
//!
//! - Fix help documentation such that it is nicely displayed and useful.
//! - Add support for user commands depending on language.
//! - Add python support.

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

use crate::commands::{command_for_mode, helix::HelixVariables, last_cmd::save_command_as_last};
pub use config::CONFIG;

mod commands;
mod config;
mod tmux;

/// The mode we might run the tests in.
#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum Mode {
    /// Run the last test or command.
    Last,
    /// Test a single file.
    TestFile,
    /// Test the nearest test.
    TestNearest,
    /// Run the whole test suite.
    TestSuite,
}

/// Helix CLI
///
/// Run test commands in tmux from within Helix editor.
///
/// Required arguments are the Mode -m/--mode and the language -l/--language.
/// If not provided, the language defaults to "rust".
///
/// Various modes required various attributes to be provided:
/// - last:         No other attributes required, but `$HEMUX_LAST_CMD` must be set.
/// - test-file:    Requires that -p/--path is provided.
/// - test-nearest: Requires that -p/--path and -r/--row is provided.
/// - test-suite:   Only requires the mandatory arguments.
#[derive(Debug, Parser)]
struct Cli {
    /// The mode of the the test file you want to use.
    #[arg(short, long)]
    mode: Mode,
    /// Path to the file in which the test(s) are that you are interested.
    #[arg(short, long)]
    path: Option<PathBuf>,
    /// Line number from where to start searching the nearest test.
    #[arg(short, long)]
    cursor_line: Option<usize>,
    /// Language the current file is in, used to select the runner.
    #[arg(short, long, default_value = "rust")]
    language: String,
}
fn main() -> Result<()> {
    // setup of program
    config::init_config()?;
    let args = Cli::parse();

    // preparation and putting command together.
    tmux::verify_inside_tmux()?;
    let helix_variables = HelixVariables::new(args.path, args.language, args.cursor_line);
    let mut cmd_to_run = command_for_mode(&args.mode, &helix_variables)?;

    // run the command the user wants to be run.
    tmux::open_command_pane()?;
    cmd_to_run.output()?;

    if let Err(e) = save_command_as_last(args.mode, helix_variables) {
        eprintln!("Warning - Could not save last command: {e}");
    };

    Ok(())
}
