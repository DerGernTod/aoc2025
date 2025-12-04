use std::{collections::HashSet, fs};

// Main function
pub fn day_04() {
    let input_file = "./input/day_04.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let entry_set = collect_entries(input);
    entry_set
        .iter()
        .filter(|coords| has_less_neighbors(&entry_set, coords, 4))
        .count()
}

fn collect_entries(input: String) -> HashSet<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'@')
                .map(move |(x, _)| (x.try_into().unwrap(), y.try_into().unwrap()))
        })
        .collect()
}

fn has_less_neighbors(
    entry_set: &HashSet<(i32, i32)>,
    coords: &(i32, i32),
    max_neighbors: usize,
) -> bool {
    let (x, y) = coords;
    let neighbor_count = (-1..=1)
        .flat_map(|val_x| (-1..=1).map(move |val_y| (val_x + x, val_y + y)))
        .filter(|check_coords| !is_same_coord(check_coords, coords))
        .filter(|check_coords| entry_set.contains(check_coords))
        .count();
    neighbor_count < max_neighbors
}

fn is_same_coord(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let (val_x, val_y) = a;
    let (x, y) = b;
    val_x == x && val_y == y
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let mut entry_set = collect_entries(input);
    let initial_count = entry_set.len();
    loop {
        let to_remove: HashSet<(i32, i32)> = entry_set
            .iter()
            .filter(|coords| has_less_neighbors(&entry_set, coords, 4))
            .map(|coords| *coords)
            .collect();
        if to_remove.len() == 0 {
            break;
        }
        entry_set = entry_set.difference(&to_remove).copied().collect();
    }
    initial_count - entry_set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_04.txt"), 13);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_04.txt"), 43);
    }
}
