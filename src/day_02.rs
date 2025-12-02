use std::fs;

// Main function
pub fn day_02() {
    let input_file = "./input/day_02.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    input
        .split(",")
        .map(|spl| {
            let from_to: Vec<&str> = spl.split("-").collect();

            let from_chars = from_to.get(0).unwrap();
            let to_chars = from_to.get(1).unwrap();
            sum_double_sequence_occurrences(&from_chars, &to_chars)
        })
        .sum()
}

fn sum_double_sequence_occurrences(from: &str, to: &str) -> usize {
    let num_from: usize = from.parse().unwrap();
    let num_to: usize = to.parse().unwrap();

    let mut cur_num = num_from;
    let mut invalid_cnt = 0;

    loop {
        let cur_digits_count = digits(cur_num);
        if cur_digits_count % 2 != 0 {
            cur_num = (10usize).pow((cur_digits_count).try_into().unwrap());
            continue;
        }
        let divisor = (10usize).pow((cur_digits_count / 2).try_into().unwrap());
        let left_digits = cur_num % divisor;
        let right_digits = cur_num / divisor;
        if left_digits == right_digits {
            invalid_cnt += cur_num;
        }
        cur_num += 1;
        if cur_num > num_to {
            break;
        }
    }

    invalid_cnt
}

fn digits(num: usize) -> usize {
    if num >= 10 {
        digits(num / 10) + 1
    } else {
        1
    }
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    input
        .split(",")
        .map(|spl| {
            let from_to: Vec<&str> = spl.split("-").collect();

            let from_chars = from_to.get(0).unwrap();
            let to_chars = from_to.get(1).unwrap();
            sum_multi_sequence_occurrences(&from_chars, &to_chars)
        })
        .sum()
}

fn sum_multi_sequence_occurrences(from: &str, to: &str) -> usize {
    let num_from: usize = from.parse().unwrap();
    let num_to: usize = to.parse().unwrap();

    let mut cur_num = num_from;
    let mut invalid_cnt = 0;

    loop {
        if is_invalid(cur_num) {
            invalid_cnt += cur_num;
        }
        cur_num += 1;
        if cur_num > num_to {
            break;
        }
    }

    invalid_cnt
}

fn pow(num: usize, exp: usize) -> usize {
    num.pow(exp.try_into().unwrap())
}

fn is_invalid(cur_num: usize) -> bool {
    let cur_digits = digits(cur_num);
    for i in 0..cur_digits {
        let sequence = cur_num % pow(10, i + 1);
        if digits(sequence) > cur_digits / 2 {
            return false;
        }
        if check_sequence(cur_num, sequence) {
            return true;
        }
    }
    false
}

fn check_sequence(num: usize, sequence: usize) -> bool {
    let sequence_digits = digits(sequence);
    let num_digits = digits(num);
    if num_digits % sequence_digits != 0 || num_digits == sequence_digits {
        return false;
    }
    let num_sequence_occurrences = num_digits / sequence_digits;
    for i in 0..num_sequence_occurrences {
        let num_part = num / pow(10, sequence_digits * i);
        let seq_extract = num_part % pow(10, sequence_digits);
        if seq_extract != sequence {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_02.txt"), 1227775554);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_02.txt"), 4174379265);
    }

    #[test]
    fn test_check_sequence() {
        assert_eq!(check_sequence(123456, 56), false);
        assert_eq!(check_sequence(565656, 56), true);
        assert_eq!(check_sequence(565655, 56), false);
        assert_eq!(check_sequence(5565656, 56), false);
        assert_eq!(check_sequence(55, 5), true);
    }

    #[test]
    fn test_is_invalid() {
        assert_eq!(is_invalid(123456), false);
        assert_eq!(is_invalid(565656), true);
        assert_eq!(is_invalid(565655), false);
        assert_eq!(is_invalid(5565656), false);
        assert_eq!(is_invalid(55), true);
    }
}
