//! Functions to save and reload last commands.

use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::{
    Mode,
    commands::{command_for_mode, helix::HelixVariables},
};

const LAST_COMMAND_CONFNAME: &str = "last_command";

/// Command to run configuration.
///
/// This stores the configuration that has last time successfully produced
/// a `cmd_to_run`. So we should be able to do it again.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommandToRun {
    mode: Mode,
    helix_variables: HelixVariables,
}

impl Default for CommandToRun {
    fn default() -> Self {
        Self {
            mode: Mode::TestSuite,
            helix_variables: HelixVariables::new(None, "rust".to_string(), None),
        }
    }
}

/// Save a given command as the last command (assuing it is any other mode than last)
pub fn save_command_as_last(mode: Mode, helix_variables: HelixVariables) -> Result<()> {
    // avoid the recursive nightmare ;)
    if mode == Mode::Last {
        return Ok(());
    }

    let ctr = CommandToRun {
        mode,
        helix_variables,
    };

    confy::store("hemux", LAST_COMMAND_CONFNAME, ctr)?;
    Ok(())
}

pub fn get_last_command() -> Result<Command> {
    let last: CommandToRun = confy::load("hemux", LAST_COMMAND_CONFNAME)
        .with_context(|| "Could not load file that contains last command.")?;

    // safety check, should not be reachable.
    if last.mode == Mode::Last {
        bail!("Last command contains a call to last command. Please report this as a bug.")
    }

    command_for_mode(&last.mode, &last.helix_variables)
}
