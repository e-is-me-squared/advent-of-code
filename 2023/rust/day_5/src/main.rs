use std::fs;

struct ConvertInfo {
    destination: u32,
    source: u32,
    range: u32,
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
            let mut parse_next_number = || line.next().unwrap().parse::<u32>().unwrap();

            ConvertInfo {
                destination: parse_next_number(),
                source: parse_next_number(),
                range: parse_next_number(),
            }
        });

        Converter {
            from,
            to,
            values: data.collect(),
        }
    }

    fn convert(&self, value: u32) -> u32 {
        let mut result = value;
        for info in &self.values {
            if value >= info.source && value < info.source + info.range {
                result = info.destination + (value - info.source);
            }
        }
        result
    }
}



fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");

    let result = part_one(&data);
    println!("Part one: {:?}", result);

    // let result = part_two(&data);
    // println!("Part two: {:?}", result);
}

fn part_one(data: &String) -> u32 {
    let converter = Converter::new("".to_string());
    let converted = converter.convert(0);
    converted
}

// fn part_two(data: &String) -> u32 {
//     let mut collection = CardCollection::from_data(data);
//
//     collection.fill_with_bonus_cards();
//     collection.get_bonus_points()
// }

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

        assert_eq!(converter.values[0].destination, 50);
        assert_eq!(converter.values[0].source, 98);
        assert_eq!(converter.values[0].range, 2);

        assert_eq!(converter.values[1].destination, 52);
        assert_eq!(converter.values[1].source, 50);
        assert_eq!(converter.values[1].range, 48);

        assert_eq!(converter.convert(79), 81);
        assert_eq!(converter.convert(14), 14);
        assert_eq!(converter.convert(55), 57);
        assert_eq!(converter.convert(13), 13);
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

    //     #[test]
    //     fn test_part_two() {
    //     }
}
