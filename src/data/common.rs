use crate::data::hardware::hardware::get_hardware_info;
use crate::data::software::software::get_software_info;
use colored::Colorize;

pub fn get_system_info() -> impl Iterator<Item = String> {
    let (mut title, mut hw, mut sw) = std::thread::scope(|s| {
        let title_handle = s.spawn(get_title);
        let hw_handle = s.spawn(get_hardware_info);
        let sw_handle = s.spawn(get_software_info);

        (
            title_handle.join().unwrap_or_default(),
            hw_handle.join().unwrap_or_default(),
            sw_handle.join().unwrap_or_default(),
        )
    });

    let mut lines: Vec<String> = Vec::with_capacity(title.len() + hw.len() + sw.len() + 1);
    lines.append(&mut title);
    lines.append(&mut hw);
    lines.push(String::new());
    lines.append(&mut sw);

    lines.into_iter()
}

fn get_username() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("LOGNAME"))
        .unwrap_or_else(|_| {
            let uid = unsafe { libc::getuid() };
            let pw = unsafe { libc::getpwuid(uid) };
            if pw.is_null() {
                "unknown".to_string()
            } else {
                unsafe { std::ffi::CStr::from_ptr((*pw).pw_name) }
                    .to_string_lossy()
                    .into_owned()
            }
        })
}

fn get_hostname() -> String {
    let mut buf = [0u8; 256];
    let res = unsafe { libc::gethostname(buf.as_mut_ptr().cast(), buf.len()) };
    if res == 0 {
        let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        String::from_utf8_lossy(&buf[..len]).into_owned()
    } else {
        std::fs::read_to_string("/proc/sys/kernel/hostname")
            .map_or_else(|_| "localhost".to_string(), |s| s.trim().to_string())
    }
}

fn get_title() -> Vec<String> {
    let username = get_username();
    let hostname = get_hostname();

    let title = format!("{}@{}", username.blue().bold(), hostname.blue().bold());
    let dashes = "=".repeat(username.len() + hostname.len() + 1);

    vec![title, dashes]
}