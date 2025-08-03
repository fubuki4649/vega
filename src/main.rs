#![feature(impl_trait_in_bindings)]

use crate::{
    data::common::get_system_info,
    logo::logo::{Logo, get_logo},
    utils::cli,
};

mod data;
mod logo;
mod utils;

/// Main function to run the Vega system information tool.
fn main() {
    if cli::handle_clap() {
        return;
    }
    // Default: print logo and system info
    let mut logo: Logo = get_logo();
    let system_info: impl Iterator<Item = String> = get_system_info();
    system_info.for_each(|info: String| {
        let content: Option<&'static str> = logo.content.next();
        if let Some(content) = content {
            print!("{}", content);
        } else {
            print!("{}", " ".repeat(logo.cols as usize));
        }
        println!("   {}", info)
    });
    // Finish logo
    loop {
        let content: Option<&'static str> = logo.content.next();
        if let Some(content) = content {
            print!("{}", content);
        } else {
            break;
        }
    }
    println!();
    cli::print_colors();
    println!();
}
