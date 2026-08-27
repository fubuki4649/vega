use std::str::Lines;

pub struct Logo {
    #[allow(dead_code)]
    pub rows: u16,
    pub cols: u16,
    pub content: Lines<'static>,
}

const LOGO_MAP: &[(&[&str], &str)] = &[
    (&["almalinux", "alma"], include_str!("../../static/logos/sh/almalinux")),
    (&["alpine"], include_str!("../../static/logos/sh/alpine")),
    (&["android"], include_str!("../../static/logos/sh/android")),
    (&["apple", "macos", "darwin", "osx"], include_str!("../../static/logos/sh/apple")),
    (&["arch", "archlinux"], include_str!("../../static/logos/sh/arch")),
    (&["archcraft"], include_str!("../../static/logos/sh/archcraft")),
    (&["arcolinux", "arco"], include_str!("../../static/logos/sh/arcolinux")),
    (&["artix"], include_str!("../../static/logos/sh/artix")),
    (&["cachyos", "cachy"], include_str!("../../static/logos/sh/cachyos")),
    (&["centos"], include_str!("../../static/logos/sh/centos")),
    (&["debian"], include_str!("../../static/logos/sh/debian")),
    (&["elementary", "elementaryos"], include_str!("../../static/logos/sh/elementary")),
    (&["endeavouros", "endeavour"], include_str!("../../static/logos/sh/endeavour")),
    (&["fedora"], include_str!("../../static/logos/sh/fedora")),
    (&["freebsd"], include_str!("../../static/logos/sh/freebsd")),
    (&["garuda"], include_str!("../../static/logos/sh/garuda")),
    (&["gentoo"], include_str!("../../static/logos/sh/gentoo")),
    (&["kali", "kalilinux"], include_str!("../../static/logos/sh/kali")),
    (&["manjaro"], include_str!("../../static/logos/sh/manjaro")),
    (&["mint", "linuxmint"], include_str!("../../static/logos/sh/mint")),
    (&["netbsd"], include_str!("../../static/logos/sh/netbsd")),
    (&["nixos"], include_str!("../../static/logos/sh/nixos")),
    (&["nobara"], include_str!("../../static/logos/sh/nobara")),
    (&["openbsd"], include_str!("../../static/logos/sh/openbsd")),
    (&["opensuse", "opensuse-tumbleweed", "opensuse-leap", "suse"], include_str!("../../static/logos/sh/opensuse")),
    (&["pop", "popos", "pop_os"], include_str!("../../static/logos/sh/popos")),
    (&["rhel", "redhat", "redhatenterpriselinux"], include_str!("../../static/logos/sh/rhel")),
    (&["rocky", "rockylinux"], include_str!("../../static/logos/sh/rocky")),
    (&["raspbian", "rpi", "raspberrypi", "raspberrypios"], include_str!("../../static/logos/sh/rpi")),
    (&["slackware"], include_str!("../../static/logos/sh/slackware")),
    (&["solus"], include_str!("../../static/logos/sh/solus")),
    (&["steamos", "holo"], include_str!("../../static/logos/sh/steamos")),
    (&["tux", "linux"], include_str!("../../static/logos/sh/tux")),
    (&["ubuntu"], include_str!("../../static/logos/sh/ubuntu")),
    (&["void", "voidlinux"], include_str!("../../static/logos/sh/void")),
    (&["zorin", "zorinos"], include_str!("../../static/logos/sh/zorin")),
];

fn match_distro(name: &str) -> Option<&'static str> {
    let name_trimmed = name.trim();
    for &(aliases, logo) in LOGO_MAP {
        for &alias in aliases {
            if name_trimmed.eq_ignore_ascii_case(alias) {
                return Some(logo);
            }
        }
    }
    None
}

fn detect_distro() -> (&'static str, Option<String>) {
    #[cfg(target_os = "macos")]
    {
        ("macos", None)
    }
    #[cfg(target_os = "freebsd")]
    {
        ("freebsd", None)
    }
    #[cfg(target_os = "openbsd")]
    {
        ("openbsd", None)
    }
    #[cfg(target_os = "netbsd")]
    {
        ("netbsd", None)
    }

    #[cfg(not(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd")))]
    {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            let mut id = None;
            let mut id_like = None;
            for line in content.lines() {
                if let Some(val) = line.strip_prefix("ID=") {
                    id = Some(val.trim_matches('"').trim_matches('\'').trim().to_lowercase());
                } else if let Some(val) = line.strip_prefix("ID_LIKE=") {
                    id_like = Some(val.trim_matches('"').trim_matches('\'').trim().to_lowercase());
                }
            }
            if let Some(id_str) = id {
                return ("linux", Some(id_str));
            }
            if let Some(like_str) = id_like {
                return ("linux", Some(like_str));
            }
        }

        ("linux", None)
    }
}

#[allow(dead_code)]
pub fn get_logo() -> Logo {
    get_logo_by_name(None)
}

pub fn get_logo_by_name(custom_name: Option<&str>) -> Logo {
    let (os, detected_id) = detect_distro();

    let raw_logo = if let Some(name) = custom_name {
        match_distro(name).unwrap_or_else(|| match_distro("tux").unwrap())
    } else if let Some(ref distro_id) = detected_id {
        if let Some(logo) = match_distro(distro_id) {
            logo
        } else {
            let mut matched = None;
            for part in distro_id.split_whitespace() {
                if let Some(logo) = match_distro(part) {
                    matched = Some(logo);
                    break;
                }
            }
            matched.unwrap_or_else(|| match os {
                "macos" => match_distro("apple").unwrap(),
                "freebsd" => match_distro("freebsd").unwrap(),
                "openbsd" => match_distro("openbsd").unwrap(),
                "netbsd" => match_distro("netbsd").unwrap(),
                _ => match_distro("tux").unwrap(),
            })
        }
    } else {
        match os {
            "macos" => match_distro("apple").unwrap(),
            "freebsd" => match_distro("freebsd").unwrap(),
            "openbsd" => match_distro("openbsd").unwrap(),
            "netbsd" => match_distro("netbsd").unwrap(),
            _ => match_distro("tux").unwrap(),
        }
    };

    let mut content = raw_logo.lines();
    if let Some(first_line) = content.next() {
        let mut logo_metadata = first_line.split_whitespace();
        let rows = logo_metadata.next().and_then(|r| r.parse::<u16>().ok()).unwrap_or(0);
        let cols = logo_metadata.next().and_then(|c| c.parse::<u16>().ok()).unwrap_or(0);

        Logo {
            rows,
            cols,
            content,
        }
    } else {
        Logo {
            rows: 0,
            cols: 0,
            content: "".lines(),
        }
    }
}