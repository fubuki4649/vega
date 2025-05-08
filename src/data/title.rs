use crate::_utils::run_command::ShellReturn;
use crate::sh;
use std::process::Command;
use colored::Colorize;

pub fn get_title() -> Vec<String> {

    let username = sh!("whoami").stdout.trim().to_string();
    let hostname = sh!("uname -n").stdout.trim().to_string();
    
    let title = format!("{}@{}", username.blue().bold(), hostname.blue().bold());
    let dashes = "=".repeat(username.len() + hostname.len() + 1);

    vec![title, dashes]
    
}