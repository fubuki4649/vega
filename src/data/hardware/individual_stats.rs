use std::ffi::CString;
use std::mem::zeroed;
use libc::{statvfs, statvfs as Statvfs};
use crate::_utils::read_file::cat;
use crate::_utils::run_command::ShellReturn;
use crate::sh;
use std::process::Command;
use sysinfo::{MemoryRefreshKind, System};


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

pub fn get_ram(sys: &mut System) -> String {
    sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());

    format!("{}MB / {}MB", sys.used_memory() / 1048576, sys.total_memory() / 1048576)
}

pub fn get_drive() -> String {
    let path = CString::new("/").unwrap();
    let mut stat: Statvfs = unsafe { zeroed() };

    let result = unsafe { statvfs(path.as_ptr(), &mut stat) };
    if result != 0 {
        return "Failed / Not Supported".to_string();
    }

    let total_space = stat.f_blocks as u64 * stat.f_frsize as u64;
    let free_space = stat.f_bfree as u64 * stat.f_frsize as u64;
    let used_space = total_space - free_space;

    format!("{}GB / {}GB", used_space / 1073741824, total_space / 1073741824)
}

pub fn get_screen_res() -> String {
    let screenres = sh!("head -n1 -q /sys/class/drm/*/modes");
    let res = screenres.stdout.trim().to_string();
    
    if screenres.err_code == 0 && !res.is_empty() {
        res
    } else {
        "None".to_string()
    }
}