use super::common::*;

use std::collections::{HashMap, HashSet};

pub fn run1() -> usize {
    let input = read_file("./src/day2/input.txt");

    let rows = input.split("\n");
    let twos = rows.clone().filter(|e| count_two(e)).count();
    let threes = rows.clone().filter(|e| count_three(e)).count();

    twos * threes
}

fn count(num: u32, row: &str) -> bool {
    return row.chars().fold(HashMap::<char, u32>::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    }).iter().filter(|(_,v)| **v == num).count() > 0;
}

pub fn count_two(row: &str) -> bool {
    return count(2, row);
}

pub fn count_three(row: &str) -> bool {
    return count(3, row);
}

pub fn run2() -> HashSet<String> {
    let input = read_file("./src/day2/input.txt");

    let rows1 = input.split("\n").collect::<Vec<_>>();
    let rows2 = rows1.clone();

    let row_len: usize = rows1.iter().map(|r| r.len()).next().unwrap();

    rows1.iter().flat_map(|&e1| rows2.iter().map(move |e2| cmp(e1,e2)))
        .filter(|s| s.len() == row_len - 1)
        .collect::<HashSet<_>>()
}

fn cmp(str1: &str, str2: &str) -> String{
    str1.chars().zip(str2.chars())
        .filter(|(a,b)| a == b)
        .map(|(c1, _)| c1)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let str = "abcdef";
        assert_eq!(false, count_two(str));
        assert_eq!(false, count_three(str));
    }

    #[test]
    fn test2() {
        let str = "bababc";
        assert_eq!(true, count_two(str));
        assert_eq!(true, count_three(str));
    }

    #[test]
    fn test3() {
        let str = "abbcde";
        assert_eq!(true, count_two(str));
        assert_eq!(false, count_three(str));
    }

    #[test]
    fn test4() {
        let str = "abcccd";
        assert_eq!(false, count_two(str));
        assert_eq!(true, count_three(str));
    }

    #[test]
    fn test5() {
        let str = "aabcdd";
        assert_eq!(true, count_two(str));
        assert_eq!(false, count_three(str));
    }

    #[test]
    fn test6() {
        let str = "abcdee";
        assert_eq!(true, count_two(str));
        assert_eq!(false, count_three(str));
    }

    #[test]
    fn test7() {
        let str = "ababab";
        assert_eq!(false, count_two(str));
        assert_eq!(true, count_three(str));
    }

    #[test]
    fn test8() {
        assert_eq!("ace", cmp("abcde", "axcye"));
        assert_eq!("fgij", cmp("fghij", "fguij"));
    }

    #[test]
    fn test9() {
        let rows1 = vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"];
        let rows2 = rows1.clone();

        let b =
            rows1.iter().flat_map(|&e1| rows2.iter().map(move |e2| cmp(e1,e2)))
                .filter(|s| s.len() == 4)
                .collect::<HashSet<_>>();

        let a: HashSet<String> = vec!["fgij".to_string()].into_iter().collect();
        println!("{:?}", b);
        assert_eq!(vec!["fgij".to_string()].into_iter().collect::<HashSet<_>>(), b);

    }
}