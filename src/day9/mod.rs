use std::collections::VecDeque;

static PLAYERS: usize = 491;
static LAST_MARBLE: usize = 71058;

fn insert(ring: &mut VecDeque<usize>, e: usize) {
    ring.push_back(e);
    let f = ring.pop_front().unwrap();
    ring.push_back(f);
}

fn remove(ring: &mut VecDeque<usize>) -> usize {
    for _i in 0..8 {
        let p = ring.pop_back().unwrap();
        ring.push_front(p);
    }
    let e = ring.pop_back().unwrap();
    for _i in 0..2 {
        let i = ring.pop_front().unwrap();
        ring.push_back(i);
    }
    e
}

fn play(num_players: usize, last_marble_worth: usize) -> Vec<usize> {
    let mut ring = VecDeque::<usize>::new();
    let mut player = 1;
    let mut score = vec![0; num_players];

    ring.push_front(0);

    for marble_to_place in 1..=last_marble_worth {
        if marble_to_place % 23 == 0 {
            score[player - 1] += marble_to_place + remove(&mut ring);
        } else {
            insert(&mut ring, marble_to_place);
        }
        player = player % num_players;
        player += 1;
    }
    score
}

pub fn run1() -> usize {
    *play(PLAYERS, LAST_MARBLE).iter().max().unwrap()
}

pub fn run2() -> usize {
    *play(PLAYERS, LAST_MARBLE * 100).iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut ring = VecDeque::<usize>::new();
        ring.push_front(0);

        insert(&mut ring, 1);
        insert(&mut ring, 2);
        insert(&mut ring, 3);
        insert(&mut ring, 4);

        assert_eq!(ring, vec![1, 3, 0, 4, 2])
    }

    #[test]
    fn test2() {
        let mut a1 = VecDeque::from(vec![
            1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11,
        ]);

        let mut a2 = VecDeque::from(vec![
            20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0, 16, 8, 17, 4, 18, 19, 2,
        ]);

        let e = remove(&mut a1);

        println!("{:?}", a1);
        println!("{:?}", e);

        assert_eq!(a1, a2);
        assert_eq!(e, 9);
    }

    #[test]
    fn test4() {
        let score = play(9, 25);

        let max_score = *score.iter().max().unwrap();
        assert_eq!(max_score, 32)
    }

    #[test]
    fn test5() {
        let score = play(10, 1618);

        println!("{:?}", score);

        let max_score = *score.iter().max().unwrap();
        assert_eq!(max_score, 8317)
    }

    #[test]
    fn test6() {
        let score = play(13, 7999);

        println!("{:?}", score);

        let max_score = *score.iter().max().unwrap();
        assert_eq!(max_score, 146373)
    }

    #[test]
    fn test7() {
        let score = play(17, 1104);

        println!("{:?}", score);

        let max_score = *score.iter().max().unwrap();
        assert_eq!(max_score, 2764)
    }

    #[test]
    fn test8() {
        let score = play(21, 6111);

        println!("{:?}", score);

        let max_score = *score.iter().max().unwrap();
        assert_eq!(max_score, 54718)
    }

    #[test]
    fn test9() {
        let score = play(30, 5807);

        println!("{:?}", score);

        let max_score = *score.iter().max().unwrap();
        assert_eq!(max_score, 37305)
    }

}
