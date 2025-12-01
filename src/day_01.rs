use std::fs;

// Main function
pub fn day_01() {
    let input_file = "./input/day_01.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let ops: (usize, usize) = input
        .lines()
        .map(|line| {
            let (dir, count) = line.split_at(1);
            (dir.parse().unwrap(), count.parse().unwrap())
        })
        .fold((50, 0), |(cur_pos, count), op| {
            let res = exec_op(cur_pos, op) % 100;
            if res == 0 {
                return (res, count + 1);
            }
            (res, count)
        });
    ops.1
}

fn exec_op(loc: usize, op: (char, usize)) -> usize {
    let (dir, count) = op;
    let count = count % 100;
    match dir {
        'R' => loc + count % 100,
        'L' => loc.checked_sub(count).map_or(loc + 100 - count, |l| l),
        _ => panic!("Matched dir that is not R or L"),
    }
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");

    let ops: (usize, usize) = input
        .lines()
        .map(|line| {
            let (dir, count) = line.split_at(1);
            (dir.parse().unwrap(), count.parse().unwrap())
        })
        .fold((50, 0), |(cur_pos, count), op| {
            let (res, extra_count) = exec_op_count_zeroes(cur_pos, op);
            (res, count + extra_count)
        });
    ops.1
}

fn exec_op_count_zeroes(loc: usize, op: (char, usize)) -> (usize, usize) {
    let (dir, count) = op;
    let mut zeroes = count / 100;
    let rest = count % 100;
    let res = match dir {
        'R' => {
            let total = loc + rest;
            if total > 100 {
                zeroes += 1;
            }
            total % 100
        }
        'L' => {
            if rest > loc && rest > 0 && loc != 0 {
                zeroes += 1
            }
            (loc + (zeroes + 1) * 100 - count) % 100
        }
        _ => panic!("Matched dir that is not R or L"),
    };
    if res == 0 {
        zeroes += 1;
    }
    (res, zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_01.txt"), 3);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_01.txt"), 6);
        assert_eq!(puzzle2("./input/day_01.txt"), 5961);
    }
}
