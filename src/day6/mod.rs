use super::common::*;

use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &str = "./src/day6/input.txt";

fn manhattan_dist(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    ((pos1.0 as i32 - pos2.0 as i32).abs() + (pos1.1 as i32 - pos2.1 as i32).abs()) as usize
}

#[derive(Debug, Clone, PartialEq, Hash)]
struct Location {
    name: String,
    x: usize,
    y: usize,
}

impl Location {
    pub fn new(s: &str) -> Location {
        let a: Vec<&str> = s.split(',').map(|e| e.trim()).collect();
        let name = (if a.len() > 2 { a[2] } else { "" }).to_string();
        Location {
            name,
            x: a[0].parse::<usize>().unwrap(),
            y: a[1].parse::<usize>().unwrap(),
        }
    }

    pub fn distance_to(&self, x: usize, y: usize) -> usize {
        manhattan_dist((x, y), (self.x, self.y))
    }
}

impl Eq for Location {}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
    closest_dist: usize,
    closest_locations: Vec<Location>,
}

impl Coordinate {
    pub fn new(p: (usize, usize), locations: &[Location]) -> Coordinate {
        let closest_dist = locations
            .iter()
            .map(|location| location.distance_to(p.0, p.1))
            .min()
            .unwrap();

        let closest_locations: Vec<Location> = locations
            .iter()
            .cloned()
            .filter(|location| location.distance_to(p.0, p.1) == closest_dist)
            .collect();

        Coordinate {
            x: p.0,
            y: p.1,
            closest_dist,
            closest_locations,
        }
    }

    pub fn has_max_total_distance(&self, locations: &[Location], max_dist: usize) -> bool {
        let over = locations
            .iter()
            .scan(0usize, |state, location| {
                *state += manhattan_dist((self.x, self.y), (location.x, location.y));
                Some(state.clone())
            })
            .skip_while(|v| *v < max_dist)
            .take(1)
            .collect::<Vec<_>>();

        over.len() == 0
    }
}

fn parse_coords(input_str: &str) -> Vec<Location> {
    input_str
        .lines()
        .map(|s| Location::new(s))
        .collect::<Vec<_>>()
}

fn get_space_size(locations: &[Location]) -> (usize, usize) {
    let max_x = locations.iter().map(|location| location.x).max().unwrap() + 1;
    let max_y = locations.iter().map(|location| location.y).max().unwrap();
    (max_x, max_y)
}

fn create_space(locations: &[Location]) -> Vec<Coordinate> {
    let (max_x, max_y) = get_space_size(locations);

    (0..=max_y)
        .into_iter()
        .flat_map(|y| {
            (0..=max_x)
                .into_iter()
                .map(|x| Coordinate::new((x, y), &locations))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_infinite_coords(locations: &[Location]) -> HashSet<Location> {
    let space = create_space(&locations);
    let (max_x, max_y) = get_space_size(&locations);

    space
        .into_iter()
        .filter(|s| s.closest_locations.len() == 1)
        .flat_map(|s| {
            let Coordinate {
                x,
                y,
                closest_dist: _,
                closest_locations,
            } = s;

            if closest_locations.len() == 1 && (x == 0 || y == 0 || x == max_x || y == max_y) {
                closest_locations
            } else {
                vec![]
            }
        })
        .collect::<HashSet<_>>()
}

fn find_all_areas(coords: &[Location]) -> HashMap<(usize, usize), usize> {
    let space = create_space(coords);
    let infinite_coords = find_infinite_coords(&coords);

    let filtered = space
        .iter()
        .filter(|s| {
            s.closest_locations.len() == 1
                && !infinite_coords.contains(s.closest_locations.get(0).unwrap())
        })
        .flat_map(|s| &s.closest_locations)
        .collect::<Vec<_>>();;

    filtered
        .iter()
        .map(|c| (c.x, c.y))
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}

pub fn run1() -> usize {
    let input = read_file(INPUT);

    let locations = parse_coords(input.as_str());

    let areas = find_all_areas(&locations);

    areas.iter().map(|area| area.1).max().unwrap().clone()
}

fn find_all_with_distance_less_than(locations: &[Location], max_dist: usize) -> Vec<Coordinate> {
    let space = create_space(&locations);

    space
        .into_iter()
        .filter(|s| s.has_max_total_distance(&locations, max_dist))
        .collect::<Vec<_>>()
}

pub fn run2() -> usize {
    let input = read_file(INPUT);

    let locations = parse_coords(input.as_str());

    let coords = find_all_with_distance_less_than(&locations, 10000);

    coords.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r###"1, 1, A
                              1, 6, B
                              8, 3, C
                              3, 4, D
                              5, 5, E
                              8, 9, F"###;
    #[test]
    fn test1() {
        let locations = parse_coords(INPUT);
        assert_eq!(locations.len(), 6);
    }

    #[test]
    fn test2() {
        let locations = parse_coords(INPUT);
        let (max_x, max_y) = get_space_size(&locations);

        assert_eq!(max_x, 9);
        assert_eq!(max_y, 9);
    }

    #[test]
    fn test3() {
        let locations = parse_coords(INPUT);
        let space = create_space(&locations);

        assert_eq!(space.len(), (9 + 1) * (9 + 1));
    }

    #[test]
    fn test4() {
        let expected: String = r###"aaaaa.cccc
                                  aAaaa.cccc
                                  aaaddecccc
                                  aadddeccCc
                                  ..dDdeeccc
                                  bb.deEeecc
                                  bBb.eeee..
                                  bbb.eeefff
                                  bbb.eeffff
                                  bbb.ffffFf"###
            .lines()
            .map(|line| line.trim())
            .collect();
        println!("{}", expected);

        let locations = parse_coords(INPUT);
        let space = create_space(&locations);

        let actual = space
            .iter()
            .map(
                |Coordinate {
                     x: _,
                     y: _,
                     closest_dist: _,
                     closest_locations,
                 }| {
                    if closest_locations.len() > 1 {
                        ".".to_string()
                    } else {
                        closest_locations
                            .get(0)
                            .unwrap()
                            .name
                            .to_lowercase()
                            .clone()
                    }
                },
            )
            .collect::<String>();

        println!("{}", actual);

        assert_eq!(expected.to_lowercase(), actual);
    }

    #[test]
    fn test5() {
        let locations = parse_coords(INPUT);

        let infinite_coords = find_infinite_coords(&locations);

        println!("{:?}", infinite_coords);

        let mut chars = infinite_coords.iter().map(|c| &c.name).collect::<Vec<_>>();
        chars.sort();

        println!("{:?}", chars);

        assert_eq!(chars, vec!["A", "B", "C", "F"]);
    }

    #[test]
    fn test6() {
        let locations = parse_coords(INPUT);

        let result = find_all_areas(&locations);

        println!("{:?}", result);

        assert_eq!(result, hashmap! {(5,5) => 17, (3,4) => 9});
    }

    #[test]
    fn test7() {
        let locations = parse_coords(INPUT);

        let result = find_all_with_distance_less_than(&locations, 32);

        println!("{:?}", result);

        assert_eq!(16, result.len());
    }
}
