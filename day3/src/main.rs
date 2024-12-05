use std::fs;
use regex::Regex;

fn main() {
    let file_content = fs::read_to_string("./input.txt").unwrap();
    let trimmed_content = file_content.trim_end();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total = 0;

    for caps in re.captures_iter(trimmed_content) {
        let first_number: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let second_number: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        total += first_number * second_number;
    }

    println!("{}", total);
}
