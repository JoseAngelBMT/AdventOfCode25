use anyhow::Result;
use std::fs;

fn read_ranges(content: &str) -> Vec<(i64, i64)> {
    content
        .lines()
        .map(|line| {
            let mut parts = line.split("-");
            let r1: i64 = parts.next().unwrap().parse().unwrap_or(0);
            let r2: i64 = parts.next().unwrap().parse().unwrap_or(0);
            (r1, r2)
        })
        .collect()
}

fn read_file(path: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let file_content = fs::read_to_string(path).unwrap();

    let mut parts = file_content.split("\n\n"); // Only it works with LF separator in .txt
    let first_part = parts.next().unwrap_or("").trim();
    let second_part = parts.next().unwrap_or("").trim();
    let ranges = read_ranges(first_part);
    let values = second_part
        .lines()
        .map(|line| line.parse::<i64>().unwrap_or(0))
        .collect();
    (ranges, values)
}

fn clean_ranges(ranges: &mut Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by_key(|&(r1, _)| r1);
    let mut cleaned: Vec<(i64, i64)> = vec![ranges[0]];

    for range in ranges.iter_mut().skip(1) {
        let last = cleaned.last_mut().unwrap();

        if last.1 >= range.0 {
            last.1 = last.1.max(range.1);
        } else {
            cleaned.push(*range);
        }
    }
    cleaned
}

fn fresh_ids(ranges: &Vec<(i64, i64)>) -> i64 {
    let cleaned = clean_ranges(&mut ranges.clone());
    cleaned
        .iter()
        .map(|&(r1, r2)| (r2 - r1) + 1)
        .sum()
}

fn fresh_ingredients(ranges: &Vec<(i64, i64)>, values: &Vec<i64>) -> i64 {
    values
        .iter()
        .filter(|v| ranges
            .iter()
            .any(|(r1, r2)| r1 <= v && r2 >= v))
        .count() as i64
}

pub fn solve() -> Result<()> {
    let (ranges, values) = read_file("inputs/day05.txt");
    println!("Part 1: {}", fresh_ingredients(&ranges, &values));
    println!("Part 2: {}", fresh_ids(&ranges));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::days::day05::{fresh_ingredients, read_file, fresh_ids};

    #[test]
    fn test_day5() {
        let (ranges, values) = read_file("test/test_day05.txt");
        assert_eq!(fresh_ingredients(&ranges, &values), 3);
        assert_eq!(fresh_ids(&ranges), 14);
    }
}
