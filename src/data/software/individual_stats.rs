use crate::_utils::run_command::ShellReturn;
use crate::_utils::sort_by_priority::SortByPriority;
use crate::sh;
use std::cmp::Ordering;
use std::net::IpAddr;
use std::process::Command;
use sysinfo::{NetworkData, Networks, System};
use crate::_utils::which;

pub fn get_os() -> String {
    let linux_os_ver = sh!("awk -F= '/^PRETTY_NAME=/ {{ gsub(/\"/, \"\", $2); print $2 }}' /etc/os-release");
    
    if linux_os_ver.err_code == 0 {
        linux_os_ver.stdout.trim().to_string()
    } else {
        System::long_os_version().unwrap_or("Unknown OS".to_string())
    }
}

pub fn get_kernel() -> String {
    sh!("uname -sr").stdout.trim().to_string()
}

pub fn get_uptime() -> String {
    let uptime = System::uptime();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    let mut parts = vec![];

    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 || !parts.is_empty() {
        parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
    }
    if minutes > 0 || !parts.is_empty() {
        parts.push(format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" }));
    }
    if (seconds > 0 || !parts.is_empty()) && days == 0 {
        parts.push(format!("{} second{}", seconds, if seconds == 1 { "" } else { "s" }));
    }

    parts.join(", ")
}

pub fn get_packages() -> String {
    let script = include_str!("../../../static/sh/packages.sh");
    let mac_script = include_str!("../../../static/sh/packages_macos.sh");
    
    if sh!("uname").stdout.trim() == "Darwin" {
        sh!("{}", mac_script).stdout.trim().to_string()
    } else {
        sh!("{}", script).stdout.trim().to_string()
    }
}

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
    let desktop = sh!(": \"${{XDG_CURRENT_DESKTOP:?}}\" && echo \"$XDG_CURRENT_DESKTOP\"");
    if desktop.err_code == 0 && desktop.stdout.trim() != "" {
        return desktop.stdout.trim().to_string();
    }

    // Fallback PID method for Wayland only
    let wmpid: ShellReturn = if let Some(_) = which::which("fuser") {
        let pid_raw = sh!("fuser \"${{XDG_RUNTIME_DIR}}/${{WAYLAND_DISPLAY:-wayland-0}}\"");
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
        return sh!("ps -p {} -o comm=", wmpid.stdout.trim()).stdout.trim().to_string();
    }

    "None/Unknown".to_string()
}

pub fn get_terminal() ->  String {
    let mut pid = unsafe { libc::getppid() };
    let mut pname = sh!("ps -p {} -o comm=", pid).stdout.trim().to_string();
    
    while pname.ends_with("sh") {
        pid = sh!("ps -p {} -o ppid=", pid).stdout.trim().parse::<i32>().unwrap_or(1);
        pname = sh!("ps -p {} -o comm=", pid).stdout.trim().to_string();
    }
    
    pname
}

pub fn get_shell() -> String {
    let ppid = unsafe { libc::getppid() };
    sh!("ps -p {} -o comm=", ppid).stdout.trim().to_string()
}

pub fn get_ip_addr() -> String {
    // Extract IP address from `NetworkData` (prioritizing IPv4 over IPv6)
    let extract_ip: fn(&NetworkData) -> Option<String> = |network: &NetworkData| {
        let mut addrs: Vec<IpAddr> = network.ip_networks().iter()
            .map(|ip| ip.addr).collect();

        addrs.sort_by(|a, b|
            if a.is_ipv4() && b.is_ipv6() { Ordering::Less }
            else if b.is_ipv6() && a.is_ipv4() { Ordering::Greater }
            else { Ordering::Equal }
        );

        if addrs.len() == 0 { None } else { Some(addrs[0].to_string()) }
    };

    // Get a list of network interfaces and sort them
    let networks = Networks::new_with_refreshed_list();
    let mut networks_sorted: Vec<(&String, &NetworkData)> = networks.into_iter().collect();

    // Sort the interfaces by priority
    networks_sorted.sort_by_priority(|network| {
        let nw_name = network.0.to_lowercase();

        // Prioritize physical interfaces: Ethernet, Wifi, WWAN
        if nw_name.starts_with("en") { 0 }
        else if nw_name.starts_with("wl") { 1 }
        else if nw_name.starts_with("wwan") { 2 }
        // Deprioritize VPN interfaces
        else if nw_name.starts_with("tailscale") { u32::MAX - 1 }
        else if nw_name.starts_with("tun") { 1000 }
        else if nw_name.starts_with("tap") { 1000 }
        else if nw_name.starts_with("wg") { 1000 }
        else if nw_name.starts_with("vpn") { 1000 }
        // Also deprioritize NetworkManager stuff a bit more
        else if nw_name.starts_with("nm") { 1001 }
        // Make sure loopback is last
        else if nw_name == "lo" { u32::MAX }
        // Default priority for other interfaces (brX, hostX, etc.)
        else { 69 }
    });

    // Return the first non-loopback interface with an IP address
    for network in networks_sorted {
        if let Some(ip) = extract_ip(network.1) {
            return ip;
        }
    }
    
    "No Connection".to_string()
}