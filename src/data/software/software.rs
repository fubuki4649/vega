use colored::Colorize;
use crate::data::software::individual_stats::{get_ip_addr, get_kernel, get_os, get_packages, get_shell, get_terminal, get_uptime, get_window_manager};

pub fn get_software_info() -> Vec<String> {
    vec![
        format!("{}", "< Software >".bold()),
        format!("{}{}", "╔ OS  = ".blue().bold(), get_os().dimmed()),
        format!("{}{}", "╠ KER = ".blue().bold(), get_kernel().dimmed()),
        format!("{}{}", "╠ UPT = ".blue().bold(), get_uptime().dimmed()),
        format!("{}{}", "╠ PKG = ".blue().bold(), get_packages().dimmed()),
        format!("{}{}", "╠ WMN = ".blue().bold(), get_window_manager().dimmed()),
        format!("{}{}", "╠ TER = ".blue().bold(), get_terminal().dimmed()),
        format!("{}{}", "╠ SHL = ".blue().bold(), get_shell().dimmed()),
        format!("{}{}", "╚ IP4 = ".blue().bold(), get_ip_addr().dimmed()),
    ]
}