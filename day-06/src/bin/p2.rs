#[derive(Debug)]
struct Race {
    time: u128,
    distance: u128,
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

    fn possible_wins(&self) -> u128 {
        let (x1, x2) = self.zeros();
        let possibilities = dbg!(x2.ceil() - x1.floor()) as u128 - 1;
        dbg!(possibilities)
    }
}

fn main() {
    println!("boat races!");
    let input = include_str!("../../inputs/input.txt");
    dbg!(part2(input.to_string()));
}

fn part2(input: String) -> u128 {
    let race = dbg!(parse_race(&input));
    race.possible_wins()
}

fn parse_race(input: &str) -> Race {
    let lines: Vec<u128> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .collect::<Vec<&str>>()
                .join("")
                .parse()
                .unwrap()
        })
        .collect();
    if let [time, distance] = &lines[..] {
        return Race {
            time: *time,
            distance: *distance,
        };
    } else {
        panic!(":(");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part2_works() {
        let input = "\
            Time:      7  15   30\n\
            Distance:  9  40  200\n\
           ";
        assert_eq!(part2(input.to_string()), 71503);
    }
}
