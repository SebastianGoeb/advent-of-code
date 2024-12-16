use itertools::Itertools;
use std::fs;

pub fn part_1() {
    let input = fs::read_to_string("data/2024/2/part_1_big.txt")
        .expect("Should have been able to read the file");
    let reports = parse::parse_reports(&input);
    let safe_reports = reports
        .iter()
        .map(|report| is_report_safe(report))
        .filter(|&b| b)
        .count();

    println!(
        "Day 2 Part 1
    safe reports: {}",
        safe_reports
    );
}

fn is_report_safe(levels: &[i32]) -> bool {
    let signs: Vec<i32> = levels
        .iter()
        .tuple_windows::<(&i32, &i32)>()
        .map(|(a, b)| (a - b).signum())
        .collect();
    let all_increasing = signs.iter().all(|sign| *sign == 1);
    let all_decreasing = signs.iter().all(|sign| *sign == -1);

    let all_in_range = levels
        .iter()
        .tuple_windows::<(&i32, &i32)>()
        .map(|(a, b)| (a - b).abs())
        .all(|diff| diff >= 1 && diff <= 3);

    (all_increasing || all_decreasing) && all_in_range
}

pub fn part_2() {}

mod parse {
    pub fn parse_reports(input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .into_iter()
            .map(|line| {
                line.split_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect()
    }
}
