//! Hemux
//!
//! TODO:
//!
//! - Fix help documentation such that it is nicely displayed and useful.
//! - Add support for user commands depending on language.
//! - Add python support.

use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

pub use config::CONFIG;

use crate::commands::{create_command, helix::HelixArgs, last_cmd::save_command_as_last};

mod commands;
mod config;
mod tmux;

/// The hemux command that can be run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Subcommand)]
pub enum HemuxCommand {
    /// Run the whole test suite (default).
    TestSuite(HelixArgs),
    /// Test a single file.
    TestFile(HelixArgs),
    /// Test the nearest test.
    TestNearest(HelixArgs),
    /// Run the last test or command.
    Last,
}

/// Helix CLI
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The subcommand possivilities.
    #[command(subcommand)]
    command: Option<HemuxCommand>,
}
fn main() -> Result<()> {
    // setup of program
    config::init_config()?;
    let args = Cli::parse();

    // preparation and putting command together.
    tmux::verify_inside_tmux()?;

    if let Some(hemux_cmd) = args.command {
        let mut cmd_to_run = create_command(&hemux_cmd)?;

        tmux::open_command_pane()?;
        cmd_to_run.output()?;

        if let Err(e) = save_command_as_last(hemux_cmd) {
            eprintln!("Warning - Could not save last command: {e}");
        };
        Ok(())
    } else {
        bail!("A subcommand must be provided")
    }
    // let mut cmd_to_run = create_command(&args.command)?;

    // // run the command the user wants to be run.
}
