use crate::board::{Board, Coord};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn recursive_down(
    board: &Board<char>,
    coord: &Coord,
    splitters: &mut HashSet<Coord>,
    visited: &mut HashMap<Coord, i64>,
) -> i64 {
    if let Some(&value) = visited.get(coord) {
        return value;
    }

    let mut result = 0;
    if let Some(value) = board.get_value(*coord) {
        if *value == '^' {
            splitters.insert(coord.clone());
            result = recursive_down(board, &coord.right(), splitters, visited)
                + recursive_down(board, &coord.left(), splitters, visited);
        } else {
            result = recursive_down(board, &coord.down(), splitters, visited);
        }
    } else {
        return 1;
    }
    visited.insert(*coord, result);
    result
}

fn beam_encounters(board: &Board<char>, start_coord: &Coord) -> (i32, i64) {
    let mut splitters: HashSet<Coord> = HashSet::new();
    let mut visited: HashMap<Coord, i64> = HashMap::new();
    let paths = recursive_down(board, &start_coord.down(), &mut splitters, &mut visited);
    (splitters.len() as i32, paths)
}

pub fn solve() -> Result<()> {
    let board = Board::read_char_board("inputs/day07.txt");
    let start_coord = board.find_element('S').unwrap();
    let (part1, part2) = beam_encounters(&board, &start_coord);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::days::day07::beam_encounters;

    #[test]
    fn test_day7() {
        let board = Board::read_char_board("test/test_day07.txt");
        let start_coord = board.find_element('S').unwrap();
        let (part1, part2) = beam_encounters(&board, &start_coord);
        assert_eq!(part1, 21);
        assert_eq!(part2, 40);
    }
}
