use crate::data::hardware::hardware::get_hardware_info;
use crate::data::title::get_title;

mod data;
mod _utils;

fn main() {
    println!("Hello, world!");
    get_title().iter().for_each(|x| println!("{}", x));
    get_hardware_info().iter().for_each(|x| println!("{}", x));
}
