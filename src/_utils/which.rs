use std::process::Command;
use crate::_utils::run_command::ShellReturn;
use std::path::PathBuf;
use crate::sh;

pub fn which(cmd: &str) -> Option<PathBuf> {
    let result = sh!("command -v {}", cmd);
    match result.err_code {
        0 => Some(result.stdout.trim().into()),
        _ => None,
    }
}