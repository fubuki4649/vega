use crate::stats::common::get_system_info;
use logo::get_logo_by_name;
use std::io::{stdout, BufWriter, Write};

mod stats;
mod logo;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut custom_logo = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--logo" | "-l" => {
                custom_logo = args.next();
            }
            "--list-logos" => {
                println!("Available logos (36 total):\n  almalinux, alpine, android, apple, arch, archcraft, arcolinux, artix,\n  cachyos, centos, debian, elementary, endeavour, fedora, freebsd,\n  garuda, gentoo, kali, manjaro, mint, netbsd, nixos, nobara, openbsd,\n  opensuse, popos, rhel, rocky, rpi, slackware, solus, steamos, tux,\n  ubuntu, void, zorin");
                return;
            }
            "--help" | "-h" => {
                println!("vega - really fast command line system information tool\n\nUsage: vega [OPTIONS]\n\nOptions:\n  -l, --logo <NAME>     Specify a custom logo to display\n      --list-logos      List all available logos\n  -h, --help            Print help information\n  -v, --version         Print version information");
                return;
            }
            "--version" | "-v" => {
                println!("vega {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {}
        }
    }

    let mut logo = get_logo_by_name(custom_logo.as_deref());
    let system_info = get_system_info();
    let mut out = BufWriter::with_capacity(8192, stdout().lock());
    let pad_width = logo.cols as usize;
    
    for info in system_info {
        if let Some(content) = logo.content.next() {
            let _ = write!(out, "{content}");
        } else { 
            let _ = write!(out, "{:pad_width$}", "");
        }
        
        let _ = writeln!(out, "   {info}");
    }
    
    for content in logo.content.by_ref() {
        let _ = writeln!(out, "{content}");
    }
    
    let _ = writeln!(out);
    let _ = out.flush();
}
