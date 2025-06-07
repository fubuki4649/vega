use crate::_utils::run_command::ShellReturn;
use crate::sh;
use std::collections::HashMap;
use std::process::Command;
use std::str::Lines;
use std::sync::LazyLock;

static LOGOS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("arch", include_str!("../../static/logos/sh/arch")),
    ])
});

pub struct Logo {
    pub rows: u16,
    pub cols: u16,
    pub content: Lines<'static>,
}

pub fn get_logo() -> Logo {

    let os_distro: &str = match sh!("uname").stdout.trim() {
        "Linux" => {
            &sh!("awk -F= '/^ID=/ {{ gsub(/\"/, \"\", $2); print $2 }}' /etc/os-release").stdout.trim().to_owned()
        },
        "Darwin" => "macos",
        "FreeBSD" => "freebsd",
        _ => "unknown",
    };

    let mut content = match os_distro {
        "arch" => include_str!("../../static/logos/sh/arch"),
        "debian" => include_str!("../../static/logos/sh/debian"),
        "ubuntu" => include_str!("../../static/logos/sh/ubuntu"),
        "fedora" => include_str!("../../static/logos/sh/fedora"),
        "freebsd" => include_str!("../../static/logos/sh/freebsd"),
        "macos" => include_str!("../../static/logos/sh/apple"),
        "pop" => include_str!("../../static/logos/sh/popos"),
        "raspbian" => include_str!("../../static/logos/sh/rpi"),
        _ => "",
    }.lines();

    let first_line = content.next().unwrap();
    let mut logo_metadata = first_line.split_whitespace();
    let rows = logo_metadata.next().unwrap().parse::<u16>().unwrap();
    let cols = logo_metadata.next().unwrap().parse::<u16>().unwrap();
    
    Logo {
        rows,
        cols,
        content,   
    }

}