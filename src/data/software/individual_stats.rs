use crate::_utils::run_command::ShellReturn;
use crate::_utils::sort_by_priority::SortByPriority;
use crate::sh;
use std::cmp::Ordering;
use std::net::IpAddr;
use std::process::Command;
use sysinfo::{NetworkData, Networks, System};

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
    "Coming Soon".to_string()
}

pub fn get_window_manager() -> String {
    "Coming Soon".to_string()
}

pub fn get_terminal() ->  String {
    "Coming Soon".to_string()
}

pub fn get_shell() -> String {
    "Coming Soon".to_string()
}

pub fn get_ip_addr() -> String {

    // Extract IP address from `NetworkData` (prioritizing IPv4 over IPv6)
    let extract_ip: fn(&NetworkData) -> String = |network: &NetworkData| {
        let mut addrs: Vec<IpAddr> = network.ip_networks().iter()
            .map(|ip| ip.addr).collect();

        addrs.sort_by(|a, b|
            if a.is_ipv4() && b.is_ipv6() { Ordering::Less }
            else if b.is_ipv6() && a.is_ipv4() { Ordering::Greater }
            else { Ordering::Equal }
        );

        if addrs.len() == 0 { "No Connection".to_string() } else { addrs[0].to_string() }
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

    // Return the first non-loopback interface's IP address
    if networks_sorted.len() > 0 && networks_sorted[0].0 != "lo" {
        return extract_ip(networks_sorted[0].1);
    }
    // Otherwise, return "No Connection"
    else { return "No Connection".to_string(); }

}