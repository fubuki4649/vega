use colored::Colorize;

use crate::data::software::individual_stats;

/// Retrieves software information such as OS, kernel, uptime, packages, window manager,
pub fn get_software_info() -> Vec<String> {
    vec![
        format!("{}", "< Software >".bold()),
        format!(
            "{}{}",
            "╔ OS  = ".blue().bold(),
            individual_stats::get_os().dimmed()
        ),
        format!(
            "{}{}",
            "╠ KER = ".blue().bold(),
            individual_stats::get_kernel().dimmed()
        ),
        format!(
            "{}{}",
            "╠ UPT = ".blue().bold(),
            individual_stats::get_uptime().dimmed()
        ),
        format!(
            "{}{}",
            "╠ PKG = ".blue().bold(),
            individual_stats::get_packages().dimmed()
        ),
        format!(
            "{}{}",
            "╠ WMN = ".blue().bold(),
            individual_stats::get_window_manager().dimmed()
        ),
        format!(
            "{}{}",
            "╠ TER = ".blue().bold(),
            individual_stats::get_terminal().dimmed()
        ),
        format!(
            "{}{}",
            "╠ SHL = ".blue().bold(),
            individual_stats::get_shell().dimmed()
        ),
        format!(
            "{}{}",
            "╚ IP4 = ".blue().bold(),
            individual_stats::get_ip_addr().dimmed()
        ),
    ]
}
