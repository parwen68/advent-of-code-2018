use super::common::*;

static INPUT: &str = "./src/day8/input.txt";

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_node<'a>(
    input: &'a [usize],
    stack: &mut Vec<(usize, usize, usize)>,
    meta: &mut Vec<usize>,
    values: &mut Vec<usize>,
) -> &'a [usize] {
    let ((num_child_nodes, num_metadata), rest) = ((input[0], input[1]), &input[2..]);

    //println!("({:?},{:?}) rest={:?}", num_child_nodes, num_metadata, rest);

    if stack.len() > 0 {
        let last_index = stack.len() - 1;
        stack[last_index].0 -= 1;
    }

    if num_child_nodes == 0 {
        let metadata = &rest[..num_metadata];
        let value: usize = metadata.iter().cloned().sum();
        //println!("push={:?} {:?}", value, metadata);
        values.push(value);

        meta.extend_from_slice(metadata);
        &rest[num_metadata..]
    } else {
        stack.push((num_child_nodes, num_metadata, num_child_nodes));
        rest
    }
}

fn handle_stack<'a>(
    input: &'a [usize],
    stack: &mut Vec<(usize, usize, usize)>,
    meta: &mut Vec<usize>,
    values: &mut Vec<usize>,
) -> &'a [usize] {
    //println!("values={:?}", values);

    let mut r = input;
    if stack.len() > 0 {
        for i in (0..stack.len()).rev() {
            if stack[i].0 != 0 {
                break;
            }
            let m = &r[..stack[i].1];
            let num_children = stack[i].2;
            let index = values.len() - num_children;
            let pop = values.drain(index..).collect::<Vec<_>>();
            //println!("pop: {:?}", pop);
            //println!("meta: {:?}", m);
            //println!("values: {:?}", values);
            let sum: usize = m
                .iter()
                .filter(|&v| *v > 0 && *v <= pop.len())
                .map(|&v| &pop[v - 1])
                .sum();
            //println!("sum: {:?}", sum);
            values.push(sum);

            meta.extend_from_slice(m);
            r = &r[(stack[i].1)..];
            stack.remove(stack.len() - 1);
        }
    }
    r
}

fn parse3(input: &[usize]) -> (Vec<usize>, usize) {
    let mut stack: Vec<(usize, usize, usize)> = Vec::new();
    let mut meta: Vec<usize> = Vec::new();
    let mut values: Vec<usize> = Vec::new();

    let mut next = parse_node(&input, &mut stack, &mut meta, &mut values);
    while next.len() > 0 {
        next = parse_node(&next, &mut stack, &mut meta, &mut values);
        next = handle_stack(&next, &mut stack, &mut meta, &mut values);
    }
    (meta, values[0])
}
pub fn run1() -> usize {
    let input = read_file(INPUT);

    let parsed = parse_input(&input);

    let (metadata, _value) = parse3(&parsed);

    metadata.iter().sum()
}

pub fn run2() -> usize {
    let input = read_file(INPUT);

    let parsed = parse_input(&input);

    let (_metadata, value) = parse3(&parsed);

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test1() {
        let parsed = parse_input(INPUT);

        assert_eq!(parsed.len(), 16);
        assert_eq!(&parsed[..4], &vec![2, 3, 0, 3][..]);
    }

    #[test]
    fn test2() {
        let parsed = parse_input(INPUT);
        println!("{:?}", parsed);

        let (metadata, value) = parse3(&parsed);
        println!("{:?}", metadata);

        let sum: usize = metadata.iter().sum();

        assert_eq!(sum, 138);
        assert_eq!(value, 66);
    }

    // [(2 3) 1 2 0 3 10 11 12 0 2 0 2 7 3 1 1 2]
    // [(2 3) [(1 2) [(0 3) 10 11 12] [(0 2) 0 2] 7 3] 1 1 2]

    #[test]
    fn test3() {
        let parsed = parse_input("2 3 1 2 0 3 10 11 12 0 2 0 2 7 3 1 1 2");

        println!("{:?}", parsed);

        let (metadata, value) = parse3(&parsed);

        println!("metadata={:?}", metadata);

        let sum: usize = metadata.iter().sum();

        assert_eq!(sum, 49);
        assert_eq!(value, 10);
    }

    #[test]
    fn test4() {
        let parsed = parse_input("2 3 1 2 0 2 8 9 1 2 0 4 3 4 5 7 1 1 2");

        println!("{:?}", parsed);

        let (metadata, value) = parse3(&parsed);

        println!("metadata={:?}", metadata);

        let sum: usize = metadata.iter().sum();

        assert_eq!(sum, 43);
        assert_eq!(value, 53);
    }

}
