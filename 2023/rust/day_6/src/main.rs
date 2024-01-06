use std::fs;

/**
* NOTE: Puzzle
*/

fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");

    let result = part_one(&data);
    println!("Part one: {:?}", result);

    // let result = part_two(&data);
    // println!("Part two: {:?}", result);
}

fn part_one(data: &String) -> u64 {
    0
}

// fn part_two(data: &String) -> u64 {
//     0
// }

/**
* NOTE: Tests
*/

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        let result = part_one(&TEST_DATA.to_string());

        assert_eq!(result, 288);
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&TEST_DATA.to_string());
    //
    //     assert_eq!(result, 46);
    // }
}
