//! Holds the config of the whole program.

use std::{collections::HashMap, sync::OnceLock};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{commands::schema::TestRunnerSchema, tmux::TmuxConfig};

pub static CONFIG: OnceLock<HemuxConfig> = OnceLock::new();

/// Hemux program configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct HemuxConfig {
    pub tmux_config: TmuxConfig,
    pub test_runners: HashMap<String, TestRunnerSchema>,
}

impl Default for HemuxConfig {
    fn default() -> Self {
        let test_runner_rust = TestRunnerSchema {
            test_suite: Some("cargo test".into()),
            test_file: Some("cargo test %{file_name_no_suffix}".into()),
            test_nearest: Some("cargo test %{function_name}".into()),
            regex_nearest: Some(r"(?:.*\s+|^)fn\s+([A-Za-z_][A-Za-z0-9_]*)".into()),
        };

        let mut test_runners = HashMap::new();
        test_runners.insert("rust".to_string(), test_runner_rust);

        Self {
            tmux_config: Default::default(),
            test_runners,
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
