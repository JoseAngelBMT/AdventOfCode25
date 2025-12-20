use crate::board::Board;
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn operate(row: &Vec<i64>, operation: String) -> i64 {
    match operation.as_ref() {
        "+" => row.iter().sum(),
        "*" => row.iter().product(),
        _ => panic!("Unknown operation"),
    }
}

fn operate_numbers(n1: i64, n2: i64, operation: String) -> i64 {
    match operation.as_ref() {
        "+" => n1 + n2,
        "*" => n1 * n2,
        _ => panic!("Unknown operation"),
    }
}

fn read_file(path: &str) -> (Vec<Vec<i64>>, Vec<String>) {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let mut content_line: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let operation_line: Vec<String> = content_line
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let numbers: Vec<Vec<i64>> = content_line
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    (transpose(numbers), operation_line)
}

fn transpose(vec: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut transposed: Vec<Vec<i64>> = vec![vec![]; vec[0].len()];
    for i in 0..vec[0].len() {
        for j in 0..vec.len() {
            transposed[i].push(vec[j][i].clone());
        }
    }
    transposed
}

fn apply_operation(numbers: &Vec<Vec<i64>>, operation: &Vec<String>) -> i64 {
    numbers
        .iter()
        .enumerate()
        .map(|(i, row)| operate(row, operation[i].clone()))
        .sum()
}

fn transpose_to_vector(board: Board<char>) -> Vec<Vec<i64>> {
    let result: Vec<i64> = board
        .rows
        .iter()
        .map(|x| {
            let s: String = x.iter().collect();
            let s_n = s.trim();
            s_n.parse::<i64>().unwrap_or(-1)
        })
        .collect();

    result
        .split(|x| *x == -1)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn columns_to_rows(board: &mut Board<char>, operations: &Vec<String>) -> i64 {
    board.rows.pop();
    let transpose = board.transpose();
    let numbers: Vec<Vec<i64>> = transpose_to_vector(transpose);
    apply_operation(&numbers, &operations)
}

pub fn solve() -> Result<()> {
    let path: &str = "inputs/day06.txt";
    let (numbers, operations) = read_file(path);
    println!("Part 1: {}", apply_operation(&numbers, &operations));
    let mut board = Board::read_char_board(path);
    println!("Part 2: {}", columns_to_rows(&mut board, &operations));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::days::day06::{apply_operation, read_file, columns_to_rows};

    #[test]
    fn test_day6() {
        let path: &str = "test/test_day06.txt";
        let (numbers, operations) = read_file(path);
        assert_eq!(apply_operation(&numbers, &operations), 4277556);
        let mut board = Board::read_char_board(path);
        assert_eq!(columns_to_rows(&mut board, &operations), 3263827)
    }
}
