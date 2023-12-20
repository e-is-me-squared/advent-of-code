use std::fs;

struct ConvertInfo {
    destination: u32,
    source: u32,
    range: u32,
}

struct Converter {
    values: Vec<ConvertInfo>,
}

impl Converter {
    fn new() -> Converter {
        Converter {
            values: Vec::new(),
        }
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
    0
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
