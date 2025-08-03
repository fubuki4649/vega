use std::{ffi::CString, mem, process::Command};

use libc::{statvfs, statvfs as Statvfs};
use pci_ids::{FromId, Vendor};
use pci_info::{PciInfo, PciInfoError, pci_enums::PciDeviceClass};
use sysinfo::{MemoryRefreshKind, System};

use crate::{
    sh,
    utils::{read_file::cat, run_command::ShellReturn},
};

/// Retrieves the system model name.
pub fn get_model() -> String {
    match sh!("uname").stdout.trim() {
        "Linux" => cat("/sys/devices/virtual/dmi/id/product_name")
            .trim()
            .to_string(),
        "FreeBSD" => {
            sh!("grep -i \"smbios: product\" /var/run/dmesg.boot | sed 's/.*[Pp]roduct: *//'")
                .stdout
                .trim()
                .to_string()
        }
        "Darwin" => sh!("sysctl hw.model | awk '{{print $2}}'")
            .stdout
            .trim()
            .to_string(),
        _ => "Not Supported".to_string(),
    }
}

/// Retrieves the CPU model and core count.
pub fn get_cpu() -> String {
    let [cpu, corecount] = match sh!("uname").stdout.trim() {
        "Linux" => {
            let cpuname: String =
                sh!("awk -F: '/model name/ {{print $2; exit}}' /proc/cpuinfo | sed 's/^ //'")
                    .stdout;
            let nproc: ShellReturn = sh!("nproc");

            if nproc.err_code == 0 {
                [cpuname, nproc.stdout]
            } else {
                [
                    cpuname,
                    sh!("grep '^processor' /proc/cpuinfo | wc -l").stdout,
                ]
            }
        }
        "FreeBSD" => [
            sh!("sysctl -n hw.model").stdout,
            sh!("sysctl -n hw.ncpu").stdout,
        ],
        "Darwin" => [
            sh!("sysctl -n machdep.cpu.brand_string").stdout,
            sh!("sysctl -n hw.ncpu").stdout,
        ],
        _ => ["Not Supported".to_string(), "0".to_string()],
    };

    format!("{} ({})", cpu.trim(), corecount.trim())
}

/// Retrieves the GPU model and vendor.
pub fn get_gpu() -> String {
    // Enumerate the devices on the PCI bus
    let info: Result<PciInfo, PciInfoError> = PciInfo::enumerate_pci();

    if let Ok(devices) = info {
        for r in devices {
            if let Ok(device) = r {
                // Ignores non-gpus
                if device
                    .device_class()
                    .unwrap_or(PciDeviceClass::Unclassified)
                    == PciDeviceClass::DisplayController
                {
                    // Extracts user-friendly strings for the first GPU
                    let vendor: Option<&'static Vendor> = Vendor::from_id(device.vendor_id());
                    if let Some(vendor) = vendor {
                        for d in vendor.devices() {
                            if d.id() == device.device_id() {
                                return format!(
                                    "{} {} [{:04X}:{:04X}]",
                                    vendor.name(),
                                    d.name(),
                                    device.vendor_id(),
                                    device.device_id()
                                );
                            }
                        }
                    }
                }
            }
        }

        "None".to_string()
    } else {
        "Not Supported".to_string()
    }
}

/// Retrieves the amount of RAM in use and total RAM.
pub fn get_ram(sys: &mut System) -> String {
    sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());

    format!(
        "{}MB / {}MB",
        sys.used_memory() / 1048576,
        sys.total_memory() / 1048576
    )
}

/// Retrieves free space / total space of the root partition.
pub fn get_drive() -> String {
    let path: CString = CString::new("/").unwrap();
    let mut stat: Statvfs = unsafe { mem::zeroed() };

    let result: i32 = unsafe { statvfs(path.as_ptr(), &mut stat) };
    if result != 0 {
        return "Failed / Not Supported".to_string();
    }

    let total_space: u64 = stat.f_blocks as u64 * stat.f_frsize as u64;
    let free_space: u64 = stat.f_bfree as u64 * stat.f_frsize as u64;
    let used_space: u64 = total_space - free_space;

    format!(
        "{}GB / {}GB",
        used_space / 1073741824,
        total_space / 1073741824
    )
}

#[cfg(target_os = "macos")]
/// Retrieves the screen resolution for macOS.
pub fn get_screen_res() -> String {
    use core_graphics::display::{CGDisplay, CGDisplayPixelsHigh, CGDisplayPixelsWide};

    let screens: Vec<_> = CGDisplay::active_displays()
        .unwrap_or_default()
        .iter()
        .map(|&id| unsafe { format!("{}x{}", CGDisplayPixelsWide(id), CGDisplayPixelsHigh(id)) })
        .collect::<Vec<_>>();

    if screens.is_empty() {
        "None".to_string()
    } else {
        screens.join(" ")
    }
}

#[cfg(not(target_os = "macos"))]
/// Retrieves the screen resolution for Linux and FreeBSD.
pub fn get_screen_res() -> String {
    let screen_res: ShellReturn = match sh!("uname").stdout.trim() {
        "Linux" => sh!("head -n1 -q /sys/class/drm/*/modes | tr '\n' ' '"),
        "FreeBSD" => sh!("sysctl -n kern.vt.fb.default_mode"),
        _ => sh!("meow"),
    };
    let res: String = screen_res.stdout.trim().to_string();

    if screen_res.err_code == 0 && !res.is_empty() {
        res
    } else {
        "None".to_string()
    }
}
