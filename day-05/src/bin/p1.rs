use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/day4/input/input-mati.txt").unwrap();
    let min_location = day4(&input);
    println!("min location = {}", min_location)
}

#[derive(Debug)]
struct MappingRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
struct GardenMapping {
    ranges: Vec<MappingRange>,
}

impl GardenMapping {
    fn transform(&self, input: u64) -> u64 {
        for mapping in &self.ranges {
            let MappingRange {source_start, length, destination_start} = mapping;
            if input >= *source_start && input < *source_start+*length {
                return input - source_start + destination_start;
            }
        }
        return input;
    }
}

fn parse_garden_mapping(input: &str) -> GardenMapping {
    let ranges: Vec<MappingRange> = input.lines()
        .skip(1)
        .map(|line: &str| {
            let a: Vec<u64> = line.split_whitespace().flat_map(|x| x.parse()).collect();
            a
        })
        .filter_map(|numbers: Vec<u64>| {
            match numbers[..] {
                [x, y, z] => {
                    Some(MappingRange { destination_start: x, source_start: y, length: z })
                }
                _ => None
            }
        })
        .collect();
    GardenMapping { ranges }
}

fn parse_seeds(seeds_line: &str) -> Vec<u64> {
    seeds_line.split_whitespace().skip(1).flat_map(|x| x.parse()).collect()
}

fn day4(input: &str) -> u64 {
    let mut groups = input.split("\n\n");
    let seeds: Vec<u64> = (&mut groups).next().map(parse_seeds).unwrap();
    let mappings: Vec<_> = (&mut groups)
        .map(parse_garden_mapping)
        .collect();

    let min_location = seeds.iter()
        .map(|seed| {
            let mut last_transformation: u64 = *seed;
            for mapping in (&mappings).iter() {
                last_transformation = mapping.transform(last_transformation);
            }
            last_transformation
        }).min().unwrap();
    return min_location;
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let example = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let result = super::day4(example);
        assert_eq!(result, 35);
    }
}
