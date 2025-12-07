use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Board<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Board<T>
where
    T: PartialEq + Debug + Display + Copy,
{
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        Self { rows }
    }

    pub fn read_board(path: &str, parser: &dyn Fn(&char) -> T) -> Self {
        let file = File::open(path);
        let reader = BufReader::new(file.unwrap());
        let rows = reader
            .lines()
            .map(|line| line.unwrap().chars().map(|c| parser(&c)).collect())
            .collect();
        Self::new(rows)
    }

    fn empty_board(width: usize, height: usize, default_value: T) -> Self {
        let rows = vec![vec![default_value; width]; height];
        Self::new(rows)
    }

    pub fn get_value(&self, coord: Coord) -> Option<&T> {
        let x: usize = coord.x.try_into().ok()?;
        let y: usize = coord.y.try_into().ok()?;

        self.rows.get(y).and_then(|row| row.get(x))
    }
    pub fn set_value(&mut self, coord: Coord, value: T) {
        let x: usize = coord.x.try_into().ok().unwrap();
        let y: usize = coord.y.try_into().ok().unwrap();
        self.rows[y][x] = value;
    }

    pub fn is_in_bound(&self, coord: Coord) -> bool {
        self.get_value(coord).is_some()
    }

    pub fn find_element(&self, element: T) -> Option<Coord> {
        self.rows.iter().enumerate().find_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .find(|&(_, cell)| cell == &element)
                .map(|(col_idx, _)| Coord::new(col_idx as i32, row_idx as i32))
        })
    }

    pub fn print_board(&self) {
        for row in &self.rows {
            let line: String = row
                .iter()
                .map(|cell| format!("{}", cell))
                .collect::<Vec<_>>()
                .join("");
            println!("{}", line);
        }
    }
}

impl Board<char> {
    pub fn read_char_board(path: &str) -> Self {
        Self::read_board(path, &|c| *c)
    }

    pub fn from_string(p0: &str) -> Self {
        let rows: Vec<Vec<char>> = p0
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        Self { rows }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self::empty_board(width, height, '.')
    }
}

impl Board<i32> {
    pub fn read_int_board(path: &str) -> Self {
        Self::read_board(path, &|c| c.to_digit(10).unwrap() as i32)
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self::empty_board(width, height, 0)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    pub fn up(&self) -> Coord {
        Coord::new(self.x, self.y - 1)
    }

    pub fn down(&self) -> Coord {
        Coord::new(self.x, self.y + 1)
    }

    pub fn left(&self) -> Coord {
        Coord::new(self.x - 1, self.y)
    }

    pub fn right(&self) -> Coord {
        Coord::new(self.x + 1, self.y)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul for Coord {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}