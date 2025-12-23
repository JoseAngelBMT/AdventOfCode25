use crate::board::Coord;
use anyhow::Result;
use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn read_file(path: &str) -> Vec<Coord> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let nums: [i32; 2] = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap();
            Coord::new(nums[0], nums[1])
        })
        .collect()
}

fn find_large_area(coords: &Vec<Coord>) -> i64 {
    coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            coords[i + 1..].iter().map(move |b| calculate_area(a, b))
        })
        .max()
        .unwrap_or(0)
}

fn perimeter_between_coords(a: &Coord, b: &Coord) -> HashSet<Coord> {
    if a.x == b.x {
        let start = a.y.min(b.y);
        let end = a.y.max(b.y);
        (start..=end).map(|y| Coord::new(a.x, y)).collect()
    } else {
        let start = a.x.min(b.x);
        let end = a.x.max(b.x);
        (start..=end).map(|x| Coord::new(x, a.y)).collect()
    }
}

fn calculate_perimeter(coord: &Vec<Coord>) -> HashSet<Coord> {
    // It is assumed that the coordinates are ordered to construct the perimeter.
    let mut perimeter: HashSet<Coord> = HashSet::new();
    for subset in coord.windows(2) {
        let a = subset[0];
        let b = subset[1];
        perimeter.extend(perimeter_between_coords(&a, &b));
    }
    perimeter.extend(perimeter_between_coords(&coord[0], &coord[coord.len() - 1]));
    perimeter
}

fn perimeter_to_hashmap(perimeter: &HashSet<Coord>) -> HashMap<i32, Vec<Coord>> {
    let mut perimeter_y: HashMap<i32, Vec<Coord>> = HashMap::new();
    for c in perimeter {
        perimeter_y
            .entry(c.y)
            .or_default()
            .push(*c);
    }
    for (_y, coords) in perimeter_y.iter_mut() {
        coords.sort_by_key(|c| c.x);
    }
   perimeter_y
}

fn calculate_area(a: &Coord, b: &Coord) -> i64 {
    let dx = (a.x as i64 - b.x as i64).abs() + 1;
    let dy = (a.y as i64 - b.y as i64).abs() + 1;
    dx * dy
}

fn find_cross_perimeter(min: i32, max: i32, perimeter_x: &Vec<Coord>) -> bool {
    for per in perimeter_x {
        if (min < per.x) && (per.x < max) { return true }
    }
    false
}

fn is_perimeter_inside(a: &Coord, b: &Coord, perimeter_y: &HashMap<i32, Vec<Coord>>) -> bool {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);

    for y in (min_y+1)..=(max_y-1) {
        if let Some(x_coords) = perimeter_y.get(&y) {
            if find_cross_perimeter(min_x, max_x, x_coords) {
                return true;
            }
        }
    }
    false
}

fn sort_pairs(coords: &[Coord]) -> HashMap<(Coord, Coord), i64> {
    let mut pairs: Vec<((Coord, Coord), i64)> = coords
        .iter()
        .combinations(2)
        .map(|pair| {
            let a = *pair[0];
            let b = *pair[1];
            let area = calculate_area(&a, &b);
            ((a, b), area)
        })
        .collect();

    pairs.sort_by(|(_, area1), (_, area2)| area2.cmp(area1));

    pairs.into_iter().collect()
}

fn find_green_area(coords: &Vec<Coord>) -> i64 {
    let perimeter = calculate_perimeter(coords);
    let perimeter_y = perimeter_to_hashmap(&perimeter);
    let pairs = sort_pairs(&coords);
    let mut result: i64 = 0;

    for ((a, b), area) in pairs {
        if area > result {
            if !is_perimeter_inside(&a, &b, &perimeter_y) {
                result = area;
            }
        }
    }
    result
}

pub fn solve() -> Result<()> {
    let coords = read_file("inputs/day09.txt");
    println!("Part 1: {}", find_large_area(&coords));
    println!("Part 2: {}", find_green_area(&coords));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::days::day09::{find_green_area, find_large_area, read_file};

    #[test]
    fn test_day9() {
        let coords = read_file("test/test_day09.txt");
        assert_eq!(find_large_area(&coords), 50);
        assert_eq!(find_green_area(&coords), 24);
    }

}
