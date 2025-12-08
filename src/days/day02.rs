use anyhow::Result;
use std::fs;

fn read_file(path: &str) -> Vec<(i64, i64)> {
    let file_content = fs::read_to_string(path).unwrap();
    file_content
        .trim()
        .split(",")
        .map(|range| {
            let mut parts = range
                .split("-")
                .map(|x| x
                    .trim()
                    .parse::<i64>()
                    .unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect()
}

fn find_duplicated(number: &str, digits: usize) -> bool{
    if number.len() % digits != 0 {return false;}
    let parts: Vec<&str> = (0..number.len())
        .step_by(digits)
        .map(|i| &number[i..i+digits])
        .collect();
    parts.iter().all(|&part| part == parts[0])
}

fn is_invalid_all(number: &i64) -> bool {
    let str_number = number.to_string();
    let len = str_number.len();
    for i in 1..len/2+1 {
        if find_duplicated(str_number.as_str(), i) {
            return true;
        }
    }
    false
}
fn is_invalid(number: &i64) -> bool {
    let str_number = number.to_string();
    let len = str_number.len();
    if len % 2 == 0{
        return find_duplicated(str_number.as_str(), len/2);
    }
    false
}

fn day2(numbers: Vec<(i64, i64)>, f: fn(&i64) -> bool) -> i64 {
    numbers
        .iter()
        .map(|(r1, r2)|
            (*r1..*r2+1)
            .filter(|x| f(x))
                .sum::<i64>()
        )
        .sum()
}

pub fn solve() -> Result<()> {
    let numbers = read_file("inputs/day02.txt");
    println!("Part1: {}", day2(numbers.clone(), is_invalid));
    println!("Part2: {}", day2(numbers, is_invalid_all));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::days::day02::{day2, is_invalid, is_invalid_all, read_file};

    #[test]
    fn test_day1(){
        let numbers = read_file("test/test_day02.txt");
        assert_eq!(day2(numbers.clone(), is_invalid), 1227775554);
        assert_eq!(day2(numbers, is_invalid_all), 4174379265);
    }
}