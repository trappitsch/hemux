//! Do regex searches and return the results.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::{Context, Result, bail};
use regex::Regex;

/// Search file for first match upwards from line number.
///
/// For a given file name (and path), this function will read the file and start searching
/// for a given regex pattern from (and including) the provided line number upwards.
/// If found, the first matching group (not the total match) will be returned as a [`String`].
///
/// Errors:
/// - File does not exist.
/// - Did not find a match.
pub fn find_nearest_function_name(
    fname: &PathBuf,
    regex: &str,
    line_number: usize,
) -> Result<String> {
    let rgx = Regex::new(regex)
        .with_context(|| format!("Failed to create regex from pattern: {}", regex))?;

    let file = File::open(fname).with_context(|| format!("Failed to open file: {:?}", fname))?;
    let reader = BufReader::new(file);

    let mut matches = vec![];

    for (ln, line) in reader.lines().enumerate() {
        if ln > line_number {
            break;
        }

        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        if !rgx.is_match(&line) {
            continue;
        }

        if let Some(cpt) = rgx.captures(&line).and_then(|c| c.get(1)) {
            matches.push(cpt.as_str().to_string());
        }
    }

    if matches.is_empty() {
        bail!("No nearest function found.")
    } else {
        Ok(matches.last().expect("vec is not empty").into())
    }
}
