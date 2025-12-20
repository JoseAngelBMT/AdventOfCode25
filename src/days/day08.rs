use anyhow::Result;
use itertools::Itertools;
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Box {
    x: i32,
    y: i32,
    z: i32,
}

impl PartialOrd for Box {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Box {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => match self.y.cmp(&other.y) {
                Ordering::Equal => self.z.cmp(&other.z),
                ord => ord,
            },
            ord => ord,
        }
    }
}

struct Circuit{
    id: usize,
    boxes: Vec<Box>,
}


fn read_file(path: &str) -> Vec<Box> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let nums: [i32; 3] = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap();
            Box {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            }
        })
        .collect()
}

fn ordered_pair(a: Box, b: Box) -> (Box, Box) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

fn closest_n_pairs(boxes: &Vec<Box>, n: usize) -> Vec<(f64, Box, Box)> {
    let mut distances: HashMap<(Box, Box), f64> = HashMap::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let (a, b) = ordered_pair(boxes[i], boxes[j]);
            *distances.entry((a, b)).or_insert(0.0) = distance(&a, &b);
        }
    }
    let top_n = {
        let mut vec: Vec<_> = distances
            .iter()
            .map(|(&(a, b), &dist)| (dist, a, b))
            .collect();

        vec.sort_unstable_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap());
        vec.truncate(n);
        vec
    };
    top_n
}

fn distance(a: &Box, b: &Box) -> f64 {
    // Euclidean distance
    let dx = (a.x - b.x) as f64;
    let dy = (a.y - b.y) as f64;
    let dz = (a.z - b.z) as f64;
    (dx.powf(2.0) + dy.powf(2.0) + dz.powf(2.0)).sqrt()
}

fn create_circuit_map(boxes: &Vec<Box>) -> HashMap<Box, usize> {
    boxes.iter().map(|b| (b, -1)).collect()
}

fn make_circuits(boxes: &Vec<Box>, n: usize) -> i64 {
    let shortest_distance = closest_n_pairs(boxes, n);
    let mut circuits: Vec<Circuit> = Vec::new();
    let mut circuit_map = create_circuit_map(boxes);

    for (_, a, b) in shortest_distance {
        let a_value = *circuit_map.get(&a).unwrap();
        circuit_map.insert(b, a_value);
        println!("{:?} -> {:?}, Group: {}", a, b, a_value);
    }
    0
}

pub fn solve() -> Result<()> {
    let boxes: Vec<Box> = read_file("test/test_day08.txt");
    println!("{:?}", make_circuits(&boxes, 10));
    Ok(())
}
