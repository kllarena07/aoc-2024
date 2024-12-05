use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();
    let split_contents = file_contents.trim_end().split('\n');

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for (_, line) in split_contents.enumerate() {
        let numbers: Vec<&str> = line.split("   ").collect();
        let left_number = numbers[0].parse().unwrap();
        let right_number = numbers[1].parse().unwrap();

        left_list.push(left_number);
        right_list.push(right_number);
    }

    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;

    let list_len = left_list.len();

    for i in 0..list_len {
        let distance = (left_list[i] - right_list[i]).abs();
        total_distance += distance;
    }

    println!("{}", total_distance);
}
