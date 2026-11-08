//! Hemux
//!
//! TODO:
//!
//! - Fix help documentation such that it is nicely displayed and useful.
//! - Add support to select test runner, add Nextest for rust.
//! - Add support for user commands depending on language.
//! - Add python support.
//!
//! TODO: Running tests
//!
//! - Running test suite in Rust works fine.
//! - Running test file works meh: tests in `lib`, `main` will not work
//! - Running nearest test: Just searches for the function name, no module name currently.
//! - Add a "run nearest module" as an alternative? Very rust centric.
//!
//! NOTE:
//!
//! - Buffer path contains `src` for Rust -> that's not really what we need for test
//!   - Does it always contian that, is it always with respect to the WD?
//! - The test runners will be highly language specific, probably not configuratble.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};

use crate::{
    commands::{command_for_mode, helix::HelixVariables},
    config::HemuxConfig,
};
pub use config::CONF;

mod commands;
mod config;
mod tmux;

/// The mode we might run the tests in.
#[derive(Debug, Clone, ValueEnum)]
pub enum Mode {
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
    // load and set the config
    let hemux_config: HemuxConfig = confy::load("hemux", Some("config"))
        .with_context(|| "Cannot acces the hemux configuration file.")?;
    CONF.set(hemux_config)
        .expect("first initialization of config; please file a bug report");
    config::init_config()?;

    let args = Cli::parse();
    // Verify we are in a tmux session.
    tmux::verify_inside_tmux()?;

    let helix_variables = HelixVariables::new(args.path, args.language, args.cursor_line);

    let mut cmd_to_run = command_for_mode(args.mode, helix_variables)?;

    tmux::open_command_pane()?;
    cmd_to_run.output()?;

    Ok(())
}
