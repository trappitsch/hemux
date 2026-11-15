//! Commands and their implementation.

use std::process::Command;

use anyhow::Result;

use crate::HemuxCommand;
use crate::commands::last_cmd::get_last_command;

pub mod helix;
pub mod last_cmd;
pub mod regex_searches;
pub mod schema;
pub mod shell_cmd;

/// Create the [`std::process::Command`] from the hemux command..
pub fn create_command(cmd: &HemuxCommand) -> Result<Command> {
    let cmd_str = match cmd {
        HemuxCommand::Cmd(cmd_args) => cmd_args.process_cmd_args()?,
        HemuxCommand::Last => return get_last_command(),
        HemuxCommand::TestSuite(helix_args) => {
            let schema = helix_args.get_test_runner_schema()?;
            helix_args.make_command_argument(&schema.get_test_suite()?)?
        }
        HemuxCommand::TestFile(helix_args) => {
            let schema = helix_args.get_test_runner_schema()?;
            helix_args.make_command_argument(&schema.get_test_file()?)?
        }
        HemuxCommand::TestNearest(helix_args) => {
            let schema = helix_args.get_test_runner_schema()?;
            helix_args.make_command_argument(&schema.get_test_nearest()?)?
        }
    };

    Ok(tmux_send_key_cmd(&cmd_str))
}

/// Create a tmux command that can be run in another pane and return it.
///
/// For good measure, we strip surrounding single and double quotes from the command.
fn tmux_send_key_cmd(cmd: &str) -> Command {
    let mut tmux_cmd = Command::new("tmux");
    tmux_cmd.args(["send-key", "-t", ":.+"]);

    let cmd = cmd.trim().trim_matches('\'').trim_matches('"');
    tmux_cmd.arg(cmd);

    tmux_cmd.arg("Enter");

    tmux_cmd
}
