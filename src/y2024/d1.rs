use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

pub fn part_1() {
    let string = fs::read_to_string("data/2024/1/part_1_big.txt")
        .expect("Should have been able to read the file");

    let (mut left_numbers, mut right_numbers) = parse::parse_input(string);

    left_numbers.sort();
    right_numbers.sort();
    let sorted_pairs = left_numbers.into_iter().zip(right_numbers.into_iter());

    let sum_of_differences: i32 = sorted_pairs.map(|(a, b)| (a - b).abs()).sum();

    print!(
        "Day 1 Part 1
    total distance between lists: {}",
        sum_of_differences
    );
}

pub fn part_2() {
    let string = fs::read_to_string("data/2024/1/part_1_big.txt")
        .expect("Should have been able to read the file");

    let (left_numbers, right_numbers) = parse::parse_input(string);
    let right_counts: HashMap<i32, usize> = right_numbers.into_iter().counts();

    let similarity_score: i32 = left_numbers
        .into_iter()
        .map(|l| l * *right_counts.get(&l).unwrap_or(&0) as i32)
        .sum();

    print!(
        "Day 1 Part 2
    similarity score: {}",
        similarity_score
    );
}

mod parse {

    pub fn parse_input(string: String) -> (Vec<i32>, Vec<i32>) {
        let lines = string.split("\n").collect::<Vec<&str>>();
        let left_numbers = lines.iter().map(split_left_num).collect::<Vec<i32>>();
        let right_numbers = lines.iter().map(split_right_num).collect::<Vec<i32>>();
        (left_numbers, right_numbers)
    }

    fn split_left_num(line: &&str) -> i32 {
        let items = line.split_whitespace().collect::<Vec<&str>>();
        items[0].parse::<i32>().unwrap()
    }

    fn split_right_num(line: &&str) -> i32 {
        let items = line.split_whitespace().collect::<Vec<&str>>();
        items[1].parse::<i32>().unwrap()
    }
}
