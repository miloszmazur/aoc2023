use std::collections::HashMap;

use regex::Regex;
fn main() {
    println!("boat races!");
    let input = include_str!("../../inputs/input-mati.txt");
    dbg!(part1(input.to_string()));
}

fn part1(input: String) -> usize {
    let (instruction, graph) = parse_graph(&input);
    let instruction: Vec<char> = instruction.chars().collect();

    let mut idx = 0usize;
    let mut current_vertex: &str = "AAA";
    while current_vertex != "ZZZ" {
       let way : char= instruction[idx % instruction.len()];
       let new_curr = &graph[current_vertex];
        current_vertex = match way {
            'L' => &new_curr.0,
            'R' => &new_curr.1,
            _ => panic!("quo vadis?")
        };
        idx += 1;
    }
    idx
}

fn parse_graph(input: &str) -> (String, HashMap<String, (String, String)>) {
    let (instrution, input) = input.split_once("\n\n").unwrap();
    let vertices: HashMap<String, (String, String)> = input.lines().filter_map(|line| {
        println!("{}", &line);
        extract_vertex(line)
    }).collect();
    dbg!((instrution, &vertices));
    (instrution.to_string(), vertices)
}

fn extract_vertex(line: &str) -> Option<(String, (String, String))> {
    let re = Regex::new(r"^(?<parent>[A-Z]{3}) = \((?<c1>[A-Z]{3}), (?<c2>[A-Z]{3})\)$").unwrap();
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
            RL\n\
            \n\
            AAA = (BBB, CCC)\n\
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input.to_string()), 2);
    }

    #[test]
    fn longer_example_works() {
        let input = "\
            LLR\n\
            \n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)\n\
           ";
        assert_eq!(part1(input.to_string()), 6);
    }
}
