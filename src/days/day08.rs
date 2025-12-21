use anyhow::Result;
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

#[derive(Debug)]
struct Circuit {
    circuits: HashMap<Box, usize>,
    groups: HashMap<usize, Vec<Box>>,
}

impl Circuit {
    fn create_circuits(boxes: &Vec<Box>) -> Self {
        let mut circuits: HashMap<Box, usize> = HashMap::new();
        let mut groups: HashMap<usize, Vec<Box>> = HashMap::new();
        for (i, b) in boxes.iter().enumerate() {
            circuits.insert(b.clone(), i);
            groups.insert(i, vec![b.clone()]);
        }
        Self { circuits, groups }
    }

    fn update_group(&mut self, a: &Box, b: &Box) {
        let group_a = *self.circuits.get_mut(a).unwrap();
        let group_b = *self.circuits.get_mut(b).unwrap();
        if group_a == group_b {
            return;
        }

        let mut changes = self.groups.remove(&group_b).unwrap();
        let group_boxes = self.groups.get_mut(&group_a).unwrap();
        group_boxes.append(&mut changes);

        for change in group_boxes.iter() {
            self.circuits.insert(*change, group_a);
        }
    }

    fn multiply_three_largest_circuits(&self) -> i64 {
        let mut lengths = self.groups
            .iter()
            .map(|(_,vec)| vec.len() as i64)
            .collect::<Vec<i64>>();

        lengths.sort_unstable_by(|a, b| b.cmp(a));
        lengths.iter().take(3).product()
    }

    fn is_one_circuit(&self) -> bool {
        self.groups.len() == 1
    }
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


fn make_circuits(boxes: &Vec<Box>, n: usize) -> i64 {
    let shortest_distance = closest_n_pairs(boxes, n);
    let mut circuit = Circuit::create_circuits(boxes);

    for (_, a, b) in shortest_distance {
        circuit.update_group(&a, &b);
    }

    circuit.multiply_three_largest_circuits()
}

fn last_boxes_conected(boxes: &Vec<Box>) -> i64 {
    let n = boxes.len();
    let shortest_distance = closest_n_pairs(boxes, n*n);
    let mut circuit = Circuit::create_circuits(boxes);

    for (_, a, b) in shortest_distance {
        circuit.update_group(&a, &b);
        if circuit.is_one_circuit() {
            return (a.x * b.x) as i64
        }
    }
    -1
}

pub fn solve() -> Result<()> {
    let boxes: Vec<Box> = read_file("inputs/day08.txt");
    println!("Part 1: {}", make_circuits(&boxes, 1000));
    println!("Part 2: {}", last_boxes_conected(&boxes));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::days::day08::{last_boxes_conected, make_circuits, read_file, Box, Circuit};
    #[test]
    fn test_change_groups(){
        let a = Box { x: 0, y: 0, z: 0 };
        let b = Box { x: 1, y: 1, z: 1 };
        let c = Box { x: 2, y: 2, z: 2 };
        let mut circuit = Circuit::create_circuits(&vec![a, b, c]);
        circuit.update_group(&a, &b);
        assert_eq!(circuit.circuits.get(&a).unwrap(), circuit.circuits.get(&b).unwrap());
        assert_eq!(circuit.groups.get(&0).unwrap().len(), 2);
        circuit.update_group(&c, &a);
        assert_eq!(circuit.groups.get(&2).unwrap().len(), 3);
    }

    #[test]
    fn test_day8(){
        let boxes: Vec<Box> = read_file("test/test_day08.txt");
        assert_eq!(make_circuits(&boxes, 10), 40);
        assert_eq!(last_boxes_conected(&boxes), 25272);
    }
}
