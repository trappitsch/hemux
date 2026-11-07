//! Handle tmux.

use std::process::Command;

use anyhow::{Context, Result, anyhow};

/// Verify if we inside a tmux environment.
///
/// Checks if the `TMUX` environmental variable is set.
/// If it is not set, this will return an error.
pub fn verify_inside_tmux() -> Result<()> {
    std::env::var("TMUX").map_err(|_| anyhow!("Not inside tmux environment."))?;
    Ok(())
}

/// Open a second pane if necessary.
///
/// This checks if multiple panes are already open. If not, a horizontal split
/// is done with the bottom pane of 25% the height of the top one.
pub fn open_command_pane() -> Result<()> {
    let tmux_window_panes = Command::new("tmux")
        .args(["display-message", "-p", "#{window_panes}"])
        .output()
        .context("could not determine the number of open tmux panes in this window")?;

    let num_panes = String::from_utf8(tmux_window_panes.stdout)
        .context("number of window panes contains non-UTF8 characters")?
        .trim()
        .parse::<u8>()
        .context("could not parse number of window panes")?;

    if num_panes < 2 {
        Command::new("tmux")
            .args(["split-window", "-v", "-p", "30", "-d"])
            .output()
            .context("could not open a second tmux window pane")?;
    }
    Ok(())
}
