use crate::data::common::get_system_info;

mod data;
mod _utils;

fn main() {
    get_system_info().iter().for_each(|x| println!("{}", x));
}
