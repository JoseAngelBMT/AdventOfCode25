use crate::board::{Board, Coord};
use anyhow::Result;

fn surrounding_cells(board: &Board<char>, coord: Coord) -> i32 {
    [
        coord.up(),
        coord.down(),
        coord.left(),
        coord.right(),
        Coord::new(coord.x + 1, coord.y + 1),
        Coord::new(coord.x + 1, coord.y - 1),
        Coord::new(coord.x - 1, coord.y + 1),
        Coord::new(coord.x - 1, coord.y - 1),
    ]
    .iter()
    .map(|&adj_coord| board.get_value(adj_coord))
    .map(|x| match x {
        Some('@') => 1,
        _ => 0,
    })
    .sum::<i32>()
}


fn remove_rolls(grid: &mut Board<char>) -> i32 {
    let grid_ref: &Board<char> = &grid.clone();
    grid.rows
        .iter_mut()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .filter_map(move |(x, v)| {
                    let coord = Coord::new(x as i32, y as i32);
                    if surrounding_cells(&grid_ref, coord) < 4 && *v == '@' {
                        *v = '.';
                        Some(())
                    } else {
                        None
                    }
                })
        })
        .count() as i32
}

fn remove_rolls_as_possible(grid: &mut Board<char>) -> i32 {
    let mut count: i32 = 0;
    loop {
        let act_count = remove_rolls(grid);
        count += act_count;
        if act_count == 0 {
            break;
        }
    }
    count
}

pub fn solve() -> Result<()> {
    let grid = Board::read_char_board("inputs/day04.txt");
    println!("Part1: {}", remove_rolls(&mut grid.clone()));
    println!("Part2: {}", remove_rolls_as_possible(&mut grid.clone()));
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::board::Board;
    use crate::days::day04::{remove_rolls, remove_rolls_as_possible};

    #[test]
    fn test_day4(){
        let grid = Board::read_char_board("test/test_day04.txt");
        assert_eq!(remove_rolls(&mut grid.clone()), 13);
        assert_eq!(remove_rolls_as_possible(&mut grid.clone()), 43);
    }
}
