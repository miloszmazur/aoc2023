const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn main() {
    println!("trebuchet!");
    let input = include_str!("../../inputs/01-mati.txt");
    dbg!(part2(input));
}

fn part2(input: &str) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        let line_numbers = parse_line(line);
        let first = *line_numbers.first().unwrap();
        let last = *line_numbers.last().unwrap();
        numbers.push(first * 10 + last);
    }
    numbers.iter().sum::<i32>()
}

fn parse_line(line: &str) -> Vec<i32> {
    let mut line_numbers = Vec::new();
    for (index, character) in line.chars().enumerate() {
        match character.to_string().parse::<i32>() {
            Ok(num) => line_numbers.push(num),
            Err(_) => {
                // see if substring from this char to end of string starts with one of the numbers
                let substr = &line[index..];
                for (num_idx, number) in NUMBERS.iter().enumerate() {
                    if substr.starts_with(number) {
                        line_numbers.push(num_idx as i32);
                        break;
                    }
                }
            }
        }
    }
    line_numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2_works() {
        let input = "\
        two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen\n";
        assert_eq!(part2(input), 281);
    }
}
