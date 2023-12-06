use std::collections::{HashSet, VecDeque};

fn main() {
    println!("scratchcards!");
    let input = include_str!("../../inputs/input.txt");
    dbg!(part2(input));
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let mut card_stack: VecDeque<usize> = VecDeque::new();
    let lines: Vec<&str> = input.lines().collect();
    for line_count in 0..lines.len() {
        card_stack.push_back(line_count);
    }

    while !card_stack.is_empty() {
        sum += 1;
        let line_idx = card_stack.pop_back().unwrap();
        let wins = process_line(&lines[line_idx]);
        // let card_id = line_idx + 1;
        // println!("card {card_id} : {wins} wins");
        println!("stack size: {}", card_stack.len());

        for idx in (line_idx + 1)..(line_idx + wins + 1) {
            // let id = idx + 1;
            // println!("pushing front card {id}");
            card_stack.push_front(idx);
        }
    }
    sum
}

fn process_line(line: &str) -> usize {
    let (_, nums) = line.split_once(": ").expect("failed to parse game string!");
    let (winning, mine) = nums
        .split_once(" | ")
        .expect("failed to parse game string again");
    let winning = parse_to_set(winning);
    let mine = parse_to_set(mine);
    let lfg = mine.intersection(&winning);
    let winning_numbers = lfg.count();
    winning_numbers
}

fn parse_to_set(string: &str) -> HashSet<i32> {
    let mut v = HashSet::new();
    for chunk in string.split_whitespace() {
        v.insert(chunk.parse().expect("oops, {chunk} is not an i32"));
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2_works() {
        let input = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
           ";
        assert_eq!(part2(input), 30);
    }
}
