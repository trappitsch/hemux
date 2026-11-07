//! Library with all the testrunners we know and can configure.

use anyhow::{Result, anyhow};
use strum_macros::EnumIter;

/// This is the structure we need for a test runner.
///
/// All possibilities are optional as not every test runner might have all the individual possibilities.
/// For each command, you can use the abreviations documented in [`SchemaVariables`].
#[derive(Debug, Clone)]
pub struct TestRunnerSchema {
    /// Command to run the whole test suite.
    pub test_suite: Option<String>,
    /// Command to run the test of a given file.
    pub test_file: Option<String>,
    /// Command to run a signle test.
    pub test_nearest: Option<String>,
    /// Regex pattern to find extract the function name when searching backwards from the current cursor.
    pub regex_function_name: Option<String>,
}

impl TestRunnerSchema {
    /// Get the test suit command, return error if it does not exist.
    pub fn get_test_suite(&self) -> Result<String> {
        self.test_suite.clone().ok_or(anyhow!(
            "Test runner command for testing the whole suite not set."
        ))
    }

    /// Get the test file command, return error if it does not exist.
    pub fn get_test_file(&self) -> Result<String> {
        self.test_file.clone().ok_or(anyhow!(
            "Test runner command for testing the whole file not set."
        ))
    }

    /// Get the test nearest function command, return error if it does not exist.
    pub fn get_test_nearest(&self) -> Result<String> {
        self.test_nearest.clone().ok_or(anyhow!(
            "Test runner command for testing the nearest function not set."
        ))
    }

    /// Get the regex command to find the nearest function, return error if it does not exist.
    pub fn get_regex_function_name(&self) -> Result<String> {
        self.regex_function_name
            .clone()
            .ok_or(anyhow!("Regex pattern to find nearest function not set."))
    }
}

/// Represents all possible schema variables.
///
/// The [`HelixVariables`] structure will replace all of these varibales a given command
/// with the actual values.
#[derive(Debug, Clone, EnumIter)]
pub enum SchemaVariables {
    /// `%{file_name}`: Filename only.
    FileName,
    /// `%{file_name_no_suffix}`: Filename stem, no extension.
    FileNameNoSuffix,
    /// `%{file_path}`: Relative path of the file and filename.
    FilePath,
    /// `%{file_path_no_suffix}`: Relative path of the file and filename without extension.
    FilePathNoSuffix,
    /// `%{function_name}`: Name of the nearest function as found by matchin regex pattern from cursor line up.
    FunctionName,
    /// `%{cursor_line}`: The line number the cursor is at when called.
    CursorLine,
}

impl SchemaVariables {
    /// Provide the variable name of a given schema variable.
    pub fn as_variable_name(&self) -> &str {
        match self {
            Self::FileName => "%{file_name}",
            Self::FileNameNoSuffix => "%{file_name_no_suffix}",
            Self::FilePath => "%{file_path}",
            Self::FilePathNoSuffix => "%{file_path_no_suffix}",
            Self::FunctionName => "%{function_name}",
            Self::CursorLine => "%{cursor_line}",
        }
    }
}
