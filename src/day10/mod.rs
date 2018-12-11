use super::common::*;
use itertools::Itertools;
use regex::Regex;

static INPUT: &str = "./src/day10/input.txt";

#[derive(Debug, Copy, Clone, PartialEq)]
struct Star(isize, isize, isize, isize);

fn parse_coord(s: &str) -> Vec<Star> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"position=<\s*([-\d]+),\s*([-\d]+)>\s+velocity=<\s*([-\d]+),\s*([-\d]+)>")
                .unwrap();
    }

    RE.captures_iter(s)
        .map(|cap| {
            Star(
                cap[1].parse::<isize>().unwrap(),
                cap[2].parse::<isize>().unwrap(),
                cap[3].parse::<isize>().unwrap(),
                cap[4].parse::<isize>().unwrap(),
            )
        })
        .collect::<Vec<Star>>()
}

fn generation(stars: &mut Vec<Star>) -> &Vec<Star> {
    for Star(x, y, dx, dy) in stars.iter_mut() {
        *x = *x + *dx;
        *y = *y + *dy;
    }
    stars
}

fn is_neighbor(star: &Star, other: &Star) -> bool {
    (star.0 - other.0).abs() < 2 && (star.1 - other.1).abs() < 2
}

fn has_neighbour(star: &Star, stars: &Vec<Star>) -> bool {
    let a = stars
        .iter()
        .filter(|&other| *other != *star)
        .map(|other| is_neighbor(&star, &other))
        .skip_while(|&p| !p)
        .next();

    match a {
        Some(_) => true,
        None => false,
    }
}

fn all_stars_have_neighbour(stars: &Vec<Star>) -> bool {
    let cnt = stars
        .iter()
        .map(|star| has_neighbour(&star, &stars))
        .take_while(|e| *e)
        .filter(|e| *e)
        .count();

    cnt == stars.len()
}

fn print(stars: &Vec<Star>) -> Vec<String> {
    let (x_min, x_max, y_min, y_max) = (
        stars.iter().map(|Star(x, y, _, _)| x).min().unwrap(),
        stars.iter().map(|Star(x, y, _, _)| x).max().unwrap(),
        stars.iter().map(|Star(x, y, _, _)| y).min().unwrap(),
        stars.iter().map(|Star(x, y, _, _)| y).max().unwrap(),
    );
    let row_length = *x_max - *x_min + 1;

    let mut coords = stars
        .iter()
        .map(|Star(x, y, _, _)| Star(x - x_min, y - y_min, 0, 0))
        .map(|Star(x, y, _, _)| x + y * row_length)
        .unique()
        .collect::<Vec<_>>();
    coords.sort();

    let c = coords
        .iter()
        .scan(0, |state, next| {
            let mut e = vec![0; (*next - *state) as usize];
            e.push(1);
            *state = *next + 1;
            Some(e)
        })
        .flat_map(|e| e)
        .collect::<Vec<usize>>();

    c.chunks(row_length as usize)
        .map(|c| {
            c.iter()
                .map(|e| if *e > 0 { '*' } else { ' ' })
                .collect::<String>()
        })
        .collect()
}

pub fn run1() -> Vec<String> {
    // 10136
    let input = read_file(INPUT);
    let mut stars = parse_coord(&input);

    let mut seconds = 1;
    loop {
        generation(&mut stars);
        if all_stars_have_neighbour(&stars) {
            break;
        }
        seconds += 1
    }

    print(&stars)
}

pub fn run2() -> usize {
    let input = read_file(INPUT);
    let mut stars = parse_coord(&input);

    let mut seconds = 1;
    loop {
        generation(&mut stars);
        if all_stars_have_neighbour(&stars) {
            break;
        }
        seconds += 1
    }

    seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let star1 = Star(2, 3, -1, -1);
        let star2 = Star(3, 4, -1, -1);
        let star3 = Star(4, 5, -1, -1);

        assert_eq!(true, is_neighbor(&star1, &star2));
        assert_eq!(true, is_neighbor(&star2, &star3));
        assert_ne!(true, is_neighbor(&star1, &star3));
    }

    #[test]
    fn test2() {
        let star1 = Star(2, 3, -1, -1);
        let star2 = Star(3, 4, -1, -1);
        let star3 = Star(4, 5, -1, -1);

        let others = vec![star2, star3];

        assert_eq!(true, has_neighbour(&star1, &others));
    }

    #[test]
    fn test3() {
        let input = read_file("./src/day10/test.txt");
        let mut stars = parse_coord(&input);

        println!("{:?}", stars.len());

        let mut seconds = 1;
        loop {
            generation(&mut stars);
            if all_stars_have_neighbour(&stars) {
                break;
            }
            seconds += 1;
            if seconds > 5 {
                break;
            }
        }

        print(&stars);

        println!("{:?}", seconds);

        assert_eq!(seconds, 2);
    }
}
