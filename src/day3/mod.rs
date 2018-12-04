use super::common::*;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &str = "./src/day3/input.txt";

#[derive(Debug, Clone)]
struct Row {
    name: String,
    x: usize,
    y: usize,
    x_width: usize,
    y_width: usize,
}

impl Row {
    pub fn new(row_str: &str) -> Row {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+).@.(\d+),(\d+):.(\d+)x(\d+)").unwrap();
        }
        let cap = RE.captures(row_str).unwrap();
        Row {
            name: cap[1].to_string(),
            x: cap[2].parse().unwrap(),
            y: cap[3].parse().unwrap(),
            x_width: cap[4].parse().unwrap(),
            y_width: cap[5].parse().unwrap(),
        }
    }

    pub fn coords(&self) -> HashSet<(usize, usize)> {
        let x_coords = self.x..self.x + self.x_width;
        let y_coords = self.y..self.y + self.y_width;
        iproduct!(x_coords, y_coords).collect::<HashSet<_>>()
    }

    pub fn not_in_any_coord(&self, coords: &HashSet<(usize, usize)>) -> bool {
        self.coords().is_disjoint(coords)
    }
}

fn overlapped_coords(rows: &[Row]) -> HashSet<(usize, usize)> {
    rows.iter()
        .flat_map(|e| e.coords())
        .fold(HashMap::<(usize, usize), usize>::new(), |mut acc, e| {
            *acc.entry(e).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|(_k, v)| *v > 1)
        .map(|((x, y), _)| (x, y))
        .collect::<HashSet<_>>()
}

pub fn run1() -> usize {
    let rows = read_file(INPUT)
        .par_lines()
        .map(Row::new)
        .collect::<Vec<_>>();

    overlapped_coords(&rows).len()
}

pub fn run2() -> String {
    let rows = read_file(INPUT)
        .par_lines()
        .map(Row::new)
        .collect::<Vec<_>>();

    let overlapped_coords = overlapped_coords(&rows);

    rows.iter()
        .filter(|e| e.not_in_any_coord(&overlapped_coords))
        .take(1)
        .collect::<Vec<_>>()
        .get(0)
        .map(|r| r.name.clone())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let rows: Vec<Row> = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
            .into_iter()
            .map(Row::new)
            .collect::<Vec<_>>();
        let overlapped = overlapped_coords(&rows);
        assert_eq!(overlapped.len(), 4);
    }

    #[test]
    fn test2() {
        let rows: Vec<Row> = vec!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"]
            .into_iter()
            .map(Row::new)
            .collect::<Vec<_>>();
        let overlapped = overlapped_coords(&rows);
        assert_eq!(rows[2].not_in_any_coord(&overlapped), true);
    }
}
