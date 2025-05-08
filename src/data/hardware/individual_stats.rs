use std::process::Command;
use crate::_utils::run_command::ShellReturn;
use crate::_utils::read_file::cat;
use crate::sh;


pub fn get_model() -> String {

    let model = cat("/sys/devices/virtual/dmi/id/product_name").trim().to_string();
    if model.is_empty() { "Not Supported".to_string() } else { model }

}

pub fn get_cpu() -> String {

    let [cpu, corecount] = match sh!("uname").stdout.trim() {
        "Linux" => {
            let cpuname = sh!("awk -F: '/model name/ {{print $2; exit}}' /proc/cpuinfo | sed 's/^ //'").stdout;
            let nproc = sh!("nproc");

            if nproc.err_code == 0 {
                [cpuname, nproc.stdout]
            } else {
                [cpuname, sh!("grep '^processor' /proc/cpuinfo | wc -l").stdout]
            }
        },
        "FreeBSD" => [sh!("sysctl -n hw.model").stdout, sh!("sysctl -n hw.ncpu").stdout],
        "Darwin" => [sh!("sysctl -n hw.machdep.cpu.brand_string").stdout, sh!("sysctl -n hw.ncpu").stdout],
        _ => ["Not Supported".to_string(), "0".to_string()]
    };

    format!("{} ({})", cpu.trim(), corecount.trim())

}

pub fn get_gpu() -> String {
    "Coming Soon!".to_string()
}

pub fn get_ram() -> String {
    "Coming Soon!".to_string()
}

pub fn get_drive() -> String {
    "Coming Soon!".to_string()
}

pub fn get_screen_res() -> String {
    "Coming Soon!".to_string()
}