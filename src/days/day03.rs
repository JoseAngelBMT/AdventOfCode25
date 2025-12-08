use crate::board::Board;
use anyhow::Result;

fn find_max_joltage(row: &[i32], n: usize, result: u64) -> u64 {
    if n == 1 {
        return result + row.iter().max().unwrap().clone() as u64;
    }

    let numbers = &row[..(row.len()-n+1)];
    let max_value = *numbers.iter().max().unwrap();
    let pos_value = numbers.iter().position(|&x| x == max_value).unwrap();
    find_max_joltage(&row[(pos_value+1)..],
                               n-1,
                               result + (max_value as u64 * 10u64.pow(n as u32 - 1)))
}

fn day3(board: &Board<i32>, n: usize) -> u64 {
    board.rows
        .iter()
        .map(|row| find_max_joltage(row.as_slice(), n, 0))
        .sum()
}

pub fn solve() -> Result<()> {
    let matrix = Board::read_int_board("inputs/day03.txt");
    println!("Parte 1: {}", day3(&matrix, 2));
    println!("Parte 12: {}", day3(&matrix, 12));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::days::day03::day3;

    #[test]
    fn test_day3(){
        let board = Board::read_int_board("test/test_day03.txt");
        assert_eq!(day3(&board, 2), 357);
        assert_eq!(day3(&board, 12), 3121910778619);
    }
}
