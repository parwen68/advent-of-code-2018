use super::common::*;
use regex::Regex;
use std::collections::HashMap;

static INPUT: &str = "./src/day4/input.txt";

#[derive(Debug)]
struct Guard {
    id: usize,
    minutes: Vec<usize>,
}

impl Guard {
    pub fn new(id: usize, minutes: Vec<usize>) -> Guard {
        Guard { id, minutes }
    }

    pub fn minutes(&self) -> usize {
        self.minutes.len()
    }

    pub fn max_minute(&self) -> (usize, usize) {
        let a = self.minutes.iter().fold(HashMap::new(), |mut m, &c| {
            *m.entry(c).or_insert(0) += 1;
            m
        });

        a.into_iter().max_by_key(|(_, v)| *v).unwrap()
    }
}

pub fn run1() -> usize {
    let guards = get_guards();
    let guard = guards.iter().max_by_key(|guard| guard.minutes()).unwrap();
    guard.id * guard.max_minute().0
}

pub fn run2() -> usize {
    let guards = get_guards();
    let guard = guards
        .iter()
        .max_by_key(|guard| guard.max_minute().1)
        .unwrap();
    guard.id * guard.max_minute().0
}

fn get_guards() -> Vec<Guard> {
    let mut input: Vec<String> = read_file(INPUT)
        .split('\n')
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    input.sort();

    lazy_static! {
        static ref match1: Regex = Regex::new(r"\[[\d-]+.(\d{2}):(\d{2})\].*").unwrap();
        static ref match2: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    }

    let r = input
        .into_iter()
        .scan(0, |state, e| {
            *state = match2
                .captures(e.as_str())
                .map(|e| e[1].parse::<usize>().unwrap())
                .unwrap_or(*state);

            let time: (usize, usize) = match1
                .captures(e.as_str())
                .map(|e| (e[1].parse().unwrap(), e[2].parse().unwrap()))
                .unwrap();

            Some(if e.contains("wakes") {
                Some((*state, time, "wakes"))
            } else if e.contains("sleep") {
                Some((*state, time, "sleep"))
            } else {
                None
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    r.chunks(2)
        .map(|c| (c[0].0, (c[0].1).1..(c[1].1).1))
        .fold(HashMap::<usize, Vec<usize>>::new(), |mut acc, e| {
            acc.entry(e.0)
                .or_insert(vec![])
                .append(&mut e.1.collect::<Vec<_>>());
            acc
        })
        .iter()
        .map(|(k, v)| Guard::new(*k, v.clone()))
        .collect()
}
