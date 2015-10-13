#![allow(dead_code)]

#[macro_use]
extern crate conrod;

mod estimator;
mod gui;




fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // let out: String = get_output(args);
    // println!("{}", out);
    gui::run();
}
