#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate rayon;

mod common;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    println!("Day 1, step 1: {:?}", day1::run1());
    println!("Day 1, step 2: {:?}", day1::run2());
    println!("Day 2, step 1: {:?}", day2::run1());
    println!("Day 2, step 2: {:?}", day2::run2());
    println!("Day 3, step 1: {:?}", day3::run1());
    println!("Day 3, step 2: {:?}", day3::run2());
    println!("Day 4, step 1: {:?}", day4::run1());
    println!("Day 4, step 2: {:?}", day4::run2());
}
