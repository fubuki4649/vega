use colored::Colorize;
use sysinfo::System;

use crate::data::hardware::individual_stats;

/// Retrieves hardware information such as model, CPU, GPU, RAM, HDD, and screen resolution.
pub fn get_hardware_info(sys: &mut System) -> Vec<String> {
    vec![
        format!("{}", "< Hardware >".bold()),
        format!(
            "{}{}",
            "╔ MDL = ".blue().bold(),
            individual_stats::get_model().dimmed()
        ),
        format!(
            "{}{}",
            "╠ CPU = ".blue().bold(),
            individual_stats::get_cpu().dimmed()
        ),
        format!(
            "{}{}",
            "╠ GPU = ".blue().bold(),
            individual_stats::get_gpu().dimmed()
        ),
        format!(
            "{}{}",
            "╠ RAM = ".blue().bold(),
            individual_stats::get_ram(sys).dimmed()
        ),
        format!(
            "{}{}",
            "╠ HDD = ".blue().bold(),
            individual_stats::get_drive().dimmed()
        ),
        format!(
            "{}{}",
            "╚ RES = ".blue().bold(),
            individual_stats::get_screen_res().dimmed()
        ),
    ]
}
