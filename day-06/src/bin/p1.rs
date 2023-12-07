use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

impl Race {
    // distance
    // (race_time - hold_time) * hold_time
    // hold_time*race_time - hold_time^2 - distance > 0
    // hold_time = x
    // a -1
    // race_time = b
    // distance = c

    fn delta(&self) -> f64 {
        (self.time * self.time) as f64 - 4.0 * self.distance as f64
    }

    fn zeros(&self) -> (f64, f64) {
        let delta = self.delta().sqrt();
        let x1 = ((self.time as f64) - delta) / 2.0;
        let x2 = ((self.time as f64) + delta) / 2.0;

        // dbg!(x1, x2);
        (x1, x2)
    }

    fn possible_wins(&self) -> u32 {
        let (x1, x2) = self.zeros();
        let possibilities = dbg!(x2.ceil() - x1.floor()) as u32 - 1;
        dbg!(possibilities)
    }
}

fn main() {
    println!("boat races!");
    let input = include_str!("../../inputs/input.txt");
    dbg!(part1(input.to_string()));
}

fn part1(input: String) -> u32 {
    let races = dbg!(parse_races(&input));
    races.iter().map(|race| race.possible_wins()).product()
}

fn parse_races(input: &str) -> Vec<Race> {
    let lines: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    if let [times, distances] = &lines[..] {
        return zip(times, distances)
            .map(|(time, distance)| Race {
                time: *time,
                distance: *distance,
            })
            .collect();
    } else {
        panic!(":(");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let input = "\
            Time:      7  15   30\n\
            Distance:  9  40  200\n\
           ";
        assert_eq!(part1(input.to_string()), 288);
    }
}
