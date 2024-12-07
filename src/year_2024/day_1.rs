use std::fs;

pub fn run() {
    let string = fs::read_to_string("data/2024/1/part_1_big.txt")
        .expect("Should have been able to read the file");

    let lines = string.split("\n").collect::<Vec<&str>>();

    let mut left_numbers = lines.iter().map(split_left_num).collect::<Vec<i32>>();
    let mut right_numbers = lines.iter().map(split_right_num).collect::<Vec<i32>>();

    left_numbers.sort();
    right_numbers.sort();

    let pairs = left_numbers.iter().zip(right_numbers.iter());
    let sum_of_differences: i32 = pairs.map(difference).sum();

    print!("sum of differences: {}", sum_of_differences);
}

fn split_left_num(line: &str) -> i32 {
    let items = line.split_whitespace().collect::<Vec<&str>>();
    items[0].parse::<i32>().unwrap()
}

fn split_right_num(line: &str) -> i32 {
    let items = line.split_whitespace().collect::<Vec<&str>>();
    items[1].parse::<i32>().unwrap()
}

fn difference(a: i32, b: i32) -> i32 {
    (b - a).abs()
}
