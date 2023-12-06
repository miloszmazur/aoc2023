use core::panic;

fn main() {
    println!("conundrum!");
    let input = include_str!("../../inputs/input.txt");
    dbg!(part1(input));
}

fn part1(input: &str) -> i32 {
    let sum: i32 = input
        .lines()
        .inspect(|line| {
            dbg!(line);
        })
        .map(|line| {
            let (gid, game) = parse_line(line);
            dbg!(gid, game);
            if game_is_legal(game) {
                gid
            } else {
                0
            }
        })
        .sum();
    sum
}

fn game_is_legal(game: &str) -> bool {
    // constraint: 12 red, 13 green, 14 blue
    let rounds: Vec<&str> = game.split("; ").collect();
    let legal: Vec<bool> = rounds
        .iter()
        .flat_map(|round| {
            let balls: Vec<bool> = round
                .split(", ")
                .map(|ball| {
                    let (num, color) = ball.split_once(" ").expect("oh noes");
                    let num: i32 = num.parse().expect("{num} is NaN");
                    dbg!(num, color);
                    match color {
                        "red" => num <= 12,
                        "green" => num <= 13,
                        "blue" => num <= 14,
                        _ => panic!("aaa"),
                    }
                })
                .collect();
            dbg!(&balls);
            balls
        })
        .collect();
    println!("----");
    legal.iter().all(|x| *x)
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
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
           ";
        assert_eq!(part1(input), 8);
    }
}
