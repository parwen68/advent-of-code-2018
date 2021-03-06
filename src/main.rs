#![feature(drain_filter)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate rayon;
#[macro_use]
extern crate maplit;

mod common;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    println!("Day 8, step 1: {:?}", day8::run1());
    println!("Day 8, step 2: {:?}", day8::run2());
    println!("Day 9, step 1: {:?}", day9::run1());
    println!("Day 9, step 2: {:?}", day9::run2());
    println!("Day 10, step 1:\n");
    day10::run1().iter().for_each(|line| println!("{}", line));
    println!("Day 10, step 2: {:?}", day10::run2());
    println!("Day 11, step 1: {:?}", day11::run1());
    println!("Day 11, step 2: {:?}", day11::run2());
    println!("Day 12, step 1: {:?}", day12::run1());
    println!("Day 12, step 2: {:?}", day12::run2());
    println!("Day 13, step 1: {:?}", day13::run1());
    println!("Day 13, step 2: {:?}", day13::run2());
}
