//! Deal with helix related things, like variables.
//!
//! TODO:
//! - The language can actually serve to get the regex stuff from the test runner and extract the function name.
//! - Useful error messages when something fails, we propagate these up!

use std::path::PathBuf;

use anyhow::{Result, anyhow};
use strum::IntoEnumIterator;

use crate::{
    CONFIG,
    commands::{regex_searches, schema::SchemaVariables},
};

/// Helix variables.
///
/// A general struct of all the helix variables that we want to use.
pub struct HelixVariables {
    /// The file path, if given.
    file_path: Option<PathBuf>,
    /// The language, if given.
    language: String,
    /// The cursor line, if given.
    cursor_line: Option<usize>,
}

impl HelixVariables {
    /// Create a new HelixVariables path from the input arguments.
    pub fn new(file_path: Option<PathBuf>, language: String, cursor_line: Option<usize>) -> Self {
        Self {
            file_path,
            language,
            cursor_line,
        }
    }

    /// Form a command from a given template.
    ///
    /// Take a command, replace all the variables that we allow in this command, and return a command.
    pub fn make_command_argument(&self, cmd: &str) -> Result<String> {
        let mut cmd = cmd.trim().to_string();
        for sv in SchemaVariables::iter() {
            let variable_name = sv.as_variable_name();
            // continue in for loop if this schema variable is not available.
            if cmd.find(variable_name).is_none() {
                continue;
            }

            // get replacement string.
            let replacement = match sv {
                SchemaVariables::FileName => self.get_file_name(true),
                SchemaVariables::FileNameNoSuffix => self.get_file_name(false),
                SchemaVariables::FilePath => self.get_file_path(true),
                SchemaVariables::FilePathNoSuffix => self.get_file_path(false),
                SchemaVariables::FunctionName => self.get_function_name(),
                SchemaVariables::CursorLine => self.get_cursor_line(),
            }?;

            // replace all occurences of this variable name
            cmd = cmd.replace(variable_name, &replacement);
        }

        Ok(cmd.to_string())
    }

    /// Get the set language as a string.
    pub fn get_lang(&self) -> &str {
        self.language.as_str()
    }

    /// Get the file name as a string.
    ///
    /// Arguments:
    /// - with_suffix: Should the suffix be kept?
    ///
    /// Errors:
    /// - File path is not set.
    /// - File name contains invalid UTF-8 characters.
    fn get_file_name(&self, with_suffix: bool) -> Result<String> {
        let pth = self.file_path.as_ref().ok_or(anyhow!(
            "The path to the file of interest must be given with the -p/--path argument."
        ))?;

        let res = if with_suffix {
            pth.file_name()
        } else {
            pth.file_stem()
        }
        .ok_or(anyhow!("An invalid path was provided."))?;

        let res_str = res.to_str().ok_or(anyhow!(
            "Provided path contains invalid UTF-8 characters: {}",
            pth.to_string_lossy()
        ))?;

        Ok(res_str.to_string())
    }

    /// Get the file path as a string.
    ///
    /// Arguments:
    /// - with_suffix: Should the suffix be kept?
    ///
    /// Errors:
    /// - File path is not set.
    /// - File path contains invalid UTF-8 characters.
    fn get_file_path(&self, with_suffix: bool) -> Result<String> {
        let mut pth = self.file_path.clone().ok_or(anyhow!(
            "The path to the file of interest must be given with the -p/--path argument."
        ))?;

        if !with_suffix {
            pth = pth.as_path().with_extension("");
        }

        pth.into_string().map_err(|pth| {
            anyhow!(
                "Provided path contains invalid UTF-8 characters: {}",
                pth.to_string_lossy()
            )
        })
    }

    /// Get the function name of the nearest function (above current cursor line).
    fn get_function_name(&self) -> Result<String> {
        let file_path = self.file_path.as_ref().ok_or(anyhow!(
            "The path to the file of interest must be given with the -p/--path argument.",
        ))?;

        // get at the Regex value.
        // FIXME: This should be done somewhere else, i.e., some module for test runners access!
        let test_runners = &CONFIG
            .get()
            .expect("config is initialized by now")
            .test_runners;
        let schema = test_runners.get(self.get_lang()).ok_or(anyhow!(
            "Could not find test runner for language: {}",
            self.get_lang()
        ))?;
        let pattern = schema.get_regex_nearest()?;

        regex_searches::find_nearest_function_name(
            file_path,
            &pattern,
            self.get_cursor_line_usize()?,
        )
    }

    /// Get the cursor line where we are currently at.
    ///
    /// Returns the cursor line as an `Result<String>`.
    ///
    /// Errors:
    /// - Cursor line not provided.
    fn get_cursor_line(&self) -> Result<String> {
        let cursor_line = self.cursor_line.ok_or(anyhow!(
            "Cursor line must be provided with -c/--cursor-line argument."
        ))?;
        Ok(format!("{cursor_line}"))
    }

    /// Get the cursor line where we are currently at as a usize.
    ///
    /// Returns the cursor line as an `Result<usize>`.
    ///
    /// Errors:
    /// - Cursor line not provided.
    fn get_cursor_line_usize(&self) -> Result<usize> {
        let cursor_line = self.cursor_line.ok_or(anyhow!(
            "Cursor line must be provided with -c/--cursor-line argument."
        ))?;
        Ok(cursor_line)
    }
}
