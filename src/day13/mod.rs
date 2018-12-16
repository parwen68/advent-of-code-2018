use super::common::*;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;

static INPUT: &str = "./src/day13/input.txt";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
#[derive(Debug, PartialEq)]
enum TrackType {
    Horizontal,      // -
    Vertical,        // |
    Intersection,    // +
    BottomRight,     // /      -/, +/
    TopRight,        // \      -\, +\
    TopLeft,         // /      /-, /+
    BottomLeft,      // \      \-, \+
    Cart(Direction), // <
    Empty,           //
}

#[derive(Debug, PartialOrd, PartialEq, Hash, Copy, Clone)]
struct Coord(isize, isize);

impl Eq for Coord {}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Equal => self.0.cmp(&other.0),
            v => v,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Cart {
    id: usize,
    direction: Direction,
    turn: Turn,
    coord: Coord,
}

impl Cart {
    fn new(id: usize, direction: Direction, coord: Coord) -> Cart {
        Cart {
            id,
            direction,
            coord,
            turn: Turn::Left,
        }
    }

    fn turn(&mut self) {
        self.direction = match (&self.direction, &self.turn) {
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Left, Turn::Right) => Direction::Up,
            _ => self.direction,
        };
        self.turn = match self.turn {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        };
    }

    fn tick(&mut self, tracks: &BTreeMap<Coord, TrackType>) {
        match self.direction {
            Direction::Up => self.move_up(tracks),
            Direction::Right => self.move_right(tracks),
            Direction::Down => self.move_down(tracks),
            Direction::Left => self.move_left(tracks),
        }
    }

    fn move_up(&mut self, tracks: &BTreeMap<Coord, TrackType>) {
        self.coord.1 -= 1;
        match tracks.get(&self.coord) {
            Some(TrackType::TopLeft) => self.direction = Direction::Right,
            Some(TrackType::TopRight) => self.direction = Direction::Left,
            Some(TrackType::Vertical) => self.direction = Direction::Up,
            Some(TrackType::Intersection) => self.turn(),
            _ => panic!("unexpected move up {:?}", self),
        }
    }

    fn move_left(&mut self, tracks: &BTreeMap<Coord, TrackType>) {
        self.coord.0 -= 1;
        match tracks.get(&self.coord) {
            Some(TrackType::TopLeft) => self.direction = Direction::Down,
            Some(TrackType::BottomLeft) => self.direction = Direction::Up,
            Some(TrackType::Horizontal) => self.direction = Direction::Left,
            Some(TrackType::Intersection) => self.turn(),
            _ => panic!("unexpected move left {:?}", self),
        }
    }

    fn move_down(&mut self, tracks: &BTreeMap<Coord, TrackType>) {
        self.coord.1 += 1;
        match tracks.get(&self.coord) {
            Some(TrackType::BottomLeft) => self.direction = Direction::Right,
            Some(TrackType::BottomRight) => self.direction = Direction::Left,
            Some(TrackType::Vertical) => self.direction = Direction::Down,
            Some(TrackType::Intersection) => self.turn(),
            _ => panic!("unexpected move down {:?}", self),
        }
    }

    fn move_right(&mut self, tracks: &BTreeMap<Coord, TrackType>) {
        self.coord.0 += 1;
        match tracks.get(&self.coord) {
            Some(TrackType::BottomRight) => self.direction = Direction::Up,
            Some(TrackType::TopRight) => self.direction = Direction::Down,
            Some(TrackType::Horizontal) => self.direction = Direction::Right,
            Some(TrackType::Intersection) => self.turn(),
            _ => panic!("unexpected move down {:?}", self),
        }
    }
}

fn parse(input: &str) -> BTreeMap<Coord, char> {
    input
        .split('\n')
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| (Coord(x as isize, y as isize), c))
        })
        .collect::<BTreeMap<Coord, char>>()
}

fn get_carts(tracks: &BTreeMap<Coord, char>) -> Vec<Cart> {
    tracks
        .iter()
        .enumerate()
        .flat_map(|t| match (t.1).1 {
            '^' => Some(Cart::new(t.0, Direction::Up, (t.1).0.clone())),
            '>' => Some(Cart::new(t.0, Direction::Right, (t.1).0.clone())),
            'v' => Some(Cart::new(t.0, Direction::Down, (t.1).0.clone())),
            '<' => Some(Cart::new(t.0, Direction::Left, (t.1).0.clone())),
            _ => None,
        })
        .collect::<Vec<Cart>>()
}

fn replace_carts(tracks: &mut BTreeMap<Coord, char>) {
    tracks.iter_mut().for_each(|kv| match kv.1 {
        '^' => *kv.1 = '|',
        '>' => *kv.1 = '-',
        'v' => *kv.1 = '|',
        '<' => *kv.1 = '-',
        _ => (),
    })
}

fn to_map(tracks: BTreeMap<Coord, char>) -> BTreeMap<Coord, TrackType> {
    tracks
        .keys()
        .map(|coord| {
            let x_min = if coord.0 == 0 { 0 } else { coord.0 - 1 };
            let x_max = coord.0 + 1;

            let r = (
                tracks.get(&Coord(x_min, coord.1)),
                tracks.get(coord),
                tracks.get(&Coord(x_max, coord.1)),
            );

            (
                coord.clone(),
                match r {
                    (_, Some('-'), _) => TrackType::Horizontal,
                    (_, Some('|'), _) => TrackType::Vertical,
                    (_, Some('+'), _) => TrackType::Intersection,
                    (Some('-'), Some('/'), _) => TrackType::BottomRight,
                    (Some('+'), Some('/'), _) => TrackType::BottomRight,
                    (_, Some('/'), Some('+')) => TrackType::TopLeft,
                    (_, Some('/'), Some('-')) => TrackType::TopLeft,
                    (Some('-'), Some('\\'), _) => TrackType::TopRight,
                    (Some('+'), Some('\\'), _) => TrackType::TopRight,
                    (_, Some('\\'), Some('+')) => TrackType::BottomLeft,
                    (_, Some('\\'), Some('-')) => TrackType::BottomLeft,
                    (_, Some('<'), _) => TrackType::Cart(Direction::Left),
                    (_, Some('>'), _) => TrackType::Cart(Direction::Right),
                    (_, Some('^'), _) => TrackType::Cart(Direction::Up),
                    (_, Some('v'), _) => TrackType::Cart(Direction::Down),
                    (_, _, _) => TrackType::Empty,
                },
            )
        })
        .filter(|e| e.1 != TrackType::Empty)
        .collect()
}

fn find_crashes(carts: &mut Vec<Cart>, tracks: &BTreeMap<Coord, TrackType>) -> Coord {
    (1..)
        .map(|_turn| {
            carts.sort_by_key(|cart| cart.coord);
            carts.iter_mut().for_each(|cart| cart.tick(&tracks));

            let map = carts
                .iter()
                .fold(HashMap::<Coord, usize>::new(), |mut acc, e| {
                    *acc.entry(e.coord).or_insert(0) += 1;
                    acc
                });

            let crashes = map.into_iter().filter(|&kv| kv.1 > 1).collect::<Vec<_>>();
            if crashes.len() > 0 {
                Some(crashes)
            } else {
                None
            }
        })
        .skip_while(|e| e.is_none())
        .next()
        .unwrap()
        .unwrap()
        .first()
        .unwrap()
        .to_owned()
        .0
}

fn count(carts: &Vec<Cart>) -> Vec<usize> {
    let map = carts
        .iter()
        .fold(HashMap::<Coord, usize>::new(), |mut acc, e| {
            *acc.entry(e.coord).or_insert(0) += 1;
            acc
        });

    let crashes = map
        .into_iter()
        .filter(|&kv| kv.1 > 1)
        .map(|(coord, _)| coord)
        .collect::<Vec<_>>();

    carts
        .iter()
        .filter(|cart| crashes.contains(&cart.coord))
        .map(|cart| cart.id)
        .collect()
}

fn find_last(carts: &mut Vec<Cart>, tracks: &BTreeMap<Coord, TrackType>) -> Cart {
    let r: &Cart = (1..)
        .fold_while(carts, |carts, _turn| {
            carts.sort_by_key(|cart| cart.coord);

            let mut crashed_carts = Vec::new();

            for i in 0..carts.len() {
                if let Some(cart) = carts.get_mut(i) {
                    cart.tick(&tracks)
                }
                crashed_carts.append(&mut count(&carts));
            }

            while let Some(index) = carts
                .iter()
                .enumerate()
                .find(|(_, cart)| crashed_carts.contains(&cart.id))
                .map(|(index, _)| index)
            {
                carts.remove(index);
            }

            if carts.len() == 1 {
                Done(carts)
            } else {
                Continue(carts)
            }
        })
        .into_inner()
        .get(0)
        .unwrap();

    r.clone()
}

pub fn run1() -> String {
    let input = read_file(INPUT);
    let mut parsed = parse(&input);
    let mut carts = get_carts(&parsed);

    replace_carts(&mut parsed);

    let tracks = to_map(parsed);

    let crash = find_crashes(&mut carts, &tracks);

    format!("{},{}", crash.0, crash.1)
}

pub fn run2() -> String {
    let input = read_file(INPUT);
    let mut parsed = parse(&input);
    let mut carts = get_carts(&parsed);

    replace_carts(&mut parsed);

    let tracks = to_map(parsed);

    let cart = find_last(&mut carts, &tracks);

    format!("{},{}", cart.coord.0, cart.coord.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const TRACKS: &str = r###"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
"###;

    #[test]
    fn test1() {
        let mut parsed = parse(TRACKS);
        let mut carts = get_carts(&parsed);
        replace_carts(&mut parsed);

        println!("{:?}", parsed);

        let mut tracks = to_map(parsed);

        tracks.iter().for_each(|e| println!("{:?}", e));

        carts.sort_by_key(|cart| cart.coord);

        carts.iter().for_each(|e| println!("{:?}", e));

        let crash = find_crashes(&mut carts, &tracks);

        println!("{:?}", crash);

        assert_eq!(Coord(7, 3), crash)
    }

    const TRACKS_CRASH: &str = r###"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"###;

    #[test]
    fn test2() {
        let mut parsed = parse(TRACKS_CRASH);
        let mut carts = get_carts(&parsed);

        replace_carts(&mut parsed);

        let mut tracks = to_map(parsed);
        tracks.iter().for_each(|e| println!("{:?}", e));

        tracks.iter().for_each(|e| println!("{:?}", e));

        let mut cart = find_last(&mut carts, &tracks);

        println!("{:?}", cart);

        assert_eq!(Coord(6, 4), cart.coord)
    }
}
