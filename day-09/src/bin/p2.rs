use std::collections::HashMap;

use regex::Regex;
fn main() {
    println!("boat races!");
    let input = include_str!("../../inputs/input-mati.txt");
    dbg!(part2(input.to_string()));
}

fn part2(input: String) -> u128 {
    let (instruction, graph) = parse_graph(&input);
    let instruction: Vec<char> = instruction.chars().collect();
    let mut counter = 0u128;
    let instruction_count = instruction.len() as u128;
    let mut all_currs: Vec<&String> = graph.keys().filter(|k| k.ends_with('A')).collect();

    while !all_currs.iter().all(|f| f.ends_with('Z')) {
        let way: char = instruction[(counter % instruction_count) as usize];
        for index in 0..all_currs.len() {
            let current_position = all_currs[index];
            let vertex = &graph[current_position];
            let new_current_vertex = match way {
                'L' => &vertex.0,
                'R' => &vertex.1,
                _ => panic!("quo vadis?"),
            };
            all_currs[index] = &new_current_vertex;
        }
        counter = counter.checked_add(1).expect("AAAAA OVERFLOW");
        if(counter % 100000000 == 0) {
            println!("counter = {}", counter);
        }
        // dbg!(&counter);
    }
    counter
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
