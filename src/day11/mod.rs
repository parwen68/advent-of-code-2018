use rayon::prelude::*;
use std::collections::HashMap;

static INPUT: isize = 3613;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub struct Coord(isize, isize);

impl Eq for Coord {}

fn calc(coord: &Coord, serial: isize) -> isize {
    let Coord(x, y) = coord;

    let rack_id = *x + 10;
    ((rack_id * *y + serial) * rack_id / 100) % 10 - 5
}

fn translate(x: isize, y: isize) -> usize {
    ((x - 1) + 300 * (y - 1)) as usize
}

fn translate_back(p: usize) -> Coord {
    let y = p / 300 + 1;
    let x = p % 300 + 1;
    Coord(x as isize, y as isize)
}

fn set_up(serial: isize) -> Vec<isize> {
    let mut cells: Vec<isize> = vec![0; 300 * 300];

    for p in 0..300 * 300 {
        cells[p] = calc(&translate_back(p), serial)
    }
    cells
}

fn mask(size: isize) -> Vec<usize> {
    let xs: Vec<isize> = (1..=size).map(|dx| dx).collect();
    let ys: Vec<isize> = (1..=size).map(|dy| dy).collect();

    xs.iter()
        .flat_map(|x| ys.iter().map(move |y| translate(*x, *y)))
        .collect()
}

fn find(serial: isize, size: isize) -> (Coord, isize) {
    let cells = set_up(serial);

    let x_max = 300 - size;
    let y_max = 300 - size;

    let mask = mask(size);

    let d = (0..300 * 300)
        .filter(|p| {
            let Coord(x, y) = translate_back(*p);
            x < x_max && y < y_max
        })
        .fold((0usize, std::isize::MIN), |r, p| {
            let sum = mask.iter().map(|c| cells[p + *c as usize]).sum();
            if sum > r.1 {
                (p, sum)
            } else {
                r
            }
        });

    (translate_back(d.0), d.1)
}

pub fn run1() -> String {
    let r = find(INPUT, 3);

    format!("{},{}", (r.0).0, (r.0).1)
}

pub fn run2() -> String {
    let r: (isize, (Coord, isize)) = (1..300isize)
        .into_par_iter()
        .map(|i| (i, find(INPUT, i)))
        .max_by_key(|e| (e.1).1)
        .unwrap();

    format!("{},{},{}", ((r.1).0).0, ((r.1).0).1, r.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut coord = Coord(3, 5);

        let v = calc(&coord, 8);
        assert_eq!(v, 4)
    }

    #[test]
    fn test2() {
        let mut coord = Coord(122, 79);

        let v = calc(&coord, 57);
        assert_eq!(v, -5)
    }

    #[test]
    fn test3() {
        let mut coord = Coord(217, 196);

        let v = calc(&coord, 39);
        assert_eq!(v, 0)
    }

    #[test]
    fn test4() {
        let mut coord = Coord(101, 153);

        let v = calc(&coord, 71);
        assert_eq!(v, 4)
    }
}
