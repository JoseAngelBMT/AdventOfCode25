use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Clone, Debug)]
struct Machine {
    wiring: u32,
    raw_buttons: Vec<Vec<usize>>,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
}

fn str_to_bin(s: String) -> u32 {
    let bin_str: String = s
        .chars()
        .map(|c| match c {
            '.' => '0',
            '#' => '1',
            _ => panic!("Wrong char: {}", c),
        })
        .collect();
    u32::from_str_radix(&bin_str, 2).expect("Failed to parse number")
}
fn vector_to_bin(vector: &Vec<usize>, n: usize) -> u32 {
    let mut result = 0;
    for v in vector {
        if v < &n {
            let shift = n - 1 - v;
            result |= 1 << shift;
        }
    }
    result
}
fn remove_parenthesis(s: String) -> String {
    s.replace("[", "")
        .replace("]", "")
        .replace("{", "")
        .replace("}", "")
        .replace("(", "")
        .replace(")", "")
}

fn read_file(path: &str) -> Vec<Machine> {
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let mut vectors: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
            let n = vectors[0].len() - 2;
            let wiring = str_to_bin(remove_parenthesis(vectors[0].clone()));
            let last_value = remove_parenthesis(vectors.pop().unwrap());
            let joltage = last_value
                .split(',')
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
            let raw_buttons: Vec<Vec<usize>> = vectors[1..]
                .to_vec()
                .iter()
                .map(|s| {
                    let clean = remove_parenthesis(s.clone());
                    clean
                        .split(',')
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            let buttons: Vec<u32> = raw_buttons
                .iter()
                .map(|vec| vector_to_bin(vec, n))
                .collect();
            Machine {
                wiring,
                raw_buttons,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn press_button(button: u32, state: u32) -> u32 {
    button ^ state
}

fn press_less_buttons(machine: &Machine, start_number: u32) -> Option<u32> {
    let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
    let mut visited: HashSet<u32> = HashSet::new();
    queue.push_back((start_number, 0));
    visited.insert(start_number);
    while let Some((current_val, count)) = queue.pop_front() {
        if current_val == machine.wiring {
            return Some(count);
        }
        for button in &machine.buttons {
            let next_val = press_button(*button, current_val);
            if !visited.contains(&next_val) {
                visited.insert(next_val);
                queue.push_back((next_val, count + 1));
            }
        }
    }
    None
}

fn press_all_machines(machines: &Vec<Machine>) -> u32 {
    machines
        .iter()
        .map(|machine| press_less_buttons(&machine, 0).unwrap())
        .sum()
}

// PART 2 https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

struct RecursiveSolver {
    buttons: Vec<Vec<usize>>,
    memo: HashMap<Vec<i64>, Option<u64>>,
}

impl RecursiveSolver {
    fn new(buttons: Vec<Vec<usize>>) -> Self {
        Self {
            buttons,
            memo: HashMap::new(),
        }
    }

    fn solve_recursive(&mut self, target: Vec<i64>) -> Option<u64> {
        if target.iter().all(|&x| x == 0) {
            return Some(0);
        }

        if let Some(&res) = self.memo.get(&target) {
            return res;
        }

        let n_buttons = self.buttons.len();
        let mut min_presses: Option<u64> = None;

        for mask in 0..(1 << n_buttons) {
            let mut current_target = target.clone();
            let mut current_cost = 0;

            for i in 0..n_buttons {
                if (mask >> i) & 1 == 1 {
                    current_cost += 1;
                    for &idx in &self.buttons[i] {
                        current_target[idx] -= 1;
                    }
                }
            }

            let is_valid = current_target.iter().all(|&x| x >= 0 && x % 2 == 0);

            if is_valid {
                let next_target: Vec<i64> = current_target.iter().map(|&x| x / 2).collect();

                if let Some(recursive_cost) = self.solve_recursive(next_target) {
                    let total = current_cost + 2 * recursive_cost;
                    min_presses = Some(match min_presses {
                        None => total,
                        Some(m) => m.min(total),
                    });
                }
            }
        }

        self.memo.insert(target, min_presses);
        min_presses
    }
}

fn press_joltages(machines: &Vec<Machine>) -> u64 {
    machines
        .iter()
        .map(|machine| {
            let target: Vec<i64> = machine.joltage.iter().map(|&x| x as i64).collect();
            let mut solver = RecursiveSolver::new(machine.raw_buttons.clone());
            solver.solve_recursive(target).unwrap_or(0)
        })
        .sum()
}

pub fn solve() -> Result<()> {
    let machines = read_file("inputs/day10.txt");
    println!("Part 1: {}", press_all_machines(&machines));
    println!("Part 2: {}", press_joltages(&machines));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10() {
        let machines = read_file("test/test_day10.txt");
        assert_eq!(press_all_machines(&machines), 7);
        assert_eq!(press_joltages(&machines), 33);
    }

}
