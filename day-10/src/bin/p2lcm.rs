use std::collections::HashMap;

use regex::Regex;
fn main() {
    println!("boat races!");
    let input = include_str!("../../inputs/input-mati.txt");
    dbg!(part2(input.to_string()));
}

fn part2(input: String) -> usize {
    let (instruction, graph) = parse_graph(&input);
    let instruction: Vec<char> = instruction.chars().collect();
    let instruction_count = instruction.len();
    let cycles: Vec<usize> = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|element| {
            let mut current_element = element;
            let mut idx = 0usize;

            while !current_element.ends_with('Z') {
                let new_curr = &graph[current_element];
                let way: char = instruction[(idx % instruction_count) as usize];
                let new_current_vertex = match way {
                    'L' => &new_curr.0,
                    'R' => &new_curr.1,
                    _ => panic!("quo vadis?"),
                };
                current_element = new_current_vertex;
                idx += 1;
            }
            idx
        })
        .collect();
    lcm2(&cycles)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}


fn lcm2(numbers: &Vec<usize>) -> usize {
    let mut lcm = numbers[0];
    for index in 1..numbers.len() {
        let current = numbers[index];
        lcm = (lcm * current) / gcd(lcm, current);
    }
    lcm
}

fn parse_graph(input: &str) -> (String, HashMap<String, (String, String)>) {
    let (instrution, input) = input.split_once("\n\n").unwrap();
    let vertices: HashMap<String, (String, String)> = input
        .lines()
        .filter_map(|line| {
            println!("{}", &line);
            extract_vertex(line)
        })
        .collect();
    dbg!((instrution, &vertices));
    (instrution.to_string(), vertices)
}

fn extract_vertex(line: &str) -> Option<(String, (String, String))> {
    let re = Regex::new(r"^(?<parent>[0-9A-Z]{3}) = \((?<c1>[0-9A-Z]{3}), (?<c2>[0-9A-Z]{3})\)$")
        .unwrap();
    let cap = re.captures(line)?;
    let parent = cap.name("parent")?.as_str();
    let c1 = cap.name("c1")?.as_str();
    let c2 = cap.name("c2")?.as_str();
    Some((parent.to_string(), (c1.to_string(), c2.to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_example_works() {
        let input = "\
            LR\n\
            \n\
            11A = (11B, XXX)\n\
            11B = (XXX, 11Z)\n\
            11Z = (11B, XXX)\n\
            22A = (22B, XXX)\n\
            22B = (22C, 22C)\n\
            22C = (22Z, 22Z)\n\
            22Z = (22B, 22B)\n\
            XXX = (XXX, XXX)";
        assert_eq!(part2(input.to_string()), 6);
    }
}
