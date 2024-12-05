use std::fs;

fn main() {
    let file_contents = fs::read_to_string("./input.txt").unwrap();
    let split_contents = file_contents.trim_end().split('\n');
    let mut safe_reports = 0;

    'outer: for (_, line) in split_contents.enumerate() {
        let str_numbers: Vec<&str> = line.split(" ").collect();
        let mut int_numbers: Vec<i32> = Vec::new();
        
        for number in str_numbers.iter() {
            let to_int = number.parse().unwrap();
            int_numbers.push(to_int); 
        }

        let increasing = int_numbers[1] > int_numbers[0];

        for (i, _) in int_numbers.iter().enumerate() {
            if i + 1 < int_numbers.len() {
                if int_numbers[i + 1] < int_numbers[i] && increasing {
                    // break if increasing isn't consistent
                    continue 'outer;
                }

                if int_numbers[i + 1] > int_numbers[i] && !increasing {
                    // break if decreasing isn't consistent
                    continue 'outer;
                }

                let difference = (int_numbers[i + 1] - int_numbers[i]).abs();
                if difference < 1 || difference > 3 {
                    // break if adjacent level difference is too little or too much
                    continue 'outer;
                }
            }
        }

        safe_reports += 1;
    }

    println!("{}", safe_reports);
}
