use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Graph {
    adjacency: HashMap<String, Vec<String>>,
    cache: HashMap<String, u64>,
    cond_cache: HashMap<(String, u8), u64>,
}

impl Graph {
    fn new(edges: HashMap<String, Vec<String>>) -> Self {
        Self {
            adjacency: edges,
            cache: HashMap::new(),
            cond_cache: HashMap::new(),
        }
    }

    fn count_paths(&mut self, current: String, end: String) -> u64 {
        if current == end {
            return 1;
        }

        if let Some(&count) = self.cache.get(&current) {
            return count;
        }

        let mut count = 0;
        if let Some(neighbors) = self.adjacency.get(&current).cloned() {
            for neighbor in neighbors {
                count += self.count_paths(neighbor, end.clone());
            }
        }
        self.cache.insert(current, count);
        count
    }

    fn count_paths_with_conditions(&mut self, current: String, end: String, mask: u8) -> u64 {
        self.cache.clear();
        let mut new_mask = mask;
        if current == "dac" {
            new_mask |= 1;
        } else if current == "fft" {
            new_mask |= 2;
        }

        if current == end {
            return if new_mask == 3 { 1 } else { 0 };
        }

        let state_key = (current.clone(), new_mask);
        if let Some(&count) = self.cond_cache.get(&state_key) {
            return count;
        }

        let mut count = 0;
        if let Some(neighbors) = self.adjacency.get(&current).cloned() {
            for neighbor in neighbors {
                count += self.count_paths_with_conditions(neighbor, end.clone(), new_mask);
            }
        }
        self.cond_cache.insert(state_key, count);
        count
    }
}

fn read_file(path: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    reader.lines().for_each(|line| {
        // CAMBIO: map -> for_each
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(":").collect();
        let key = parts[0].trim().to_string();
        let raw = parts[1].trim().to_string();
        let value: Vec<String> = raw.split_whitespace().map(|s| s.to_string()).collect();

        map.insert(key, value);
    });
    map
}

pub fn solve() -> Result<()> {
    let map: HashMap<String, Vec<String>> = read_file("inputs/day11.txt");
    let mut graph = Graph::new(map);
    println!(
        "Part 1: {}",
        graph.count_paths("you".to_string(), "out".to_string())
    );
    println!(
        "Part 2: {}",
        graph.count_paths_with_conditions("svr".to_string(), "out".to_string(), 0)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::days::day11::{read_file, Graph};
    use std::collections::HashMap;

    #[test]
    fn test_day11() {
        let map: HashMap<String, Vec<String>> = read_file("test/test_day11.txt");
        let mut graph = Graph::new(map);
        assert_eq!(graph.count_paths("you".to_string(), "out".to_string()), 5);

        let map2: HashMap<String, Vec<String>> = read_file("test/test_day11-b.txt");
        let mut graph2 = Graph::new(map2);
        assert_eq!(
            graph2.count_paths_with_conditions("svr".to_string(), "out".to_string(), 0),
            2
        );
    }
}
