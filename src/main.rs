extern crate advent_of_code_2018;
use std::fs::read_to_string;
use advent_of_code_2018::*;
fn main() {
    println!("Let's save christmas");
     
    let input_day1 = read_to_string("day1-1.input").unwrap().split('\n').map(|x| 
x.trim())
                                                   .filter_map(|s| 
s.parse().ok()).collect();
println!("{}",day1_chronal_calibration::process(&input_day1));
println!("{}",day1_chronal_calibration::process2(&input_day1));



}
