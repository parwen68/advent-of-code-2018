#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate rayon;
#[macro_use]
extern crate maplit;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    println!("Day 1, step 1: {:?}", day1::run1());
    println!("Day 1, step 2: {:?}", day1::run2());
    println!("Day 2, step 1: {:?}", day2::run1());
    println!("Day 2, step 2: {:?}", day2::run2());
    println!("Day 3, step 1: {:?}", day3::run1());
    println!("Day 3, step 2: {:?}", day3::run2());
    println!("Day 4, step 1: {:?}", day4::run1());
    println!("Day 4, step 2: {:?}", day4::run2());
    println!("Day 5, step 1: {:?}", day5::run1());
    println!("Day 5, step 2: {:?}", day5::run2());
    println!("Day 6, step 1: {:?}", day6::run1());
    println!("Day 6, step 2: {:?}", day6::run2());
    println!("Day 7, step 1: {:?}", day7::run1());
    println!("Day 7, step 2: {:?}", day7::run2());
}
