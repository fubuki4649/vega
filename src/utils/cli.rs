use clap::{Parser, Subcommand};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use crate::data::{hardware::individual_stats as hw_stats, software::individual_stats as sw_stats};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simpler and faster CLI system information tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Displays device model and exits
    Model,
    /// Displays cpu and exits
    Cpu,
    /// Displays the first gpu detected and exits
    Gpu,
    /// Displays memory info and exits
    Ram,
    /// Displays free space / total space of the root partition
    Disk,
    /// Displays the screen resolution of the first monitor
    Res,
    /// Displays the name of the operating system
    Os,
    /// Displays the kernel version
    Kernel,
    /// Displays the system uptime
    Uptime,
    /// Displays the number of packages
    Pkg,
    /// Displays the current window manager
    Wm,
    /// Displays the current terminal
    Term,
    /// Displays the current shell
    Shell,
    /// Displays the current device's local ip address
    Ip,
    /// Displays a color test
    Colors,
}

/// Handles the command line arguments and executes the corresponding command.
pub fn handle_clap() -> bool {
    let cli: Cli = Cli::parse();
    match &cli.command {
        Some(Commands::Model) => {
            println!("{}", hw_stats::get_model());
            true
        }
        Some(Commands::Cpu) => {
            println!("{}", hw_stats::get_cpu());
            true
        }
        Some(Commands::Gpu) => {
            println!("{}", hw_stats::get_gpu());
            true
        }
        Some(Commands::Ram) => {
            let mut sys: System = System::new_with_specifics(
                RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
            );
            println!("{}", hw_stats::get_ram(&mut sys));
            true
        }
        Some(Commands::Disk) => {
            println!("{}", hw_stats::get_drive());
            true
        }
        Some(Commands::Res) => {
            println!("{}", hw_stats::get_screen_res());
            true
        }
        Some(Commands::Os) => {
            println!("{}", sw_stats::get_os());
            true
        }
        Some(Commands::Kernel) => {
            println!("{}", sw_stats::get_kernel());
            true
        }
        Some(Commands::Uptime) => {
            println!("{}", sw_stats::get_uptime());
            true
        }
        Some(Commands::Pkg) => {
            println!("{}", sw_stats::get_packages());
            true
        }
        Some(Commands::Wm) => {
            println!("{}", sw_stats::get_window_manager());
            true
        }
        Some(Commands::Term) => {
            println!("{}", sw_stats::get_terminal());
            true
        }
        Some(Commands::Shell) => {
            println!("{}", sw_stats::get_shell());
            true
        }
        Some(Commands::Ip) => {
            println!("{}", sw_stats::get_ip_addr());
            true
        }
        Some(Commands::Colors) => {
            print_colors();
            true
        }
        None => false,
    }
}

/// Prints a color test to the terminal.
pub fn print_colors() {
    for i in 0..8 {
        print!("\x1b[1;4{}m   \x1b[0m", i);
    }
    for i in 0..8 {
        print!("\x1b[10{}m   \x1b[0m", i);
    }
    println!();
}
