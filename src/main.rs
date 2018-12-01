extern crate advent_of_code_2018;
use std::fs::read_to_string;
use advent_of_code_2018::*;
fn main() {
    println!("Let's save christmas");
    
println!("{}",day1_chronal_calibration::process(&read_to_string("day1-1.input").unwrap()));
}
