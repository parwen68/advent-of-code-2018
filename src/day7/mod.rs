use super::common::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &str = "./src/day7/input.txt";

fn parse(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| (line.as_bytes()[5], line.as_bytes()[36]))
        .collect::<Vec<_>>()
}

fn collect(pairs: &[(u8, u8)]) -> HashMap<u8, Vec<u8>> {
    pairs.iter().fold(HashMap::new(), |mut acc, pair| {
        let list = acc.entry(pair.0).or_insert(Vec::with_capacity(1));
        list.push(pair.1);
        acc
    })
}

fn find_candidates(
    collected: &HashMap<u8, Vec<u8>>,
    max_num: usize,
    filter: &HashSet<u8>,
) -> Vec<u8> {
    let keys = collected.keys().cloned().collect::<HashSet<_>>();
    let values = collected
        .values()
        .cloned()
        .flat_map(|v| v)
        .collect::<HashSet<_>>();

    let mut candidates = keys.difference(&values).cloned().collect::<HashSet<u8>>();
    filter.iter().for_each(|f| {
        candidates.remove(f);
    });
    let mut v: Vec<u8> = Vec::new();
    v.extend(candidates.iter());
    v.sort_by_key(|&key| key);

    let idx = max_num.min(v.len());

    v[0..idx].to_vec()
}

fn find_next(collected: &HashMap<u8, Vec<u8>>) -> u8 {
    find_candidates(&collected, 1, &HashSet::new())
        .get(0)
        .cloned()
        .unwrap()
}

fn find_order(parsed: &[(u8, u8)]) -> String {
    let mut collected = collect(&parsed);

    let mut result: Vec<u8> = vec![];
    while collected.len() > 0 {
        let next = find_next(&collected);
        result.push(next);
        let v = collected.remove(&next);
        if collected.is_empty() {
            result.push(*v.unwrap().first().unwrap())
        }
    }

    String::from_utf8(result).unwrap()
}

pub fn run1() -> String {
    let input = read_file(INPUT);

    let parsed = parse(&input);

    find_order(&parsed)
}

fn get_all_values(collected: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    collected
        .iter()
        .flat_map(|(k, v)| [&v[..], &[*k]].concat())
        .collect()
}

fn setup_not_finished(values: Vec<u8>, delay: i32) -> HashMap<u8, i32> {
    let not_finished = values
        .iter()
        .unique()
        .cloned()
        .map(|k| (k, k as i32 - 'A' as i32 + 1 + delay))
        .collect::<HashMap<u8, i32>>();
    not_finished
}

pub fn run2() -> usize {
    let input = read_file(INPUT);

    let parsed = parse(&input);

    let collected = collect(&parsed);

    let values = get_all_values(&collected);

    let not_finished = setup_not_finished(values, 60);
    let running = HashSet::<u8>::new();

    assemble(collected, not_finished, running, 5)
}

fn assemble(
    mut collected: HashMap<u8, Vec<u8>>,
    mut not_finished: HashMap<u8, i32>,
    mut running: HashSet<u8>,
    workers: usize,
) -> usize {
    let mut cnt = 0;
    while !not_finished.is_empty() {
        if running.len() < workers {
            let candidates = find_candidates(&collected, workers - running.len(), &running);
            running.extend(candidates.iter());
        }
        running
            .iter()
            .for_each(|&e| *not_finished.entry(e).or_default() -= 1);

        let finished = running
            .iter()
            .filter(|&e| *not_finished.get(e).unwrap() == 0)
            .cloned()
            .collect::<Vec<u8>>();

        finished.iter().for_each(|f| {
            not_finished.remove(f);
            running.remove(f);
            match collected.remove(f) {
                Some(e) => {
                    if collected.is_empty() {
                        running.insert(*e.get(0).unwrap());
                    }
                }
                None => (),
            }
        });
        cnt += 1;
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r###"Step C must be finished before step A can begin.
                             Step C must be finished before step F can begin.
                             Step A must be finished before step B can begin.
                             Step A must be finished before step D can begin.
                             Step B must be finished before step E can begin.
                             Step D must be finished before step E can begin.
                             Step F must be finished before step E can begin."###;

    #[test]
    fn test1() {
        let parsed = parse(INPUT);

        let result: Vec<(char, char)> = parsed
            .iter()
            .map(|(a, b)| (*a as char, *b as char))
            .collect();
        println!("{:?}", result);

        assert_eq!(
            result,
            vec![
                ('C', 'A'),
                ('C', 'F'),
                ('A', 'B'),
                ('A', 'D'),
                ('B', 'E'),
                ('D', 'E'),
                ('F', 'E')
            ]
        );
    }

    #[test]
    fn test2() {
        let parsed = parse(INPUT);

        let collected = collect(&parsed);

        println!("{:?}", collected);

        let mut keys = collected.keys().cloned().collect::<Vec<_>>();
        keys.sort_by_key(|key| *key);

        assert_eq!(keys, vec![65, 66, 67, 68, 70]);
    }

    #[test]
    fn test3() {
        let parsed = parse(INPUT);

        let mut collected = collect(&parsed);

        let next = find_next(&collected);

        assert_eq!(next, 67);
    }

    #[test]
    fn test4() {
        let parsed: Vec<(u8, u8)> = vec![(70, 69)];

        let mut collected = collect(&parsed);

        let next = find_next(&collected);

        assert_eq!(next, 70);
    }

    #[test]
    fn test5() {
        let parsed = parse(INPUT);

        let order = find_order(&parsed);

        println!("{:?}", order);

        assert_eq!("CABDFE", order);
    }

    #[test]
    fn test6() {
        let parsed = parse(INPUT);

        let mut collected = collect(&parsed);

        let values = get_all_values(&collected);

        let not_finished = setup_not_finished(values, 0);
        let running = HashSet::<u8>::new();

        let result = assemble(collected, not_finished, running, 2);

        println!("{:?}", result);

        assert_eq!(15, result);
    }
}
