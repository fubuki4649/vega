use crate::_utils::run_command::ShellReturn;
use crate::data::hardware::hardware::get_hardware_info;
use crate::data::software::software::get_software_info;
use crate::sh;
use colored::Colorize;
use std::process::Command;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub fn get_system_info() -> Vec<String> {

    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::nothing().with_ram())
    );

    let mut lines: Vec<String> = Vec::with_capacity(19);
    
    lines.append(&mut get_title());
    lines.append(&mut get_hardware_info(&mut sys));
    lines.push(String::new());
    lines.append(&mut get_software_info());
    
    lines
    
}


fn get_title() -> Vec<String> {

    let username = sh!("whoami").stdout.trim().to_string();
    let hostname = sh!("uname -n").stdout.trim().to_string();

    let title = format!("{}@{}", username.blue().bold(), hostname.blue().bold());
    let dashes = "=".repeat(username.len() + hostname.len() + 1);

    vec![title, dashes]

}