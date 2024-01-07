use std::fs;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Race {
        Race { time, distance }
    }
    fn num_of_wins(&self) -> u64 {
        let mut records = 0;
        for hold_time in 0..self.time {
            let travel_time = self.time - hold_time;
            let distance = travel_time * hold_time;
            if distance > self.distance {
                records += 1;
            }
        }
        records
    }
}

#[derive(Debug)]
struct Track {
    races: Vec<Race>,
}

impl Track {
    fn new(data: String) -> Track {
        let mut lines = data.lines();
        let times = lines.next().unwrap().split(':').skip(1).next().unwrap();
        let times = times.split_whitespace().map(|x| x.parse::<u64>().unwrap());

        let distances = lines.next().unwrap().split(':').skip(1).next().unwrap();
        let distances = distances
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap());

        let mut races = vec![];
        for (time, distance) in times.zip(distances) {
            races.push(Race::new(time, distance));
        }

        Track { races }
    }

    fn get_num_of_wins(&self) -> Vec<u64> {
        self.races
            .iter()
            .map(|x| x.num_of_wins())
            .collect::<Vec<u64>>()
    }

    fn get_total_num_of_wins(&self) -> u64 {
        self.get_num_of_wins()
            .iter()
            .copied()
            .reduce(|a, b| a * b)
            .unwrap()
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

fn part_one(data: &String) -> u64 {
    let race = Track::new(data.to_string());
    race.get_total_num_of_wins()
}

fn part_two(data: &String) -> u64 {
    let data = data.replace(" ", "");
    let race = Track::new(data.to_string());
    race.get_total_num_of_wins()
}

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

    #[test]
    fn test_part_two() {
        let result = part_two(&TEST_DATA.to_string());
        assert_eq!(result, 71503);
    }
}
