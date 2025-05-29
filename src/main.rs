use crate::data::common::get_system_info;
use crate::logo::logo::get_logo;

mod data;
mod _utils;
mod logo;

fn main() {
    
    let mut logo = get_logo();
    let system_info = get_system_info();
    
    system_info.for_each(|info| {
        // Print logo
        let content = logo.content.next();
        if let Some(content) = content {
            print!("{}", content);
        }
        else { 
            print!("{}", " ".repeat(logo.cols as usize));
        }
        
        // Print data
        println!("   {}", info)
    });
    
    // Finish logo
    loop {
        let content = logo.content.next();
        if let Some(content) = content {
            print!("{}", content);
        } else {
            break
        }
    }
    
    println!("\n")
}
