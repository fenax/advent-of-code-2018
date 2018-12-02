extern crate advent_of_code_2018;
use std::fs::read_to_string;
use advent_of_code_2018::*;
fn main() {
    println!("Let's save christmas");
     
    let input_day1 = 
read_to_string("day1-1.input").unwrap().split('\n').map(str::trim)
                                                   .filter_map(|s| s.parse().ok()) 
.collect();



println!("Day 1");
println!(" {}",day1_chronal_calibration::process(&input_day1));
println!(" {}",day1_chronal_calibration::process2hashset(&input_day1));

let input_day2: String  = read_to_string("day2.input").unwrap();
let input_day2 = input_day2.split('\n').collect();

println!("Day 2");
println!(" {}", day2_inventory_management_system::process(&input_day2));
println!(" {}", day2_inventory_management_system::process2(&input_day2))
}
