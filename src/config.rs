//! Holds the config of the whole program.

use std::{collections::HashMap, sync::OnceLock};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{
    commands::schema::{CommandRunnerSchema, TestRunnerSchema},
    tmux::TmuxConfig,
};

pub static CONFIG: OnceLock<HemuxConfig> = OnceLock::new();

/// Hemux program configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct HemuxConfig {
    /// Tmux configuration.
    pub tmux_config: TmuxConfig,
    /// Test runners that are available.
    pub test_runners: HashMap<String, TestRunnerSchema>,
    /// Command runners for all languages (language agnostic).
    pub command_any_lang: HashMap<String, CommandRunnerSchema>,
    /// Language specific command runners. Key contains the name of the (language, command name), both as `String`.
    pub command: HashMap<String, HashMap<String, CommandRunnerSchema>>,
}

impl Default for HemuxConfig {
    fn default() -> Self {
        let test_runner_rust = TestRunnerSchema {
            test_suite: Some("cargo nextest run".into()),
            test_file: Some("cargo nextest run %{file_name_no_suffix}".into()),
            test_nearest: Some("cargo nextest run %{function_name}".into()),
            regex_nearest: Some(r"(?:.*\s+|^)fn\s+([A-Za-z_][A-Za-z0-9_]*)".into()),
        };

        let mut test_runners = HashMap::new();
        test_runners.insert("rust".to_string(), test_runner_rust);

        // commands for any language
        let mut command_any_lang = HashMap::new();
        command_any_lang.insert(
            "info".to_string(),
            CommandRunnerSchema::new(
                "echo file_name: %{file_name} ; echo file_path: %{file_path} ; echo cursor_line: %{cursor_line}"
            )
        );

        // build up Rust commands:
        let mut command_rust = HashMap::new();
        command_rust.insert("run".to_string(), CommandRunnerSchema::new("cargo run"));
        command_rust.insert(
            "run-alt".to_string(),
            CommandRunnerSchema::new("cargo run -r"),
        );
        command_rust.insert("build".to_string(), CommandRunnerSchema::new("cargo build"));
        command_rust.insert(
            "build-alt".to_string(),
            CommandRunnerSchema::new("cargo build -r"),
        );

        let command = HashMap::from([("rust".to_string(), command_rust)]);

        Self {
            tmux_config: Default::default(),
            test_runners,
            command_any_lang,
            command,
        }
    }
}

// Initialize the configuration.
//
// This MUST be run at the very beginning of the program!
pub fn init_config() -> Result<()> {
    let hemux_config: HemuxConfig = confy::load("hemux", Some("config"))
        .with_context(|| "Cannot acces the hemux configuration file.")?;
    CONFIG
        .set(hemux_config)
        .expect("first initialization of config");
    Ok(())
}
