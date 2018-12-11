use std::collections::HashMap;

static INPUT: isize = 3613;

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub struct Coord(isize, isize);

impl Eq for Coord {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell(Coord, isize);

fn calc(cell: &mut Cell, serial: isize) {
    let Cell(Coord(x, y), power) = cell;

    let rack_id = *x + 10;
    *power = ((rack_id * *y + serial) * rack_id / 100) % 10 - 5;
}

fn set_up(serial: isize) -> Vec<Cell> {
    let mut cells = Vec::with_capacity(300 * 300);
    for x in 1..=300 {
        for y in 0..=300 {
            cells.push(Cell(Coord(x, y), 0))
        }
    }
    for cell in cells.iter_mut() {
        calc(cell, serial)
    }
    cells
}

fn mask(cell: &Cell) -> Vec<Coord> {
    let Cell(Coord(x, y), _) = cell;
    let xs: Vec<isize> = vec![0, 1, 2].iter().map(|dx| x + dx).collect();
    let ys: Vec<isize> = vec![0, 1, 2].iter().map(|dx| y + dx).collect();

    xs.iter()
        .flat_map(|x| ys.iter().map(move |y| Coord(*x, *y)))
        .collect()
}

fn find(serial: isize) -> Cell {
    let cells = set_up(serial);

    let cell_map: HashMap<Coord, isize> = cells
        .iter()
        .map(|Cell(coord, power)| (coord.clone(), *power))
        .collect();

    let d = cells
        .iter()
        .filter(|Cell(Coord(x, y), _)| *x < 298 && *y < 298)
        .fold((Cell(Coord(0, 0), 0), 0), |acc, cell| {
            let sum = mask(cell)
                .iter()
                .map(|coord| cell_map.get(coord).unwrap())
                .sum::<isize>();
            if sum > acc.1 {
                (cell.clone(), sum)
            } else {
                acc
            }
        });

    d.0
}

fn find2(serial: isize) -> Cell {
    let cells = set_up(serial);

    let cell_map: HashMap<Coord, isize> = cells
        .iter()
        .map(|Cell(coord, power)| (coord.clone(), *power))
        .collect();

    let d = cells
        .iter()
        .filter(|Cell(Coord(x, y), _)| *x < 298 && *y < 298)
        .fold((Cell(Coord(0, 0), 0), 0), |acc, cell| {
            let sum = mask(cell)
                .iter()
                .map(|coord| cell_map.get(coord).unwrap())
                .sum::<isize>();
            if sum > acc.1 {
                (cell.clone(), sum)
            } else {
                acc
            }
        });

    d.0
}

pub fn run1() -> Cell {
    find(INPUT)
}

pub fn run2() -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut cell = Cell(Coord(3, 5), 0);

        calc(&mut cell, 8);
        assert_eq!(cell, Cell(Coord(3, 5), 4))
    }

    #[test]
    fn test2() {
        let mut cell = Cell(Coord(122, 79), 0);

        calc(&mut cell, 57);
        assert_eq!(cell, Cell(Coord(122, 79), -5))
    }

    #[test]
    fn test3() {
        let mut cell = Cell(Coord(217, 196), 0);

        calc(&mut cell, 39);
        assert_eq!(cell, Cell(Coord(217, 196), 0))
    }

    #[test]
    fn test4() {
        let mut cell = Cell(Coord(101, 153), 0);

        calc(&mut cell, 71);
        assert_eq!(cell, Cell(Coord(101, 153), 4))
    }
}
