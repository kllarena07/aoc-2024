use std::fs;
use crate::solution::Solution;

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();

    let s = Solution;
    // s.star_one();
    s.star_two();
}
