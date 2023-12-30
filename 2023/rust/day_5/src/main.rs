use std::fs;

struct ConvertInfo {
    destination: u64,
    source: u64,
    range: u64,
}

struct Converter {
    from: String,
    to: String,
    values: Vec<ConvertInfo>,
}

impl Converter {
    fn new(data: String) -> Converter {
        let mut words = data.split_whitespace();
        let first_word = words.next().unwrap();
        let mut first_word = first_word.split("-");

        let from = first_word.next().unwrap().to_string();
        first_word.next();
        let to = first_word.next().unwrap().to_string();

        let mut data = data.split(":");
        data.next();
        let mut data = data.next().unwrap().lines();
        data.next();

        let data = data.map(|line| {
            let mut line = line.split_whitespace();
            let mut parse_next_number = || line.next().unwrap().parse::<u64>().unwrap();

            ConvertInfo {
                destination: parse_next_number(),
                source: parse_next_number(),
                range: parse_next_number(),
            }
        });

        //sort by source
        let mut data = data.collect::<Vec<_>>();
        data.sort_by(|a, b| a.source.cmp(&b.source));

        Converter {
            from,
            to,
            values: data,
        }
    }

    fn find_next_converter_source(&self, seed: u64) -> (bool, u64) {
        let mut first_iteration = true;
        for info in &self.values {
            if first_iteration && seed < info.source {
                return (true, info.source);
            }
            first_iteration = false;
            if seed >= info.source && seed < info.source + info.range {
                return (true, info.source + info.range);
            }
        }
        (false, 0)
    }

    fn convert(&self, value: u64) -> u64 {
        let mut result = value;
        for info in &self.values {
            if value >= info.source && value < info.source + info.range {
                result = info.destination + (value - info.source);
            }
        }
        result
    }
}

struct ProductionPipeline {
    converters: Vec<Converter>,
}

impl ProductionPipeline {
    fn new(data: String) -> ProductionPipeline {
        let mut converters = vec![];
        let mut looped = "".to_string();

        let mut add_converter = |data: String| {
            let converter = Converter::new(data);
            converters.push(converter);
        };

        data.lines().for_each(|line| {
            if line == "" {
                add_converter(looped.to_string());
                looped = "".to_string();
            } else {
                looped.push_str(line);
                looped.push_str("\n");
            }
        });

        add_converter(looped.to_string());

        ProductionPipeline { converters }
    }

    fn convert(&self, seed: u64) -> u64 {
        let mut result = seed;
        for converter in &self.converters {
            result = converter.convert(result);
        }
        result
    }
}

/**
* NOTE: Puzzle
*/

fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");

    let result = part_one(&data);
    println!("Part one: {:?}", result);

    let result = part_two(&data);
    println!("Part two: {:?}", result);
}

fn get_seeds(data: &String) -> Vec<u64> {
    data.lines()
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn part_one(data: &String) -> u64 {
    let seeds = get_seeds(data);
    let data = data.lines().skip(2).collect::<Vec<_>>().join("\n");
    let production = ProductionPipeline::new(data.to_string());
    let mut lowest_converted = u64::max_value();

    seeds.iter().for_each(|seed| {
        let result = production.convert(*seed);
        if result < lowest_converted {
            lowest_converted = result;
        }
    });
    lowest_converted
}

fn part_two(data: &String) -> u64 {
    let seeds = get_seeds(data);

    let data = data.lines().skip(2).collect::<Vec<_>>().join("\n");
    let production = ProductionPipeline::new(data.to_string());
    let mut lowest_converted = u64::max_value();

    // TODO: This bruteforce method works with the test data but is way too slow for the actual data,
    // need to find a better way
    for i in 0..seeds.len() {
        if i % 2 == 1 {
            continue;
        }
        println!("{} / {}", i, seeds.len());

        for j in 0..seeds[i + 1] {
            let seed = seeds[i] + j;
            let result = production.convert(seed);
            if result < lowest_converted {
                lowest_converted = result;
            }
        }
    }

    lowest_converted
}

/**
* NOTE: Tests
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter() {
        let data = "seed-to-soil map:
50 98 2
52 50 48
";

        let converter = Converter::new(data.to_string());

        assert_eq!(converter.from, "seed");
        assert_eq!(converter.to, "soil");
        assert_eq!(converter.values.len(), 2);

        assert_eq!(converter.values[1].destination, 50);
        assert_eq!(converter.values[1].source, 98);
        assert_eq!(converter.values[1].range, 2);

        assert_eq!(converter.values[0].destination, 52);
        assert_eq!(converter.values[0].source, 50);
        assert_eq!(converter.values[0].range, 48);

        assert_eq!(converter.convert(79), 81);
        assert_eq!(converter.convert(98), 50);
        assert_eq!(converter.convert(14), 14);
        assert_eq!(converter.convert(55), 57);
        assert_eq!(converter.convert(13), 13);

        assert_eq!(converter.find_next_converter_source(3), (true, 50));
        assert_eq!(converter.find_next_converter_source(51), (true, 98));
        assert_eq!(converter.find_next_converter_source(101), (false, 0));
    }

    #[test]
    fn test_part_one() {
        let data = "seeds: 79 14 55 13

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
56 93 4"
            .to_string();

        let result = part_one(&data);

        assert_eq!(result, 35);
    }

    #[test]
    fn test_part_two() {
        let data = "seeds: 79 14 55 13

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
56 93 4"
            .to_string();

        let result = part_two(&data);

        assert_eq!(result, 46);
    }
}
