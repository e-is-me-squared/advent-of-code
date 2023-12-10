use std::fs;

fn main() {
    let binding = read_file();
    let lines = binding.split("\n");

    let mut current = 0;
    let mut biggest = 0;

    for part in lines {
        if part.len() == 0 {
            current = 0;
            continue;
        }
        current += part.parse::<u32>().unwrap();
        if current > biggest {
            biggest = current;
        }
    }

    println!("Biggest number: {}", biggest);
}

fn read_file() -> String {
    let content = fs::read_to_string("data.txt").expect("Something went wrong reading the file");
    return content;
}
