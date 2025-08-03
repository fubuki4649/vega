use std::{cmp::Ordering, net::IpAddr, process::Command};

use sysinfo::{IpNetwork, NetworkData, Networks, System};

use crate::{
    sh,
    utils::{run_command::ShellReturn, sort_by_priority::SortByPriority, which},
};

/// Retrieves the operating system name and version.
pub fn get_os() -> String {
    let linux_os_ver: ShellReturn =
        sh!("awk -F= '/^PRETTY_NAME=/ {{ gsub(/\"/, \"\", $2); print $2 }}' /etc/os-release");

    if linux_os_ver.err_code == 0 {
        linux_os_ver.stdout.trim().to_string()
    } else {
        System::long_os_version().unwrap_or("Unknown OS".to_string())
    }
}

/// Retrieves the kernel version and release.
pub fn get_kernel() -> String {
    sh!("uname -sr").stdout.trim().to_string()
}

/// Retrieves the system uptime in a human-readable format.
pub fn get_uptime() -> String {
    let uptime: u64 = System::uptime();
    let days: u64 = uptime / 86400;
    let hours: u64 = (uptime % 86400) / 3600;
    let minutes: u64 = (uptime % 3600) / 60;
    let seconds: u64 = uptime % 60;

    let mut parts: Vec<String> = vec![];

    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 || !parts.is_empty() {
        parts.push(format!(
            "{} hour{}",
            hours,
            if hours == 1 { "" } else { "s" }
        ));
    }
    if minutes > 0 || !parts.is_empty() {
        parts.push(format!(
            "{} minute{}",
            minutes,
            if minutes == 1 { "" } else { "s" }
        ));
    }
    if (seconds > 0 || !parts.is_empty()) && days == 0 {
        parts.push(format!(
            "{} second{}",
            seconds,
            if seconds == 1 { "" } else { "s" }
        ));
    }

    parts.join(", ")
}

/// Retrieves the list of installed packages on the system.
pub fn get_packages() -> String {
    let script: &'static str = include_str!("../../../static/sh/packages.sh");
    let mac_script: &'static str = include_str!("../../../static/sh/packages_macos.sh");

    if sh!("uname").stdout.trim() == "Darwin" {
        sh!("{}", mac_script).stdout.trim().to_string()
    } else {
        sh!("{}", script).stdout.trim().to_string()
    }
}

/// Retrieves the window manager name or desktop environment.
pub fn get_window_manager() -> String {
    // macOS Hardcode
    if sh!("uname").stdout.trim() == "Darwin" {
        const SUPPORTED_WMS: [&str; 2] = ["yabai", "Amethyst"];

        for wm in SUPPORTED_WMS {
            if sh!("pgrep -x {}", wm).err_code == 0 {
                return wm.to_string();
            }
        }

        return "aqua".to_string();
    }

    // Read $XDG_CURRENT_DESKTOP for Wayland and X11
    let desktop: ShellReturn =
        sh!(": \"${{XDG_CURRENT_DESKTOP:?}}\" && echo \"$XDG_CURRENT_DESKTOP\"");
    if desktop.err_code == 0 && desktop.stdout.trim() != "" {
        return desktop.stdout.trim().to_string();
    }

    // Fallback PID method for Wayland only
    let wmpid: ShellReturn = if let Some(_) = which::which("fuser") {
        let pid_raw: ShellReturn =
            sh!("fuser \"${{XDG_RUNTIME_DIR}}/${{WAYLAND_DISPLAY:-wayland-0}}\"");
        if pid_raw.err_code == 0 {
            sh!("echo {} | awk '{{print $1}}'", pid_raw.stdout.trim())
        } else {
            pid_raw
        }
    } else if let Some(_) = which::which("lsof") {
        sh!("lsof -t \"${{XDG_RUNTIME_DIR}}/${{WAYLAND_DISPLAY:-wayland-0}}\" 2>&1")
    } else {
        ShellReturn {
            stdout: "".to_string(),
            stderr: "".to_string(),
            err_code: 1,
        }
    };

    if wmpid.err_code == 0 {
        return sh!("ps -p {} -o comm=", wmpid.stdout.trim())
            .stdout
            .trim()
            .to_string();
    }

    "None/Unknown".to_string()
}

/// Retrieves the terminal emulator name.
pub fn get_terminal() -> String {
    let mut pid: i32 = unsafe { libc::getppid() };
    let mut pname: String = sh!("ps -p {} -o comm=", pid).stdout.trim().to_string();

    while pname.ends_with("sh") {
        pid = sh!("ps -p {} -o ppid=", pid)
            .stdout
            .trim()
            .parse::<i32>()
            .unwrap_or(1);
        pname = sh!("ps -p {} -o comm=", pid).stdout.trim().to_string();
    }

    pname
}

/// Retrieves the shell name used by the current process.
pub fn get_shell() -> String {
    let ppid: i32 = unsafe { libc::getppid() };
    sh!("ps -p {} -o comm=", ppid).stdout.trim().to_string()
}

const PRIORITY_ETHERNET: u32 = 1;
const PRIORITY_WIFI: u32 = 2;
const PRIORITY_WWAN: u32 = 3;
const PRIORITY_VPN_INTERFACES: u32 = 4;
const PRIORITY_NETWORK_MANAGER: u32 = 5;
const PRIORITY_TAILSCALE: u32 = 6;
const PRIORITY_LOOPBACK: u32 = 7;
const PRIORITY_DEFAULT: u32 = u32::MAX;

/// Retrieves the IP address of the system, prioritizing physical interfaces.
pub fn get_ip_addr() -> String {
    // Extract IP address from `NetworkData` (prioritizing IPv4 over IPv6)
    let extract_ip: fn(&NetworkData) -> Option<String> = |network: &NetworkData| {
        let mut addrs: Vec<IpAddr> = network
            .ip_networks()
            .iter()
            .map(|ip: &IpNetwork| ip.addr)
            .collect();

        addrs.sort_by(|a: &IpAddr, b: &IpAddr| {
            if a.is_ipv4() && b.is_ipv6() {
                Ordering::Less
            } else if b.is_ipv6() && a.is_ipv4() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        if addrs.len() == 0 {
            None
        } else {
            Some(addrs[0].to_string())
        }
    };

    // Get a list of network interfaces and sort them
    let networks: Networks = Networks::new_with_refreshed_list();
    let mut networks_sorted: Vec<(&String, &NetworkData)> = networks.into_iter().collect();

    // Sort the interfaces by priority
    networks_sorted.sort_by_priority(|network: &(&String, &NetworkData)| {
        let nw_name = network.0.to_lowercase();
        
        match nw_name.as_str() {
            name if name.starts_with("en") => PRIORITY_ETHERNET,
            name if name.starts_with("wl") => PRIORITY_WIFI,
            name if name.starts_with("wwan") => PRIORITY_WWAN,
            name if name.starts_with("tailscale") => PRIORITY_TAILSCALE,
            name if name.starts_with("tun") | name.starts_with("tap") 
                 | name.starts_with("wg") | name.starts_with("vpn") => PRIORITY_VPN_INTERFACES,
            name if name.starts_with("nm") => PRIORITY_NETWORK_MANAGER,
            "lo" => PRIORITY_LOOPBACK,
            _ => PRIORITY_DEFAULT,
        }
    });

    // Return the first non-loopback interface with an IP address
    for network in networks_sorted {
        if let Some(ip) = extract_ip(network.1) {
            return ip;
        }
    }

    "No Connection".to_string()
}
