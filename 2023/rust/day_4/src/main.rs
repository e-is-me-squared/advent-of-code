use std::fs;

#[derive(Debug)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn from_data(data: &String) -> Card {
        let mut parts = data.split(":");

        let id = parts.next().unwrap().to_string(); // first part of the data containing the `Card {id}`
        let id = Card::trim_extra_whitespaces(&id); // remove extra whitespaces `Card    { id }` -> `Card { id }`
        let id = id.split(' ').nth(1).unwrap().parse::<u32>().unwrap(); // extract the id from the string and parse it to u32

        let winners_and_numbers = parts.next().unwrap().to_string(); // second part of the data containing the `winners` and `numbers`
        let winners_and_numbers = Card::trim_extra_whitespaces(&winners_and_numbers); // remove extra whitespaces
        let mut winners_and_numbers = winners_and_numbers.split("|"); // split the winners and numbers into two parts

        let winners = winners_and_numbers.next().unwrap().trim().split(" "); // extract the first part: winners
        let winners = winners
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>(); // parse the winners to u32

        let numbers = winners_and_numbers.next().unwrap().trim().split(" "); // extract the second part: numbers
        let numbers = numbers
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>(); // parse the numbers to u32

        Card {
            id,
            numbers,
            winners,
        }
    }

    // pub fn get_num_of_winners(&self) -> u32 {
    //     let mut winners: u32 = 0;
    //
    //     for winner in &self.winners {
    //         if self.numbers.contains(&winner) {
    //             winners += 1;
    //         }
    //     }
    //     winners
    // }

    pub fn get_points(&self) -> u32 {
        let mut winners: u32 = 0;

        for winner in &self.winners {
            if self.numbers.contains(&winner) {
                winners += 1;
            }
        }

        if winners == 0 {
            return 0;
        }
        2u32.pow(winners - 1)
    }

    fn trim_extra_whitespaces(data: &String) -> String {
        data.split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

struct CardCollection {
    cards: Vec<Card>,
}

impl CardCollection {
    fn from_data(data: &String) -> CardCollection {
        let mut cards: Vec<Card> = Vec::new();

        for line in data.lines() {
            cards.push(Card::from_data(&line.to_string()));
        }

        CardCollection { cards }
    }

    fn get_points(&self) -> u32 {
        let mut points: u32 = 0;
        for card in &self.cards {
            points += card.get_points();
        }
        points
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");

    part_one(&data);
}

fn part_one(data: &String) -> u32 {
    let collection = CardCollection::from_data(data);
    let points = collection.get_points();
    println!("Points: {:?}", points);

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string();

        let result = part_one(&data);

        assert_eq!(result, 13);
    }
}
