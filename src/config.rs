//! Holds the config of the whole program.

use std::{collections::HashMap, sync::OnceLock};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::{commands::schema::TestRunnerSchema, tmux::TmuxConfig};

pub static CONF: OnceLock<HemuxConfig> = OnceLock::new();
pub static TEST_RUNNERS: OnceLock<HashMap<String, TestRunnerSchema>> = OnceLock::new();

/// Hemux program configuration.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HemuxConfig {
    pub tmux_config: TmuxConfig,
}

/// Initialize the public configuration.
///
/// This can only be done once and should be done right after argument parsing.
/// TODO: This should read from the actual configuration file!
pub fn init_config() -> Result<()> {
    let mut test_runners = HashMap::new();

    let key = String::from("rust");
    let value = TestRunnerSchema {
        test_suite: Some("cargo test".into()),
        test_file: Some("cargo test %{file_name_no_suffix}".into()),
        test_nearest: Some("cargo test %{function_name}".into()),
        regex_function_name: Some(r"(?:.*\s+|^)fn\s+([A-Za-z_][A-Za-z0-9_]*)".into()),
    };
    test_runners.insert(key, value);
    TEST_RUNNERS
        .set(test_runners)
        .map_err(|_| anyhow!("const test_runners hashmap can only be initialized once"))?;
    Ok(())
}
