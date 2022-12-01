use std::io::*;
use std::fs::File;
fn main() {
    let file = File::open("input/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut max = 0;
    let mut current = 0;
    for line in reader.lines() {
        let value = line.unwrap();
        if value.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current = current + value.parse::<i32>().unwrap()
        }
    }
    println!("max {}", max);
}