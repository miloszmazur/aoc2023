fn main() {
    println!("trebuchet!");
    let input = include_str!("../../inputs/01.txt");
    dbg!(part1(input));
}

fn part1(input: &str) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut line_numbers = Vec::new();
        for character in line.chars() {
            match character.to_string().parse::<i32>() {
                Ok(num) => line_numbers.push(num),
                Err(_) => continue,
            }
        }
        let first = line_numbers.first().unwrap();
        let last = line_numbers.last().unwrap();
        numbers.push(first * 10 + last)
    }
    numbers.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let input = "\
           1abc2\n\
           pqr3stu8vwx\n\
           a1b2c3d4e5f\n\
           treb7uchet";
        assert_eq!(part1(input), 142);
    }
}
