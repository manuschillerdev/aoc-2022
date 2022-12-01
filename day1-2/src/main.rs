use std::fs::File;
use std::io::*;

fn main() {
    let file = File::open("input/day1.txt").expect("failed to open input.");
    let reader = BufReader::new(file);

    let result_size = 3;
    let mut stack = vec![0;result_size];

    let mut current = 0;
    for line in reader.lines() {
        let value = line.expect("could not read line.");
        if value.is_empty() {
            for (pos, _) in stack.iter().enumerate() {
                if current > stack[pos] {
                    stack.insert(pos, current);
                    break;
                }
            }

            stack.drain(result_size..);
            current = 0;
        } else {
            current += value.parse::<i32>().expect("could not parse line to int.")
        }
    }

    let result: i32 = stack.iter().sum();

    println!("result: {}", result);
}
