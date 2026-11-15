//! Deal with language-specific and non-language specific shell commands that are not tests.
//!
//! These are basically user specific commands. The user can specify a command with a given name and then
//! use the passed `HelixArgs` to act  on the command.

use anyhow::{Result, anyhow};
use clap::Args;
use serde::{Deserialize, Serialize};

use crate::{
    CONFIG,
    commands::{helix::HelixArgs, schema::CommandRunnerSchema},
};

/// Arguments that you need to pass when providing a user-specific command.
///
/// This is the command structure for language specific and general commands.
/// If the `any_lang` flag is set, the command runner will serch in the language agnostic hashmap.
/// If the `any_lang` flag is not set, the command runner will search in the language specific hashmap
/// by using the language provided as part of `HelixArgs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Args)]
pub struct CmdArgs {
    /// The name of the command that, as specified in the hemux configuration file.
    #[arg(short, long)]
    name: String,
    /// Are we dealing with any language or a specific language?
    #[arg(short, long)]
    any_lang: bool,
    /// The Helix arguments that we might need.
    #[command(flatten)]
    helix_args: HelixArgs,
}

impl CmdArgs {
    /// Process a argument commands and return an actual command string.
    pub fn process_cmd_args(&self) -> Result<String> {
        let cmd_schema = self.get_command_schema()?;
        self.helix_args.make_command_argument(&cmd_schema.command)
    }

    fn get_command_schema(&self) -> Result<&CommandRunnerSchema> {
        if self.any_lang {
            let command_any_lang = &CONFIG
                .get()
                .expect("config is initialized by now")
                .command_any_lang;
            command_any_lang.get(&self.name).ok_or(anyhow!(
                "Could not find language agnostic command with name: {}",
                self.name
            ))
        } else {
            let lang = self.helix_args.get_lang();
            let command = &CONFIG.get().expect("config is initialized by now").command;
            let command_lang = command.get(lang).ok_or(anyhow!(
                "Could not find language specific commands for {}",
                lang
            ))?;
            command_lang.get(&self.name).ok_or(anyhow!(
                "Could not find command with name {} for language {}.",
                self.name,
                lang
            ))
        }
    }
}
