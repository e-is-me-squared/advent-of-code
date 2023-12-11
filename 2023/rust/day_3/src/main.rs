use std::fs;

const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug, PartialEq)]
enum Type {
    Dot,
    Symbol,
    Gear,
    Number,
}

#[derive(Debug)]
struct Group {
    start_pos: u32,
    end_pos: u32,
    value: String,
    index: u32,
}

impl Group {
    fn get_type(&self) -> Type {
        if self.value.contains(".") {
            return Type::Dot;
        }
        if self.value.contains("*") {
            return Type::Gear;
        }
        if NUMBERS.contains(&self.value.chars().next().unwrap()) {
            return Type::Number;
        }
        Type::Symbol
    }
}

#[derive(Debug)]
struct Row {
    index: u32,
    width: u32,
    parts: Vec<Group>,
}

impl Row {
    fn new(data: String, index: u32) -> Row {
        let mut parts: Vec<Group> = vec![];
        let chars = data.split("");
        let mut width = 0;

        chars.for_each(|x| {
            if x == "" {
                return;
            }
            width += 1;
            if parts.len() > 0 && Row::is_same_group(&parts.last().unwrap().value, x) {
                parts.last_mut().unwrap().end_pos += 1;
                parts.last_mut().unwrap().value.push_str(x);
            } else {
                let last_pos = if parts.len() > 0 {
                    parts.last().unwrap().end_pos + 1
                } else {
                    0
                };

                parts.push(Group {
                    start_pos: last_pos,
                    end_pos: last_pos,
                    value: x.to_string(),
                    index,
                });
            }
        });
        Row {
            parts,
            index,
            width,
        }
    }

    fn get_group_at(&self, col: u32) -> Option<&Group> {
        self.parts
            .iter()
            .find(|x| x.start_pos <= col && x.end_pos >= col)
    }

    fn is_same_group(first: &str, second: &str) -> bool {
        if NUMBERS.contains(&first.chars().next().unwrap())
            && NUMBERS.contains(&second.chars().next().unwrap())
        {
            return true;
        }
        first.contains(&second)
    }
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    pub fn from_data(data: &String) -> Grid {
        let mut rows: Vec<Row> = Vec::new();
        let lines = data.split("\n");
        let mut index = 0;
        for line in lines {
            if line == "" {
                continue;
            }

            let game = Row::new(line.to_string(), index);
            rows.push(game);
            index += 1;
        }

        Grid { rows }
    }

    fn get_groups_of_type(&self, group_type: Type) -> Vec<&Group> {
        let mut groups: Vec<&Group> = vec![];
        for row in &self.rows {
            for group in &row.parts {
                if group.get_type() == group_type {
                    groups.push(group);
                }
            }
        }
        groups
    }

    fn get_surrounding_groups(&self, group: &Group) -> Vec<&Group> {
        let mut groups: Vec<&Group> = vec![];
        let row = self.rows.get(group.index as usize).unwrap();

        let start_row_index = if group.start_pos == 0 {
            0
        } else {
            group.start_pos - 1
        };

        let end_row_index = if group.end_pos + 2 >= row.width {
            row.width
        } else {
            group.end_pos + 2
        };

        // has group above
        if row.index > 0 {
            let top_row = self.rows.get(row.index as usize - 1).unwrap();
            let mut last_value = "".to_string();
            for i in start_row_index..end_row_index {
                let target_group = top_row.get_group_at(i).unwrap();
                if last_value != target_group.value {
                    groups.push(target_group);
                }

                let loop_value = &target_group.value;
                last_value = loop_value.to_string();
            }
        }

        // has group below
        if row.index < (self.rows.len() - 1) as u32 {
            let bottom_row = self.rows.get(row.index as usize + 1).unwrap();
            let mut last_value = "".to_string();
            for i in start_row_index..end_row_index {
                let target_group = bottom_row.get_group_at(i).unwrap();
                if last_value != target_group.value {
                    groups.push(target_group);
                }
                let loop_value = &target_group.value;
                last_value = loop_value.to_string();
            }
        }

        // has group to the left
        if group.start_pos > 0 {
            groups.push(row.get_group_at(group.start_pos - 1).unwrap());
        }
        // has group to the right
        if group.end_pos < row.width - 1 {
            groups.push(row.get_group_at(group.end_pos + 1).unwrap());
        }
        groups
    }
}

fn main() {
    let data = fs::read_to_string("data.txt").expect("Unable to read file");
    let grid = Grid::from_data(&data);

    let result = part_one(&grid);
    println!("part one result {:?}", result);

    let result = part_two(&grid);
    println!("part two result {:?}", result);
}

fn part_one(grid: &Grid) -> u32 {
    let all_numbers = grid.get_groups_of_type(Type::Number);
    let mut result = 0;

    for number in all_numbers {
        let surrounding_groups = grid.get_surrounding_groups(&number);
        let has_symbol_nabor = surrounding_groups
            .iter()
            .any(|x| x.get_type() == Type::Symbol || x.get_type() == Type::Gear);

        if has_symbol_nabor {
            result += number.value.parse::<u32>().unwrap();
        }
    }

    result
}

fn part_two(grid: &Grid) -> u32 {
    let all_gears = grid.get_groups_of_type(Type::Gear);
    let mut result = 0;

    for gear in all_gears {
        let surrounding_groups = grid.get_surrounding_groups(&gear);

        let number_nabors = surrounding_groups
            .iter()
            .filter(|x| x.get_type() == Type::Number).collect::<Vec<_>>();

        if number_nabors.len() == 2 {
            let first_number = number_nabors.first().unwrap().value.parse::<u32>().unwrap();
            let second_number = number_nabors.last().unwrap().value.parse::<u32>().unwrap();

            result += first_number * second_number;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();

        let grid = Grid::from_data(&data);
        let result = part_one(&grid);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_part_two() {
        let data = "467..114.
...*.....
..35..633
......#..
617*.....
.....+.58
..592....
......755
...$.*...
.664.598."
            .to_string();

        let grid = Grid::from_data(&data);
        let result = part_two(&grid);

        assert_eq!(result, 467835);
    }
}
