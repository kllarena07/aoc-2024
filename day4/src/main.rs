use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();
    let trimmed_contents = file_contents.trim_end();

    println!("{}", trimmed_contents);
}
