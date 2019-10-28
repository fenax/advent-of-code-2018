extern crate advent_of_code_2018;
use std::fs::read_to_string;
use advent_of_code_2018::*;




fn main() {
    println!("Let's save christmas");
    day_10();
}

fn day_1(){
    let input_day1 = 
read_to_string("day1-1.input").unwrap().split('\n').map(str::trim)
                                                   .filter_map(|s| s.parse().ok()) 
.collect();



println!("Day 1");
println!(" {}",day1_chronal_calibration::process(&input_day1));
println!(" {}",day1_chronal_calibration::process2hashset(&input_day1));
}
fn day_2(){
let input_day2: String  = read_to_string("day2.input").unwrap();
let input_day2 = input_day2.split('\n').collect();

println!("Day 2");
println!(" {}", day2_inventory_management_system::process(&input_day2));
println!(" {}", day2_inventory_management_system::process2(&input_day2));
}

fn day_3(){
let input_day3: String = read_to_string("day3.input").unwrap();
let input_day3: Vec<Vec<i32>> = input_day3.split('\n').map(
    |s| s.replace("#","").replace(" @ "," ").replace(","," ").replace(": "," ")
    .replace("x"," ").split_whitespace().map(str::trim).filter_map(|s| s.parse().ok()).collect()).filter(|v: &Vec<i32>| v.len()>0).collect();



println!("Day3");
println!(" {}",day3::process(&input_day3));
println!(" {}",day3::process_2(&input_day3));
}
fn day_4(){
let input_day4: String = read_to_string("day4.input").unwrap();
let mut input_day4: Vec<&str> = input_day4.split('\n').filter(|s| s.len()>0).collect();
input_day4.sort();
let table : Vec<[&str;3]> = input_day4.iter().map(|s| [&s[15..17],&s[25..26],&s[26..]]).collect();

println!("Day 4");

println!(" {}",day4::process(&table));
println!(" {}",day4::process2(&table));
}
fn day_5(){
let mut input_day5: String = read_to_string("day5.input").unwrap();
let i = input_day5.trim_end().len();
input_day5.truncate(i);


println!("Day 5");
println!(" {}",day5::process(&input_day5));
//println!(" or {}",day5::other_process(&input_day5));
println!(" {}",day5::process2(&input_day5));
}

fn day_6(){
println!("Day 6");
let mut input: String = read_to_string("day6.input").unwrap();
let i = input.trim_end().len();
input.truncate(i);

println!(" {}", day6::process(&input));
println!(" {}", day6::process2(&input));
}
fn day_7(){
    println!("Day 7");
    let mut input: String = read_to_string("day7.input").unwrap();                                  let i = input.trim_end().len();                 input.truncate(i);
    let mut ivec: Vec<&str> = input.split('\n').filter(|s| s.len()>0).collect();

    let parsed = day7::parse(&ivec);
    println!(" {}", day7::process(&parsed));
    println!(" {}", day7::process2(&parsed));
}

fn day_8(){
    println!("Day 8");
    let mut input: String = read_to_string("day8.input").unwrap();
    let i = input.trim_end().len();
    input.truncate(i);
    let mut ivec: Vec<&str> = input.split(' ').filter(|s|s.len()>0).collect();

    let parsed = day8::parse(&ivec);
    println!(" {}", day8::process(&parsed));
    println!(" {}", day8::process2(&parsed));
}

fn day_9(){
    println!("Day 9");
    println!("{}", day9::process());
}
fn day_10(){

println!("Day 10");
    let mut input: String
        =read_to_string("day10.input").unwrap();
    let i = input.trim_end().len();                 input.truncate(i);
    let mut ivec: Vec<&str> 
        = input.split('\n')
            .filter(|s|s.len()>0).collect();

    let parsed = day10::parse(&ivec);
    println!("{}",day10::process(&parsed));
    println!("{}",day10::process2(&parsed));
}
