//! Functions to save and reload last commands.

use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::{HelixArgs, HemuxCommand, commands::create_command};

const LAST_COMMAND_CONFNAME: &str = "last_command";

impl Default for HemuxCommand {
    fn default() -> Self {
        HemuxCommand::TestSuite(HelixArgs::default())
    }
}

/// Save a given command as the last command (assuming it is any other command than last)
pub fn save_command_as_last(cmd: HemuxCommand) -> Result<()> {
    // avoid the recursive nightmare ;)
    if cmd == HemuxCommand::Last {
        return Ok(());
    }

    confy::store("hemux", LAST_COMMAND_CONFNAME, cmd)?;
    Ok(())
}

pub fn get_last_command() -> Result<Command> {
    let last: HemuxCommand = confy::load("hemux", LAST_COMMAND_CONFNAME)
        .with_context(|| "Could not load file that contains last command.")?;

    if last == HemuxCommand::Last {
        bail!("Last command contains a call to last command. Please report this as a bug.")
    }

    create_command(&last)
}
