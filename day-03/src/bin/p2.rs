use itertools::Itertools;

#[derive(Debug)]
struct StarEnginePart {
    number: i32,
    star_coords: String,
}
fn main() {
    println!("gear ratios!");
    let input = include_str!("../../inputs/input.txt");
    dbg!(part2(input.to_string()));
}

fn part2(input: String) -> i32 {
    let star_parts: Vec<StarEnginePart> = get_all_parts(input);
    let sum: i32 = star_parts
        .iter()
        .into_group_map_by(|coord| coord.star_coords.clone())
        .iter()
        .filter_map(|(key, group)| {
            println!("{key:?}");
            if group.len() == 2 {
                let product: i32 = group[0].number * group[1].number;
                Some(product)
            } else {
                None
            }
        })
        .sum();
    sum
}

fn get_all_parts(input: String) -> Vec<StarEnginePart> {
    let mut all_parts: Vec<StarEnginePart> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    print!("lines: {}", lines.len());
    for (row, line) in lines.iter().enumerate() {
        let mut num_start: i32 = -1;
        let mut num_end: i32 = -1;
        let mut last_number: u32 = 0;
        let line_chars: Vec<char> = line.chars().collect();

        for column in 0..line_chars.len() + 1 {
            if column < line_chars.len() && line_chars[column].is_numeric() {
                let symbol = line_chars[column];
                if num_start == -1 {
                    num_start = column as i32;
                    num_end = column as i32;
                    last_number = symbol.to_digit(10).unwrap() as u32;
                } else {
                    last_number = last_number * 10 + symbol.to_digit(10).unwrap() as u32;
                    num_end = column as i32;
                }
            } else {
                if num_start != -1 {
                    let mut parts =
                        get_star_coordinates(&lines, row, num_start, num_end, last_number);
                    // dbg!(column, row, num_start, num_end);
                    all_parts.append(&mut parts);
                    print!("{parts:?}")
                }
                num_start = -1;
                num_end = -1;
                last_number = 0;
            }
        }
    }
    all_parts
}

fn get_star_coordinates(
    lines: &[&str],
    row: usize,
    num_start: i32,
    num_end: i32,
    part_number: u32,
) -> Vec<StarEnginePart> {
    let width = lines[0].len();
    let height = lines.len();
    // let non_part_symbols = "1234567890.";
    let mut parts: Vec<StarEnginePart> = Vec::new();

    // above
    for idx in num_start - 1..=num_end + 1 {
        if idx < 0 || idx as usize >= width || (row as i32) - 1 < 0 {
            continue;
        }
        let characters: Vec<char> = lines[row - 1].chars().collect();
        let character: &char = characters.get(idx as usize).unwrap();
        if *character == '*' {
            parts.push(StarEnginePart {
                number: part_number as i32,
                star_coords: format!("{}x{}",idx, row as i32 - 1),
            });
        }
    }

    // below
    for idx in num_start - 1..=num_end + 1 {
        if idx < 0 || idx as usize >= width || row + 1 >= height {
            continue;
        }
        let characters: Vec<char> = lines[row + 1].chars().collect();
        let character: &char = characters.get(idx as usize).unwrap();
        if *character == '*' {
            parts.push(StarEnginePart {
                number: part_number as i32,
                star_coords: format!("{}x{}", idx, row as i32 + 1),
            });
        }
    }

    // next-to
    let row_chars: Vec<char> = lines[row].chars().collect();
    if num_start - 1 > 0 && *row_chars.get((num_start - 1) as usize).unwrap() == '*' {
        parts.push(StarEnginePart {
            number: part_number as i32,
            star_coords: format!("{}x{}", num_start - 1, row),
        });
    }
    if num_end + 1 < width as i32 && *row_chars.get((num_end + 1) as usize).unwrap() == '*' {
        parts.push(StarEnginePart {
            number: part_number as i32,
            star_coords: format!("{}x{}", num_end + 1, row),
        });
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2_works() {
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
        ..........\n\
        2.........\n\
        .*........\n\
        ..2.......\n\
        .*.......2\n\
        .*......*.\n\
        ..2......2\
        ";

        assert_eq!(part2(input.to_string()), 467835 + 4 + 4);
    }
}
