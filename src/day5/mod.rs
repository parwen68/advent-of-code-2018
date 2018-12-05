use super::common::*;
use rayon::prelude::*;
use std::borrow::BorrowMut;

static INPUT: &str = "./src/day5/input.txt";

pub fn run1() -> usize {
    let input = read_file(INPUT);

    collapse(&input).len()
}

fn collapse(input: &str) -> String {
    let mut data = Vec::from(input);
    let mut cont: Option<&mut Vec<u8>> = Some(data.borrow_mut());

    // Use scan instead??
    while let Some(inner) = cont {
        let a = inner
            .windows(2)
            .enumerate()
            .filter(|(_, s)| (s[0] as i32 - s[1] as i32).abs() == 32)
            .map(|(i, _)| i)
            .take(1)
            .collect::<Vec<_>>();

        match a.get(0) {
            None => {
                cont = None;
            }
            Some(&index) => {
                inner.remove(index);
                inner.remove(index);
                cont = Some(inner);
            }
        }
    }

    String::from_utf8(data).unwrap()
}

pub fn run2() -> Option<usize> {
    let input = read_file(INPUT);

    find(&input).iter().map(|(_, s)| *s).min()
}

fn filter(input: &str, mask: &str) -> String {
    let v = Vec::from(input);
    let m = Vec::from(mask);

    let filtered = v
        .iter()
        .filter(|&v| !m.contains(v))
        .cloned()
        .collect::<Vec<_>>();

    String::from_utf8(filtered).unwrap()
}

fn find(input: &str) -> Vec<(String, usize)> {
    let letters = Vec::from("abcdefghijklmnopqrstuvwxyz");

    let a = letters
        .par_iter()
        .map(|&s| String::from_utf8(vec![s - 32, s]).unwrap())
        .collect::<Vec<_>>();

    a.iter()
        .map(|f| (f.clone(), collapse(&filter(&input, &f)).len()))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "dabAcCaCBAcCcaDA";

        let result = collapse(input);

        println!("{:?}", result);
        assert_eq!(result, "dabCBAcaDA");
    }

    #[test]
    fn test2() {
        let result = filter("dabAcCaCBAcCcaDA", "Aa");

        assert_eq!(result, "dbcCCBcCcD");
    }

    #[test]
    fn test3() {
        let result = filter("dabAcCaCBAcCcaDA", "Bb");

        assert_eq!(result, "daAcCaCAcCcaDA");
    }

    #[test]
    fn test4() {
        let result = filter("dabAcCaCBAcCcaDA", "Cc");

        assert_eq!(result, "dabAaBAaDA");
    }

    #[test]
    fn test5() {
        let result = filter("dabAcCaCBAcCcaDA", "Dd");

        assert_eq!(result, "abAcCaCBAcCcaA");
    }

    #[test]
    fn test6() {
        let result = find("dabAcCaCBAcCcaDA");

        println!("{:?}", result);
        assert_eq!(result[0], ("Aa".to_string(), 6));
        assert_eq!(result[1], ("Bb".to_string(), 8));
        assert_eq!(result[2], ("Cc".to_string(), 4));
        assert_eq!(result[3], ("Dd".to_string(), 6));
    }
}
