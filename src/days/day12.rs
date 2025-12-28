use crate::board::Board;
use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> (HashMap<usize, Board<char>>, Vec<(usize, usize, Vec<usize>)>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut boards = HashMap::new();
    let mut instructions = Vec::new();

    let mut current_id: Option<usize> = None;
    let mut current_rows: Vec<Vec<char>> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let trim_line = line.trim();

        if trim_line.is_empty() {
            if let Some(id) = current_id {
                boards.insert(id, Board::new(current_rows.clone()));
                current_rows.clear();
                current_id = None;
            }
            continue;
        }

        // Detectar cabecera de bloque (ej: "0:") vs instrucción (ej: "4x4:")
        if trim_line.ends_with(':') && !trim_line.contains('x') {
            // Guardamos el anterior si quedó pendiente
            if let Some(id) = current_id {
                boards.insert(id, Board::new(current_rows.clone()));
                current_rows.clear();
            }

            let id_str = &trim_line[..trim_line.len() - 1];
            current_id = Some(id_str.parse().expect("ID inválido"));
        } else if trim_line.contains(':') {
            if let Some(id) = current_id {
                boards.insert(id, Board::new(current_rows.clone()));
                current_rows.clear();
                current_id = None;
            }

            let parts: Vec<&str> = trim_line.split(':').collect();
            let dims_str = parts[0].trim();
            let nums_str = parts[1].trim();

            let dims: Vec<&str> = dims_str.split('x').collect();
            let width = dims[0].parse().unwrap();
            let height = dims[1].parse().unwrap();

            let sequence: Vec<usize> = nums_str
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            instructions.push((width, height, sequence));
        } else {
            if current_id.is_some() {
                current_rows.push(trim_line.chars().collect());
            }
        }
    }

    if let Some(id) = current_id {
        boards.insert(id, Board::new(current_rows));
    }

    (boards, instructions)
}

// When checking whether an entry is possible, the number of boards that is a solution is obtained. A little trick
fn check_fit(
    boards: &HashMap<usize, Board<char>>,
    instructions: &Vec<(usize, usize, Vec<usize>)>,
) -> u32 {
    let mut count = 0;
    for (w, h, vec) in instructions {
        let area = w * h;
        let board_area: usize = vec
            .iter()
            .enumerate()
            .map(|(idx, v)| boards.get(&idx).unwrap().count_values('#') * v)
            .sum();
        println!("{:?} {:?}", area, board_area);
        if board_area < area {
            count += 1;
        }
    }
    count
}

pub fn solve() -> Result<()> {
    let (boards, instructions) = read_file("inputs/day12.txt");
    println!("Part 1: {}", check_fit(&boards, &instructions));
    Ok(())
}
