//! Commands and their implementation.

use std::process::Command;

use anyhow::{Result, anyhow};

use crate::commands::last_cmd::get_last_command;
use crate::commands::schema::TestRunnerSchema;
use crate::{CONFIG, HemuxCommand};

pub mod helix;
pub mod last_cmd;
pub mod regex_searches;
pub mod schema;

/// Create the [`std::process::Command`] from the hemux command..
pub fn create_command(cmd: &HemuxCommand) -> Result<Command> {
    let cmd_str = match cmd {
        HemuxCommand::Last => return get_last_command(),
        HemuxCommand::TestSuite(helix_args) => {
            let schema = get_test_runner_schema(helix_args.get_lang())?;
            helix_args.make_command_argument(&schema.get_test_suite()?)?
        }
        HemuxCommand::TestFile(helix_args) => {
            let schema = get_test_runner_schema(helix_args.get_lang())?;
            helix_args.make_command_argument(&schema.get_test_file()?)?
        }
        HemuxCommand::TestNearest(helix_args) => {
            let schema = get_test_runner_schema(helix_args.get_lang())?;
            helix_args.make_command_argument(&schema.get_test_nearest()?)?
        }
    };

    Ok(tmux_send_key_cmd(&cmd_str))
}

fn get_test_runner_schema(language: &str) -> Result<&TestRunnerSchema> {
    let test_runners = &CONFIG
        .get()
        .expect("config is initialized by now")
        .test_runners;

    let schema = test_runners.get(&language.to_lowercase()).ok_or(anyhow!(
        "Could not find test runner for language: {}",
        language
    ))?;
    Ok(schema)
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
