//! Handle tmux.

use std::process::Command;

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::CONF;

/// The Tmux configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct TmuxConfig {
    /// In which direction should a new tmux window open (if none is open yet?)
    split_direction: SplitDirection,
    /// Split percentage of the new window: must be between 5% and 95%.
    split_percentage: u8,
}

impl Default for TmuxConfig {
    fn default() -> Self {
        Self {
            split_direction: Default::default(),
            split_percentage: 30,
        }
    }
}

impl TmuxConfig {
    /// Get split direction flag.
    fn get_split_direction_flag(&self) -> String {
        match self.split_direction {
            SplitDirection::Horizontal => "-h".to_string(),
            SplitDirection::Vertical => "-v".to_string(),
        }
    }

    /// Get split percentage, clamped to 5,95.
    fn get_split_percentage(&self) -> String {
        self.split_percentage.clamp(5, 95).to_string()
    }
}

/// Window split direction.
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum SplitDirection {
    /// Split horizontally, i.e., new pane is to the right.
    Horizontal,
    /// Split vertically, i.e., new pane is to the below.
    #[default]
    Vertical,
}

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
    let conf = CONF
        .get()
        .expect("config file loading initialized at beginning");

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
            .args([
                "split-window",
                conf.tmux_config.get_split_direction_flag().as_str(),
                "-p",
                conf.tmux_config.get_split_percentage().as_str(),
                "-d", // keep focus in main window that called this command
            ])
            .output()
            .context("could not open a second tmux window pane")?;
    }
    Ok(())
}
