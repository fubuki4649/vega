use colored::Colorize;
use sysinfo::System;
use crate::data::hardware::individual_stats::{get_cpu, get_drive, get_gpu, get_model, get_ram, get_screen_res};

pub fn get_hardware_info(sys: &mut System) -> Vec<String> {
    vec![
        format!("{}", "< Hardware >".bold()),
        format!("{}{}", "╔ MDL = ".blue().bold(), get_model().dimmed()),
        format!("{}{}", "╠ CPU = ".blue().bold(), get_cpu().dimmed()),
        format!("{}{}", "╠ GPU = ".blue().bold(), get_gpu().dimmed()),
        format!("{}{}", "╠ RAM = ".blue().bold(), get_ram(sys).dimmed()),
        format!("{}{}", "╠ HDD = ".blue().bold(), get_drive().dimmed()),
        format!("{}{}", "╚ RES = ".blue().bold(), get_screen_res().dimmed()),
    ]
}