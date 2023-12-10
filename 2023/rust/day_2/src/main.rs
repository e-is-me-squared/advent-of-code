use std::fs;

#[derive(Debug)]
struct DiceCollection {
    blue: u32,
    green: u32,
    red: u32,
}

impl DiceCollection {
    fn new(data: String) -> DiceCollection {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        let split_data = data.split(",");
        for data in split_data {
            if data.contains("blue") {
                blue = DiceCollection::convert_to_number(data);
            } else if data.contains("green") {
                green = DiceCollection::convert_to_number(data);
            } else if data.contains("red") {
                red = DiceCollection::convert_to_number(data);
            }
        }
        DiceCollection { blue, green, red }
    }

    fn convert_to_number(num: &str) -> u32 {
        let red_str = num.split(" ").nth(1).unwrap();
        red_str.parse::<u32>().unwrap()
    }
}

#[derive(Debug)]
struct Game {
    uid: u32,
    throws: Vec<DiceCollection>,
}

impl Game {
    fn new(data: String) -> Game {
        if data == "" {
            panic!("Empty data");
        }

        let mut throws: Vec<DiceCollection> = Vec::new();
        let uid_data_split = data.split(":").nth(0).unwrap();
        let uid = uid_data_split
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let data = data.split(":").nth(1).unwrap();

        data.split(";").for_each(|data| {
            let dice_throw = DiceCollection::new(data.to_string());
            throws.push(dice_throw);
        });

        Game { uid, throws }
    }

    pub fn lowest_possible(&self) -> DiceCollection {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;

        for throw in &self.throws {
            if throw.blue > blue {
                blue = throw.blue;
            }
            if throw.green > green {
                green = throw.green;
            }
            if throw.red > red {
                red = throw.red;
            }
        }

        DiceCollection { blue, green, red }
    }

    pub fn is_possible(&self, dice_throw: &DiceCollection) -> bool {
        for throw in &self.throws {
            if throw.blue > dice_throw.blue
                || throw.green > dice_throw.green
                || throw.red > dice_throw.red
            {
                return false;
            }
        }
        true
    }
}

fn parse_data(data: &String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    let lines = data.split("\n");
    for line in lines {
        if line == "" {
            continue;
        }
        let game = Game::new(line.to_string());
        games.push(game);
    }
    games
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");
    let games = parse_data(&data);
    let rules = DiceCollection {
        blue: 14,
        green: 13,
        red: 12,
    };

    let result = part_one(&games, &rules);
    println!("{:?}", result);

    let result = part_two(games);
    println!("{:?}", result);
}

fn part_one(games: &Vec<Game>, rules: &DiceCollection) -> u32 {
    let mut combined_uid: u32 = 0;
    for game in games {
        if game.is_possible(&rules) {
            combined_uid += game.uid;
        }
    }

    combined_uid
}

fn part_two(games: Vec<Game>) -> u32 {
    let mut result: u32 = 0;
    for game in games {
        let lowest_possible = game.lowest_possible();
        let game_score = lowest_possible.blue * lowest_possible.green * lowest_possible.red;
        result += game_score;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();

        let games = parse_data(&data);
        let rules = DiceCollection {
            blue: 14,
            green: 13,
            red: 12,
        };

        let result = part_one(&games, &rules);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_two() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string();

        let games = parse_data(&data);
        let result = part_two(games);

        assert_eq!(result, 2286);
    }
}
