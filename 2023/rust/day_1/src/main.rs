use std::fs;

fn main() {
    let content = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    let result = part_one(&content);
    println!("first Biggest number: {}", result);
    let result = part_two(&content);
    println!("second Biggest number: {}", result);
}

fn part_one(data: &String) -> u32 {
    calculate(data, false)
}

fn part_two(data: &String) -> u32 {
    calculate(data, true)
}

fn calculate(data: &String, check_for_spelled_numbers: bool ) -> u32 {
    let lines = data.split("\n");

    let mut current = 0;

    for line in lines {
        let (first, last) = find_first_and_last_number(&line.to_string(), check_for_spelled_numbers);
        let first_and_last = first + &last;
        let first_and_last: u32 = first_and_last.parse().unwrap();
        current += first_and_last;
    }

    current
}

fn find_first_and_last_number(line: &String, check_for_spelled_numbers: bool) -> (String, String) {
    let mut tub = ("".to_string(), "".to_string());

    // loop through char length
    for i in 0..line.len() {
        let chars = &line[i..].to_string();

        let number = get_number(&chars);
        let number: String = if check_for_spelled_numbers && number == "0" {
            git_spelled_number(&chars)
        } else {
            number
        };

        if number != "0" {
            if tub.0 == "" {
                tub.0 = number.clone();
            }
            tub.1 = number;
        }
    }

    tub.0 = if tub.0 == "" { "0".to_string() } else { tub.0 };
    tub.1 = if tub.1 == "" { "0".to_string() } else { tub.1 };
    tub
}

fn get_number(chars: &String) -> String {
    let all_numbers = "0123456789";
    let mut chars = chars.chars();
    let first_char = chars.next().unwrap();
    if all_numbers.contains(first_char) {
        first_char.to_string()
    } else {
        "0".to_string()
    }
}

fn git_spelled_number(chars: &String) -> String {
    let spelled_numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut index = 0;
    for spelled_number in spelled_numbers {
        if chars.starts_with(spelled_number) {
            return index.to_string();
        }
        index += 1;
    }
    "0".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();
        let result = part_one(&data);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_two() {
        let data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();

        let result = part_two(&data);
        assert_eq!(result, 281);
    }

    #[test]
    fn test_find_first_and_last_number() {
        let (first, last) = find_first_and_last_number(&"abc123".to_string(), false);
        assert_eq!(first, "1");
        assert_eq!(last, "3");
    }

    #[test]
    fn test_find_first_and_last_number_with_no_numbers() {
        let (first, last) = find_first_and_last_number(&"abc".to_string(), false);
        assert_eq!(first, "0");
        assert_eq!(last, "0");
    }
}
