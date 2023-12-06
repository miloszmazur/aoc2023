use core::panic;
use std::cmp;

fn main() {
    println!("conundrum!");
    let input = include_str!("../../inputs/input.txt");
    dbg!(part2(input));
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .inspect(|line| {
            dbg!(line);
        })
        .map(|line| {
            let (gid, game) = parse_line(line);
            dbg!(gid, game);
            minimum_ballz_counter(game)
        })
        .sum()
}

fn minimum_ballz_counter(game: &str) -> i32 {
    let mut green = 0;
    let mut red = 0;
    let mut blue = 0;

    let rounds: Vec<&str> = game.split("; ").collect();

    for round in rounds {
        round.split(", ").for_each(|ball| {
            let (num, color) = ball.split_once(" ").expect("oh noes");
            let num: i32 = num.parse().expect("{num} is NaN");
            match color {
                "red" => red = cmp::max(red, num),
                "green" => green = cmp::max(green, num),
                "blue" => blue = cmp::max(blue, num),
                _ => panic!("aaa"),
            }
        });
    }
    green * red * blue
}

fn parse_line(line: &str) -> (i32, &str) {
    let (gid, game) = line.split_once(": ").expect("failed to parse game string!");
    let (_, gid) = gid.split_once(" ").expect(" failed to parse game id :x");
    let gid: i32 = gid.parse().unwrap();
    (gid, game)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let input = "\
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n\
           ";
        assert_eq!(part2(input), 2286);
    }
}
