fn main() {
    println!("gear ratios!");
    let input = include_str!("../../inputs/input.txt");
    dbg!(part1(input.to_string()));
}

fn part1(input: String) -> u32 {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for (row, line) in lines.iter().enumerate() {
        let mut num_start: i32 = -1;
        let mut num_end: i32 = -1;
        let mut last_number: u32 = 0;
        for (column, symbol) in line.chars().enumerate() {
            if symbol.is_numeric() {
                if num_start == -1 {
                    num_start = column as i32;
                    num_end = column as i32;
                    last_number = symbol.to_digit(10).unwrap();
                } else {
                    last_number = last_number * 10 + symbol.to_digit(10).unwrap();
                    num_end = column as i32;
                }
                if column + 1 == line.len() {
                    if num_start != -1 {
                        let is_part = is_part_num(&lines, row, num_start, num_end);
                        if is_part {
                            sum += last_number;
                        }
                    }
                    num_start = -1;
                    num_end = -1;
                    last_number = 0;
                }
            } else {
                if num_start != -1 {
                    let is_part = is_part_num(&lines, row, num_start, num_end);
                    if is_part {
                        sum += last_number;
                    }
                }
                num_start = -1;
                num_end = -1;
                last_number = 0;
            }
        }
    }
    sum
}

fn is_part_num(lines: &[&str], row: usize, num_start: i32, num_end: i32) -> bool {
    let width = lines[0].len();
    let height = lines.len();
    let non_part_symbols = "1234567890.";

    // above
    for idx in num_start - 1..=num_end + 1 {
        if idx < 0 || idx as usize >= width || (row as i32) - 1 < 0 {
            continue;
        }
        let characters: Vec<char> = lines[row - 1].chars().collect();
        let character: &char = characters.get(idx as usize).unwrap();
        if !non_part_symbols.contains(*character) {
            return true;
        }
    }

    // below
    for idx in num_start - 1..=num_end + 1 {
        if idx < 0 || idx as usize >= width || row + 1 >= height {
            continue;
        }
        let characters: Vec<char> = lines[row + 1].chars().collect();
        let character: &char = characters.get(idx as usize).unwrap();
        if !non_part_symbols.contains(*character) {
            return true;
        }
    }

    // next-to
    let row_chars: Vec<char> = lines[row].chars().collect();
    if num_start - 1 > 0
        && !non_part_symbols.contains(*row_chars.get((num_start - 1) as usize).unwrap())
    {
        return true;
    }
    if num_end + 1 < width as i32
        && !non_part_symbols.contains(*row_chars.get((num_end + 1) as usize).unwrap())
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let input = "\
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..\n\
            .$........\n\
            ........&3\n\
            ..........\
           ";
        assert_eq!(part1(input.to_string()), 4364);
    }
}
