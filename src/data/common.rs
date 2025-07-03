use std::process::Command;

use colored::Colorize;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use crate::{
    data::{hardware::hardware, software::software},
    sh,
    utils::run_command::ShellReturn,
};

/// Retrieves system information including hardware and software details.
pub fn get_system_info() -> impl Iterator<Item = String> {
    let mut sys: System = System::new_with_specifics(
        RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
    );

    let mut lines: Vec<String> = Vec::with_capacity(19);

    lines.append(&mut get_title());
    lines.append(&mut hardware::get_hardware_info(&mut sys));
    lines.push(String::new());
    lines.append(&mut software::get_software_info());

    lines.into_iter()
}

fn get_title() -> Vec<String> {
    let username: String = sh!("whoami").stdout.trim().to_string();
    let hostname: String = sh!("uname -n").stdout.trim().to_string();

    let title: String = format!("{}@{}", username.blue().bold(), hostname.blue().bold());
    let dashes: String = "=".repeat(username.len() + hostname.len() + 1);

    vec![title, dashes]
}
