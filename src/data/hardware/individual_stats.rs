use std::ffi::CString;
use std::mem::zeroed;
use libc::{statvfs, statvfs as Statvfs};
#[cfg(target_os = "linux")]
use pci_ids::{FromId, Vendor};

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
#[allow(dead_code)]
fn get_sysctl_int64(name: &str) -> Option<u64> {
    let cname = CString::new(name).ok()?;
    let mut val: u64 = 0;
    let mut size = std::mem::size_of::<u64>();
    let ret = unsafe {
        libc::sysctlbyname(cname.as_ptr(), (&raw mut val).cast(), &raw mut size, std::ptr::null_mut(), 0)
    };
    if ret == 0 { Some(val) } else { None }
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
#[allow(dead_code)]
fn get_sysctl_string(name: &str) -> Option<String> {
    let cname = CString::new(name).ok()?;
    let mut size = 0;
    unsafe {
        libc::sysctlbyname(cname.as_ptr(), std::ptr::null_mut(), &raw mut size, std::ptr::null_mut(), 0);
    }
    if size == 0 { return None; }
    let mut buf = vec![0u8; size];
    let ret = unsafe {
        libc::sysctlbyname(cname.as_ptr(), buf.as_mut_ptr().cast(), &raw mut size, std::ptr::null_mut(), 0)
    };
    if ret == 0 {
        let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        Some(String::from_utf8_lossy(&buf[..len]).trim().to_string())
    } else {
        None
    }
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
#[allow(dead_code)]
fn get_sysctl_int(name: &str) -> Option<i32> {
    let cname = CString::new(name).ok()?;
    let mut val: i32 = 0;
    let mut size = std::mem::size_of::<i32>();
    let ret = unsafe {
        libc::sysctlbyname(cname.as_ptr(), (&raw mut val).cast(), &raw mut size, std::ptr::null_mut(), 0)
    };
    if ret == 0 { Some(val) } else { None }
}

pub fn get_model() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(name) = std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        if let Ok(model) = std::fs::read_to_string("/sys/firmware/devicetree/base/model") {
            let trimmed = model.trim().trim_matches('\0');
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        "Unknown".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        get_sysctl_string("hw.model").unwrap_or_else(|| "Mac".to_string())
    }
    #[cfg(target_os = "freebsd")]
    {
        let vendor = get_sysctl_string("hw.vendor").unwrap_or_default();
        let product = get_sysctl_string("hw.product").unwrap_or_default();
        if !vendor.is_empty() || !product.is_empty() {
            format!("{vendor} {product}").trim().to_string()
        } else {
            get_sysctl_string("hw.model").unwrap_or_else(|| "FreeBSD Device".to_string())
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        "Not Supported".to_string()
    }
}

pub fn get_cpu() -> String {
    #[cfg(target_os = "linux")]
    {
        let mut cpu_name = String::new();
        let mut core_count = 0;

        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if cpu_name.is_empty() {
                    if let Some(name) = line.strip_prefix("model name").and_then(|val| val.split(':').nth(1)) {
                        cpu_name = name.trim().to_string();
                    } else if let Some(name) = line.strip_prefix("Hardware").and_then(|val| val.split(':').nth(1)) {
                        cpu_name = name.trim().to_string();
                    }
                }
                if line.starts_with("processor") {
                    core_count += 1;
                }
            }
        }

        if core_count == 0 {
            core_count = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
        }
        if cpu_name.is_empty() {
            cpu_name = "Unknown CPU".to_string();
        }

        format!("{cpu_name} ({core_count})")
    }
    #[cfg(target_os = "macos")]
    {
        let cpu_name = get_sysctl_string("machdep.cpu.brand_string").unwrap_or_else(|| "Apple Silicon / Intel".to_string());
        let core_count = get_sysctl_int("hw.ncpu").unwrap_or(1);
        format!("{cpu_name} ({core_count})")
    }
    #[cfg(target_os = "freebsd")]
    {
        let cpu_name = get_sysctl_string("hw.model").unwrap_or_else(|| "Unknown CPU".to_string());
        let core_count = get_sysctl_int("hw.ncpu").unwrap_or(1);
        format!("{cpu_name} ({core_count})")
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        "Not Supported (0)".to_string()
    }
}

pub fn get_gpu() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = std::fs::read_dir("/sys/bus/pci/devices") {
            for entry in entries.flatten() {
                let path = entry.path();
                let class_str = std::fs::read_to_string(path.join("class")).unwrap_or_default();
                let class_trimmed = class_str.trim();

                // 0x030000 = VGA, 0x030200 = 3D controller, 0x038000 = Display controller
                if class_trimmed.starts_with("0x03") {
                    let vendor_hex = std::fs::read_to_string(path.join("vendor")).unwrap_or_default();
                    let device_hex = std::fs::read_to_string(path.join("device")).unwrap_or_default();

                    let vendor_id = u16::from_str_radix(vendor_hex.trim().trim_start_matches("0x"), 16).unwrap_or(0);
                    let device_id = u16::from_str_radix(device_hex.trim().trim_start_matches("0x"), 16).unwrap_or(0);

                    if vendor_id > 0 {
                        if let Some(vendor) = Vendor::from_id(vendor_id) {
                            for d in vendor.devices() {
                                if d.id() == device_id {
                                    return format!("{} {} [{vendor_id:04X}:{device_id:04X}]", vendor.name(), d.name());
                                }
                            }
                            return format!("{} [{vendor_id:04X}:{device_id:04X}]", vendor.name());
                        }
                        return format!("[{vendor_id:04X}:{device_id:04X}]");
                    }
                }
            }
        }
        "None".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(cpu_brand) = get_sysctl_string("machdep.cpu.brand_string").filter(|s| s.starts_with("Apple")) {
            return format!("{cpu_brand} GPU");
        }
        "Apple Graphics".to_string()
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        "Not Supported".to_string()
    }
}

pub fn get_ram() -> String {
    #[cfg(target_os = "linux")]
    {
        let mut mem_total_kb = 0u64;
        let mut mem_avail_kb = 0u64;

        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if let Some(val) = line.strip_prefix("MemTotal:") {
                    mem_total_kb = val.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0);
                } else if let Some(val) = line.strip_prefix("MemAvailable:") {
                    mem_avail_kb = val.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0);
                }
                if mem_total_kb > 0 && mem_avail_kb > 0 {
                    break;
                }
            }
        }

        if mem_total_kb > 0 {
            let used_mb = mem_total_kb.saturating_sub(mem_avail_kb) / 1024;
            let total_mb = mem_total_kb / 1024;
            return format!("{used_mb}MB / {total_mb}MB");
        }
        "Unknown".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        unsafe extern "C" {
            fn mach_host_self() -> libc::mach_port_t;
        }

        let total_bytes = get_sysctl_int64("hw.memsize").unwrap_or(0);
        let total_mb = total_bytes / (1024 * 1024);

        let mut count = u32::try_from(std::mem::size_of::<libc::vm_statistics64>() / std::mem::size_of::<libc::integer_t>()).unwrap_or(0);
        let mut vm_stat: libc::vm_statistics64 = unsafe { std::mem::zeroed() };
        let host_port = unsafe { mach_host_self() };
        let ret = unsafe {
            libc::host_statistics64(
                host_port,
                libc::HOST_VM_INFO64,
                (&raw mut vm_stat).cast(),
                &raw mut count,
            )
        };

        if ret == libc::KERN_SUCCESS && total_mb > 0 {
            let page_size = (unsafe { libc::sysconf(libc::_SC_PAGESIZE) }).cast_unsigned();
            let active = u64::from(vm_stat.active_count) * page_size;
            let wire = u64::from(vm_stat.wire_count) * page_size;
            let compressor = u64::from(vm_stat.compressor_page_count) * page_size;
            let used_bytes = active + wire + compressor;
            let used_mb = used_bytes / (1024 * 1024);
            format!("{used_mb}MB / {total_mb}MB")
        } else if total_mb > 0 {
            format!("?MB / {total_mb}MB")
        } else {
            "Unknown".to_string()
        }
    }
    #[cfg(target_os = "freebsd")]
    {
        let total_bytes = get_sysctl_int64("hw.physmem").unwrap_or(0);
        let total_mb = total_bytes / (1024 * 1024);

        let page_size = get_sysctl_int("hw.pagesize").map_or(4096u64, |v| v.cast_unsigned());
        let page_count = get_sysctl_int("vm.stats.vm.v_page_count").map_or(0u64, |v| v.cast_unsigned());
        let free_count = get_sysctl_int("vm.stats.vm.v_free_count").map_or(0u64, |v| v.cast_unsigned());
        let inactive_count = get_sysctl_int("vm.stats.vm.v_inactive_count").map_or(0u64, |v| v.cast_unsigned());

        if total_mb > 0 && page_count > 0 {
            let used_pages = page_count.saturating_sub(free_count + inactive_count);
            let used_mb = (used_pages * page_size) / (1024 * 1024);
            format!("{used_mb}MB / {total_mb}MB")
        } else if total_mb > 0 {
            format!("?MB / {total_mb}MB")
        } else {
            "Unknown".to_string()
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        "Not Supported".to_string()
    }
}

#[allow(clippy::cast_lossless)]
pub fn get_drive() -> String {
    let path = CString::new("/").unwrap();
    let mut stat: Statvfs = unsafe { zeroed() };

    let result = unsafe { statvfs(path.as_ptr(), &raw mut stat) };
    if result != 0 {
        return "Failed / Not Supported".to_string();
    }

    let total_space = stat.f_blocks as u64 * stat.f_frsize as u64;
    let free_space = stat.f_bfree as u64 * stat.f_frsize as u64;
    let used_space = total_space - free_space;

    format!("{}GB / {}GB", used_space / 1_073_741_824, total_space / 1_073_741_824)
}

#[allow(clippy::too_many_lines)]
pub fn get_battery() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
            let mut batteries = Vec::new();
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("BAT") || name_str.starts_with("battery") {
                    let path = entry.path();
                    let cap_str = std::fs::read_to_string(path.join("capacity")).unwrap_or_default();
                    let cap_trimmed = cap_str.trim();

                    if !cap_trimmed.is_empty() {
                        let status_str = std::fs::read_to_string(path.join("status")).unwrap_or_default();
                        let status_trimmed = status_str.trim();
                        if status_trimmed.is_empty() {
                            batteries.push(format!("{cap_trimmed}%"));
                        } else {
                            batteries.push(format!("{cap_trimmed}% [{status_trimmed}]"));
                        }
                    }
                }
            }
            if !batteries.is_empty() {
                return batteries.join(", ");
            }
        }
        "None".to_string()
    }
    #[cfg(target_os = "freebsd")]
    {
        if let Some(life) = get_sysctl_int("hw.acpi.battery.life") {
            let state_str = match get_sysctl_int("hw.acpi.battery.state") {
                Some(0) => " [Full]",
                Some(1) => " [Discharging]",
                Some(2) => " [Charging]",
                _ => "",
            };
            return format!("{life}%{state_str}");
        }
        "None".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        #[link(name = "IOKit", kind = "framework")]
        unsafe extern "C" {
            fn IOPSCopyPowerSourcesInfo() -> *const std::ffi::c_void;
            fn IOPSCopyPowerSourcesList(blob: *const std::ffi::c_void) -> *const std::ffi::c_void;
            fn IOPSGetPowerSourceDescription(blob: *const std::ffi::c_void, ps: *const std::ffi::c_void) -> *const std::ffi::c_void;
            fn CFArrayGetCount(array: *const std::ffi::c_void) -> isize;
            fn CFArrayGetValueAtIndex(array: *const std::ffi::c_void, idx: isize) -> *const std::ffi::c_void;
            fn CFDictionaryGetValue(dict: *const std::ffi::c_void, key: *const std::ffi::c_void) -> *const std::ffi::c_void;
            fn CFStringCreateWithCString(alloc: *const std::ffi::c_void, c_str: *const std::ffi::c_char, encoding: u32) -> *const std::ffi::c_void;
            fn CFNumberGetValue(number: *const std::ffi::c_void, the_type: isize, value_ptr: *mut std::ffi::c_void) -> bool;
            fn CFBooleanGetValue(boolean: *const std::ffi::c_void) -> bool;
            fn CFRelease(cf: *const std::ffi::c_void);
        }

        unsafe {
            let blob = IOPSCopyPowerSourcesInfo();
            if blob.is_null() {
                return "None".to_string();
            }
            let list = IOPSCopyPowerSourcesList(blob);
            if list.is_null() {
                CFRelease(blob);
                return "None".to_string();
            }

            let count = CFArrayGetCount(list);
            let mut result = None;

            let k_cap = CFStringCreateWithCString(std::ptr::null(), c"Current Capacity".as_ptr(), 0x0800_0100);
            let k_charging = CFStringCreateWithCString(std::ptr::null(), c"Is Charging".as_ptr(), 0x0800_0100);
            let k_charged = CFStringCreateWithCString(std::ptr::null(), c"Is Charged".as_ptr(), 0x0800_0100);

            for i in 0..count {
                let ps = CFArrayGetValueAtIndex(list, i);
                let desc = IOPSGetPowerSourceDescription(blob, ps);
                if !desc.is_null() {
                    let cap_ref = CFDictionaryGetValue(desc, k_cap);
                    if !cap_ref.is_null() {
                        let mut cap_val: i32 = 0;
                        if CFNumberGetValue(cap_ref, 3, (&raw mut cap_val).cast()) {
                            let is_charging = {
                                let charging_ref = CFDictionaryGetValue(desc, k_charging);
                                !charging_ref.is_null() && CFBooleanGetValue(charging_ref)
                            };
                            let is_charged = {
                                let charged_ref = CFDictionaryGetValue(desc, k_charged);
                                !charged_ref.is_null() && CFBooleanGetValue(charged_ref)
                            };

                            let state_str = if is_charged {
                                " [Full]"
                            } else if is_charging {
                                " [Charging]"
                            } else {
                                " [Discharging]"
                            };

                            result = Some(format!("{cap_val}%{state_str}"));
                            break;
                        }
                    }
                }
            }

            if !k_cap.is_null() { CFRelease(k_cap); }
            if !k_charging.is_null() { CFRelease(k_charging); }
            if !k_charged.is_null() { CFRelease(k_charged); }
            CFRelease(list);
            CFRelease(blob);

            result.unwrap_or_else(|| "None".to_string())
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "macos")))]
    {
        "None".to_string()
    }
}

#[cfg(target_os = "macos")]
pub fn get_screen_res() -> String {
    use core_graphics::display::CGDisplay;
    use core_graphics::display::{CGDisplayPixelsWide, CGDisplayPixelsHigh};
    
    let screens = CGDisplay::active_displays().unwrap_or_default()
        .iter()
        .map(|&id| {
            unsafe {
                format!("{}x{}", CGDisplayPixelsWide(id), CGDisplayPixelsHigh(id))
            }
        })
        .collect::<Vec<_>>();

    if screens.is_empty() {
        "None".to_string()
    } else { screens.join(" ") }
}

#[cfg(not(target_os = "macos"))]
pub fn get_screen_res() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            let mut res_list = Vec::new();
            for entry in entries.flatten() {
                let modes_path = entry.path().join("modes");
                let content = std::fs::read_to_string(modes_path).unwrap_or_default();
                if let Some(first_mode) = content.lines().next() {
                    let trimmed = first_mode.trim();
                    if !trimmed.is_empty() && !res_list.contains(&trimmed.to_string()) {
                        res_list.push(trimmed.to_string());
                    }
                }
            }
            if !res_list.is_empty() {
                return res_list.join(" ");
            }
        }
        "None".to_string()
    }
    #[cfg(target_os = "freebsd")]
    {
        get_sysctl_string("kern.vt.fb.default_mode").unwrap_or_else(|| "None".to_string())
    }
    #[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
    {
        "None".to_string()
    }
}