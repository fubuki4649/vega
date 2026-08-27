use std::net::IpAddr;

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
fn get_sysctl_string(name: &str) -> Option<String> {
    let cname = std::ffi::CString::new(name).ok()?;
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

pub fn get_os() -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                    return val.trim_matches('"').trim_matches('\'').trim().to_string();
                }
            }
            for line in content.lines() {
                if let Some(val) = line.strip_prefix("NAME=") {
                    return val.trim_matches('"').trim_matches('\'').trim().to_string();
                }
            }
        }
        "Linux".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(ver) = get_sysctl_string("kern.osproductversion") {
            format!("macOS {ver}")
        } else {
            "macOS".to_string()
        }
    }
    #[cfg(target_os = "freebsd")]
    {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                    return val.trim_matches('"').trim_matches('\'').trim().to_string();
                }
            }
        }
        "FreeBSD".to_string()
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        "Unknown OS".to_string()
    }
}

pub fn get_kernel() -> String {
    let mut uts = std::mem::MaybeUninit::<libc::utsname>::uninit();
    if unsafe { libc::uname(uts.as_mut_ptr()) } == 0 {
        let uts = unsafe { uts.assume_init() };
        let sysname = unsafe { std::ffi::CStr::from_ptr(uts.sysname.as_ptr()) }.to_string_lossy();
        let release = unsafe { std::ffi::CStr::from_ptr(uts.release.as_ptr()) }.to_string_lossy();
        format!("{sysname} {release}")
    } else {
        "Unknown".to_string()
    }
}

pub fn get_uptime() -> String {
    #[cfg(target_os = "linux")]
    let uptime = {
        if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
            if let Some(first) = content.split_whitespace().next() {
                first.split('.').next().and_then(|s| s.parse::<u64>().ok()).unwrap_or(0)
            } else { 0 }
        } else {
            let mut info: libc::sysinfo = unsafe { std::mem::zeroed() };
            if unsafe { libc::sysinfo(&raw mut info) } == 0 {
                info.uptime.cast_unsigned()
            } else { 0 }
        }
    };

    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    let uptime = {
        let mut boottime = libc::timeval { tv_sec: 0, tv_usec: 0 };
        let mut size = std::mem::size_of::<libc::timeval>();
        let cname = std::ffi::CString::new("kern.boottime").unwrap();
        let ret = unsafe {
            libc::sysctlbyname(cname.as_ptr(), (&raw mut boottime).cast(), &raw mut size, std::ptr::null_mut(), 0)
        };
        if ret == 0 {
            let now = unsafe { libc::time(std::ptr::null_mut()) };
            (now - boottime.tv_sec).max(0).cast_unsigned()
        } else { 0 }
    };

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    let uptime = 0u64;

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

#[allow(clippy::too_many_lines)]
pub fn get_packages() -> String {
    let mut counts: Vec<String> = Vec::new();

    // Pacman (Arch / CachyOS / Manjaro / Endeavour / Artix / Garuda)
    if let Ok(entries) = std::fs::read_dir("/var/lib/pacman/local") {
        let mut count = 0;
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|ft| ft.is_dir()) && entry.file_name() != "ALPM_DB_VERSION" {
                count += 1;
            }
        }
        if count > 0 {
            counts.push(format!("{count} (pacman)"));
        }
    }

    // DPKG (Debian / Ubuntu / Mint / Pop / elementary / Kali / Zorin)
    if let Ok(content) = std::fs::read_to_string("/var/lib/dpkg/status") {
        let count = content.lines().filter(|l| l.starts_with("Package: ")).count();
        if count > 0 {
            counts.push(format!("{count} (dpkg)"));
        }
    }

    // APK (Alpine)
    if let Ok(content) = std::fs::read_to_string("/lib/apk/db/installed") {
        let count = content.lines().filter(|l| l.starts_with("P:")).count();
        if count > 0 {
            counts.push(format!("{count} (apk)"));
        }
    }

    // XBPS (Void Linux)
    if let Ok(entries) = std::fs::read_dir("/var/db/xbps") {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("pkgdb-") && name_str.ends_with(".plist") {
                if let Ok(content) = std::fs::read_to_string(entry.path()) {
                    let count = content.matches("<key>pkgver</key>").count();
                    if count > 0 {
                        counts.push(format!("{count} (xbps)"));
                    }
                }
                break;
            }
        }
    }

    // EOPKG (Solus)
    if let Ok(entries) = std::fs::read_dir("/var/lib/eopkg/package") {
        let count = entries.filter_map(Result::ok).filter(|e| e.path().is_dir()).count();
        if count > 0 {
            counts.push(format!("{count} (eopkg)"));
        }
    }

    // Portage / Gentoo (/var/db/pkg/*/*)
    if let Ok(categories) = std::fs::read_dir("/var/db/pkg") {
        let mut count = 0;
        for cat in categories.flatten() {
            if let Ok(pkgs) = std::fs::read_dir(cat.path()) {
                count += pkgs.filter_map(Result::ok).filter(|e| e.path().is_dir()).count();
            }
        }
        if count > 0 {
            counts.push(format!("{count} (emerge)"));
        }
    }

    // Flatpak
    let mut flatpak_count = 0;
    if let Ok(entries) = std::fs::read_dir("/var/lib/flatpak/app") {
        flatpak_count += entries.filter_map(Result::ok).filter(|e| e.path().is_dir()).count();
    }
    if let Some(home) = std::env::var_os("HOME") {
        let user_flatpak = std::path::Path::new(&home).join(".local/share/flatpak/app");
        if let Ok(entries) = std::fs::read_dir(user_flatpak) {
            flatpak_count += entries.filter_map(Result::ok).filter(|e| e.path().is_dir()).count();
        }
    }
    if flatpak_count > 0 {
        counts.push(format!("{flatpak_count} (flatpak)"));
    }

    // Snap
    if let Ok(entries) = std::fs::read_dir("/var/lib/snapd/snaps") {
        let count = entries
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().ends_with(".snap"))
            .count();
        if count > 0 {
            counts.push(format!("{count} (snap)"));
        }
    }

    // Homebrew Cellar + Caskroom (macOS / Linux)
    let mut brew_count = 0;
    for prefix in &["/opt/homebrew/Cellar", "/usr/local/Cellar", "/home/linuxbrew/.linuxbrew/Cellar"] {
        if let Ok(entries) = std::fs::read_dir(prefix) {
            brew_count += entries.filter_map(Result::ok).filter(|e| e.path().is_dir()).count();
        }
    }
    for prefix in &["/opt/homebrew/Caskroom", "/usr/local/Caskroom"] {
        if let Ok(entries) = std::fs::read_dir(prefix) {
            brew_count += entries.filter_map(Result::ok).filter(|e| e.path().is_dir()).count();
        }
    }
    if brew_count > 0 {
        counts.push(format!("{brew_count} (brew)"));
    }

    // MacPorts (macOS)
    if let Ok(entries) = std::fs::read_dir("/opt/local/var/macports/software") {
        let count = entries.filter_map(Result::ok).filter(|e| e.path().is_dir()).count();
        if count > 0 {
            counts.push(format!("{count} (macports)"));
        }
    }

    // Nix (user & system profiles)
    let mut nix_count = 0;
    if let Some(home) = std::env::var_os("HOME") {
        let nix_profile = std::path::Path::new(&home).join(".nix-profile/manifest.json");
        let content = std::fs::read_to_string(nix_profile).unwrap_or_default();
        nix_count += content.matches("\"package_attr\"").count();
    }
    if nix_count > 0 {
        counts.push(format!("{nix_count} (nix)"));
    }

    if counts.is_empty() {
        "0".to_string()
    } else {
        counts.join(", ")
    }
}

pub fn get_window_manager() -> String {
    // 1. Check environment variables
    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        let trimmed = desktop.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if let Ok(desktop) = std::env::var("DESKTOP_SESSION") {
        let trimmed = desktop.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return "Hyprland".to_string();
    }
    if std::env::var("SWAYSOCK").is_ok() {
        return "Sway".to_string();
    }

    // 2. Check running processes on Linux via /proc
    #[cfg(target_os = "linux")]
    {
        const KNOWN_WMS: &[&str] = &[
            "Hyprland", "hyprland", "sway", "kwin_wayland", "kwin_x11", "kwin",
            "gnome-shell", "mutter", "wayfire", "river", "niri", "labwc",
            "i3", "bspwm", "dwm", "awesome", "xmonad", "qtile", "xfwm4",
            "openbox", "fluxbox", "enlightenment", "compiz", "marco",
        ];

        if let Ok(entries) = std::fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let is_pid = entry.file_name().to_str().is_some_and(|s| s.chars().all(|c| c.is_ascii_digit()));
                if is_pid {
                    let comm = std::fs::read_to_string(entry.path().join("comm")).unwrap_or_default();
                    let comm_trimmed = comm.trim();
                    for &wm in KNOWN_WMS {
                        if comm_trimmed.eq_ignore_ascii_case(wm) {
                            return wm.to_string();
                        }
                    }
                }
            }
        }
        "None/Unknown".to_string()
    }

    #[cfg(target_os = "macos")]
    {
        "Aqua".to_string()
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        "None/Unknown".to_string()
    }
}

pub fn get_terminal() -> String {
    // 1. Check known environment variables
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        let trimmed = term.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if let Ok(term) = std::env::var("TERMINAL_EMULATOR") {
        let trimmed = term.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if std::env::var("KITTY_PID").is_ok() || std::env::var("KITTY_WINDOW_ID").is_ok() {
        return "kitty".to_string();
    }
    if std::env::var("ALACRITTY_LOG").is_ok() || std::env::var("ALACRITTY_WINDOW_ID").is_ok() {
        return "alacritty".to_string();
    }
    if std::env::var("WT_SESSION").is_ok() {
        return "Windows Terminal".to_string();
    }
    if std::env::var("KONSOLE_VERSION").is_ok() {
        return "konsole".to_string();
    }
    if std::env::var("FOOT_SERVER_SOCKET").is_ok() {
        return "foot".to_string();
    }

    // 2. In-process process-tree traversal on Linux via /proc
    #[cfg(target_os = "linux")]
    {
        let mut curr_pid = unsafe { libc::getppid() };
        let mut depth = 0;
        while curr_pid > 1 && depth < 10 {
            depth += 1;
            let stat_path = format!("/proc/{curr_pid}/stat");
            let stat = std::fs::read_to_string(stat_path).unwrap_or_default();
            let op = stat.find('(');
            let cp = stat.rfind(')');
            match (op, cp) {
                (Some(open_paren), Some(close_paren)) if open_paren < close_paren => {
                    let comm = &stat[open_paren + 1..close_paren];
                    let rest = stat[close_paren + 2..].trim_start();
                    let fields: Vec<&str> = rest.split_whitespace().collect();
                    let parent_pid = fields.get(1).and_then(|p| p.parse::<i32>().ok()).unwrap_or(1);

                    if !comm.ends_with("sh") && comm != "sudo" && comm != "su" && comm != "vega" {
                        return comm.to_string();
                    }
                    curr_pid = parent_pid;
                }
                _ => break,
            }
        }
    }

    std::env::var("TERM").unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_shell() -> String {
    #[cfg(target_os = "linux")]
    {
        let ppid = unsafe { libc::getppid() };
        if let Ok(comm) = std::fs::read_to_string(format!("/proc/{ppid}/comm")) {
            let trimmed = comm.trim();
            if trimmed.ends_with("sh") || trimmed == "fish" || trimmed == "nu" || trimmed == "ion" || trimmed == "elvish" {
                return trimmed.to_string();
            }
        }
    }

    if let Ok(shell_path) = std::env::var("SHELL") {
        let path = std::path::Path::new(&shell_path);
        if let Some(name) = path.file_name() {
            return name.to_string_lossy().into_owned();
        }
    }

    "Unknown".to_string()
}

pub fn get_ip_addr() -> String {
    #[cfg(unix)]
    {
        let mut ifaddrs = std::ptr::null_mut();
        if unsafe { libc::getifaddrs(&raw mut ifaddrs) } == 0 && !ifaddrs.is_null() {
            let mut list: Vec<(String, IpAddr)> = Vec::new();
            let mut curr = ifaddrs;
            while !curr.is_null() {
                let ifa = unsafe { &*curr };
                if !ifa.ifa_addr.is_null() {
                    let family = i32::from(unsafe { (*ifa.ifa_addr).sa_family });
                    let flags = ifa.ifa_flags;
                    let is_up = (flags & (libc::IFF_UP as u32)) != 0;
                    let is_loopback = (flags & (libc::IFF_LOOPBACK as u32)) != 0;

                    if is_up && !is_loopback {
                        let name = unsafe { std::ffi::CStr::from_ptr(ifa.ifa_name) }.to_string_lossy().into_owned();
                        if family == libc::AF_INET {
                            let sa_in = ifa.ifa_addr.cast::<libc::sockaddr_in>();
                            let ip = std::net::Ipv4Addr::from(u32::from_be(unsafe { (*sa_in).sin_addr.s_addr }));
                            list.push((name, IpAddr::V4(ip)));
                        } else if family == libc::AF_INET6 {
                            let sa_in6 = ifa.ifa_addr.cast::<libc::sockaddr_in6>();
                            let ip = std::net::Ipv6Addr::from(unsafe { (*sa_in6).sin6_addr.s6_addr });
                            if !ip.is_loopback() && !ip.is_unicast_link_local() {
                                list.push((name, IpAddr::V6(ip)));
                            }
                        }
                    }
                }
                curr = ifa.ifa_next;
            }
            unsafe { libc::freeifaddrs(ifaddrs) };

            list.sort_by_key(|(name, ip)| {
                let nw_name = name.to_lowercase();
                let proto_rank = match ip {
                    IpAddr::V4(_) => 0,
                    IpAddr::V6(_) => 10,
                };
                let iface_rank = if nw_name.starts_with("en") || nw_name.starts_with("eth") { 0 }
                else if nw_name.starts_with("wl") { 1 }
                else if nw_name.starts_with("wwan") { 2 }
                else if nw_name.starts_with("tailscale") { 2000 }
                else if nw_name.starts_with("tun") || nw_name.starts_with("tap") || nw_name.starts_with("wg") || nw_name.starts_with("vpn") { 1000 }
                else if nw_name.starts_with("nm") { 1001 }
                else { 69 };

                iface_rank + proto_rank
            });

            if let Some((_, ip)) = list.first() {
                return ip.to_string();
            }
        }
    }

    "No Connection".to_string()
}