use super::common::*;

use std::collections::HashSet;

static INPUT: &str = "./src/day1/input.txt";

pub fn run1() -> i32 {
    let input = read_file(INPUT);

    input.split("\n")
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .sum()
}

pub fn run2() -> Option<i32> {
    let input = read_file(INPUT);

    let result = input.split("\n")
        .map(|s| s.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut r: HashSet<i32> = vec![].into_iter().collect();
    let mut last = 0;
    let mut found: Option<i32> = None;
    for _i in (0..).step_by(505) {
        for v in &result {
            let a = last + v;
            if !r.insert(a) {
                found = Some(a);
                break;
            }
            last = a;
        }
        if found.is_some() {
            break;
        }
    }
    found
}