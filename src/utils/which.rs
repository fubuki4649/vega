use std::{path::PathBuf, process::Command};

use crate::{sh, utils::run_command::ShellReturn};

/// Searches for the specified command in the system's PATH.
pub fn which(cmd: &str) -> Option<PathBuf> {
    let result: ShellReturn = sh!("command -v {}", cmd);
    match result.err_code {
        0 => Some(result.stdout.trim().into()),
        _ => None,
    }
}
