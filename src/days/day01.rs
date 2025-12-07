use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        let cleaned = line.unwrap().replace("R", "").replace("L", "-");

        let number: i32 = cleaned.trim().parse().unwrap();
        numbers.push(number);
    }
    numbers
}

fn day1(numbers: &Vec<i32>) -> (i32, i32) {
    let mut current: i32 = 50;
    let mut last_number: i32 = current;
    let (mut count_1, mut count_2): (i32, i32) = (0, 0);

    for number in numbers {
        last_number = current;
        current += number;

        count_2 += part2(current, last_number);

        current = ((current % 100) + 100) % 100;
        if current == 0 {
            count_1 += 1;
        }
    }
    (count_1, count_2)
}

fn part2(current: i32, last_number: i32) -> i32 {
    match (current, last_number) {
        (current, 0) => current.abs() / 100,
        (0, _) => 1,
        (current, _) if current > 0 => current / 100,
        (current, _) if current < 0 => current.abs() / 100 + 1,
        _ => panic!("Part 2 of day 1 should never get here"),
    }
}

pub fn solve() -> Result<()> {
    let input_day1 = read_file("inputs/day1.txt");
    let (res_part1, res_part2) = day1(&input_day1);
    println!("Part 1: {}", res_part1);
    println!("Part 2: {}", res_part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::days::day01::{day1, read_file};

    #[test]
    fn test_day1() {
        let input_day1 = read_file("test/test_day01.txt");
        let result = day1(&input_day1);
        assert_eq!(result, (4, 24));
    }
}