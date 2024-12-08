use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

fn main() {
    let input = include_str!("../../inputs/input-mati.txt");
    dbg!(part1(input.to_string()));
}

type Point = (isize, isize);
type Maze = Vec<Vec<char>>;

fn part1(input: String) -> i32 {
    let maze: Maze = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut distances: Vec<Vec<i32>> = maze
        .iter()
        .map(|f| vec![-1i32; f.len().try_into().unwrap()])
        .collect();
    fill_distances(&maze, &mut distances);
    print_distances(&distances);
    distances
        .iter()
        .map(|f| *f.iter().max().unwrap())
        .max()
        .unwrap()
}

fn print_distances(p0: &Vec<Vec<i32>>) {
    let mut file = File::create("foo.txt").unwrap();
    for i in p0 {
        file.write(b"x").unwrap();
        for j in i {
            file.write(format!("{:4}", j).as_bytes()).unwrap();
        }
        file.write(b"x\n").unwrap();
    }
}

fn fill_distances(maze: &Maze, distances: &mut Vec<Vec<i32>>) {
    let start_position = find_start(maze);
    distances[start_position.1 as usize][start_position.0 as usize] = 0;
    let mut stack: VecDeque<(Point, i32)> = VecDeque::new();
    stack.extend(possible_starts(&maze, start_position));
    while !stack.is_empty() {
        let (current_point, distance) = stack.pop_front().unwrap();
        if current_point.0 < 0 || current_point.1 < 0 {
            continue;
        }
        let x = current_point.0 as usize;
        let y = current_point.1 as usize;
        if y >= distances.len() || x >= distances[y].len() {
            continue;
        }

        if distances[y][x] == -1 || distances[y][x] > distance + 1 {
            distances[y][x] = distance + 1;
            stack.extend(get_possible_connections(maze, current_point, distance + 1))
        } else {
            continue;
        }
    }
}

fn find_start(maze: &Maze) -> Point {
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                return (j as isize, i as isize);
            }
        }
    }
    panic!("aaaAAa no S")
}
fn possible_starts(maze: &Maze, point: Point) -> Vec<(Point, i32)> {
    let x = point.0 as usize;
    let y = point.1 as usize;
    let up = ['|', '7', 'F'];
    let down = ['|', 'L', 'J'];
    let right = ['-', 'J', '7'];
    let left = ['-', 'F', 'L'];
    let mut possible_options = vec![];
    if up.contains(&maze[y+1][x]) {
        possible_options.push(((point.0, point.1-1), 0));
    }
    if down.contains(&maze[y-1][x]) {
        possible_options.push(((point.0, point.1+1), 0));
    }
    if left.contains(&maze[y][x-1]) {
        possible_options.push(((point.0-1, point.1), 0));
    }
    if right.contains(&maze[y][x+1]) {
        possible_options.push(((point.0+1, point.1), 0));
    }
    possible_options
}

fn get_possible_connections(maze: &Maze, point: Point, distance: i32) -> Vec<(Point, i32)> {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is a starting position
    // -S---J
    let current_char = maze[point.1 as usize][point.0  as usize];
    match current_char {
        '|' => vec![
            ((point.0, point.1 + 1), distance),
            ((point.0, point.1 - 1), distance),
        ],
        '-' => vec![
            ((point.0 + 1, point.1), distance),
            ((point.0 - 1, point.1), distance),
        ],
        'L' => vec![
            ((point.0 + 1, point.1), distance),
            ((point.0, point.1 - 1), distance),
        ],
        'J' => vec![
            ((point.0 - 1, point.1), distance),
            ((point.0, point.1 - 1), distance),
        ],
        '7' => vec![
            ((point.0 - 1, point.1), distance),
            ((point.0, point.1 + 1), distance),
        ],
        'F' => vec![
            ((point.0 + 1, point.1), distance),
            ((point.0, point.1 + 1), distance),
        ],
        '.' => vec![],
        other => panic!("aAAaA dunno what's that {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1_works() {
        let input = "\
            .....\n\
            .S-7.\n\
            .|.|.\n\
            .L-J.\n\
            .....\
            ";
        assert_eq!(part1(input.to_string()), 4);
    }
}
