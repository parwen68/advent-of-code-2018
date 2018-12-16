use std::collections::VecDeque;

fn set_up(input: &str) -> VecDeque<char> {
    input.chars().collect::<VecDeque<_>>()
}

fn get_rules(input: &str) -> Vec<Vec<&str>> {
    input
        .split('\n')
        .map(|rule| rule.split("=>").map(|s| s.trim()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn generation(
    plants: &mut VecDeque<char>,
    rules: &Vec<Vec<&str>>,
    index: &mut isize,
) -> VecDeque<char> {
    for i in 0..4 {
        plants.push_front('.');
        plants.push_back('.');
    }

    *index = *index - 2;

    let mut r = plants
        .iter()
        .scan((-1, "".to_string()), |state, e| {
            if state.1.len() == 5 {
                state.1 = state.1[1..].to_string()
            }
            state.1.push(*e);

            Some(state.1.clone())
        })
        .filter(|s| s.len() == 5)
        .map(|s| {
            let e = rules.iter().find(|&rule| *rule[0] == *s.as_str());
            e.map_or('.', |v| v[1].chars().next().unwrap())
        })
        .collect::<VecDeque<_>>();

    let mut e = r.pop_front();
    while e == Some('.') {
        *index += 1;
        e = r.pop_front();
    }
    r.push_front(e.unwrap());

    e = r.pop_back();
    while e == Some('.') {
        e = r.pop_back();
    }
    r.push_back(e.unwrap());
    r
}

fn generate(plants: &mut VecDeque<char>, rules: &Vec<Vec<&str>>, generations: usize) -> isize {
    let mut index = 0;
    for i in 0..generations {
        println!("{} {} {}", i, index, plants.iter().collect::<String>());
        *plants = generation(plants, &rules, &mut index);
    }

    let sum = plants.iter().fold((index, 0), |acc, e| {
        let r = acc.1 + if *e == '#' { acc.0 } else { 0 };
        (acc.0 + 1, r)
    });

    sum.1
}

pub fn run1() -> isize {
    let mut state = set_up(INITIAL_STATE);
    let rules = get_rules(RULES);

    generate(&mut state, &rules, 20)
}

pub fn run2() -> isize {
    let mut state = set_up(INITIAL_STATE);
    let rules = get_rules(RULES);

    // Pattern stabilizes after 129 generations, just "gliding" to the left
    (50000000000 - 129) * 52 + 8580

    //generate(&mut state, &rules, 50000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_TEST: &str = r"#..#.#..##......###...###";
    const RULES_TEST: &str = r###"...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
"###;

    #[test]
    fn test1() {
        println!("{}", INITIAL_TEST);
        let mut init = set_up(INITIAL_TEST);
        let rules = get_rules(RULES_TEST);

        println!("{:?}", rules);

        let mut index = 0;
        let g1 = generation(&mut init, &rules, &mut index);

        assert_eq!(7, g1.iter().filter(|&c| *c == '#').count());
    }

    #[test]
    fn test2() {
        let mut state = set_up(INITIAL_TEST);
        let rules = get_rules(RULES_TEST);

        let r = generate(&mut state, &rules, 20);

        assert_eq!(r, 325);
        assert_eq!(1, 2)
    }

}

const INITIAL_STATE: &str =r"###.......##....#.#.#..###.##..##.....#....#.#.....##.###...###.#...###.###.#.###...#.####.##.#....#";

const RULES: &str = r###"..... => .
#..## => .
..### => #
..#.# => #
.#.#. => .
####. => .
##.## => #
#.... => .
#...# => .
...## => .
##..# => .
.###. => #
##### => #
#.#.. => #
.##.. => #
.#.## => .
...#. => #
#.##. => #
..#.. => #
##... => #
....# => .
###.# => #
#..#. => #
#.### => #
##.#. => .
###.. => #
.#### => .
.#... => #
..##. => .
.##.# => .
#.#.# => #
.#..# => ."###;
