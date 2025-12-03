use std::fs;

// Main function
pub fn day_03() {
    let input_file = "./input/day_03.txt";

    println!("Puzzle 1 result: {}", puzzle1(input_file));
    println!("Puzzle 2 result: {}", puzzle2(input_file));
}

// Puzzle 1 function
fn puzzle1(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    input
        .lines()
        .map(|line| line_to_digits_vec(line))
        .map(|list| find_largest_digit_num(list, 2))
        .sum()
}

// Puzzle 2 function
fn puzzle2(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    input
        .lines()
        .map(|line| line_to_digits_vec(line))
        .map(|list| find_largest_digit_num(list, 12))
        .sum()
}

fn find_largest_digit_num(nums: Vec<usize>, digits: u32) -> usize {
    let mut cur_highest_index = nums
        .iter()
        .enumerate()
        .rev()
        .skip((digits - 1).try_into().unwrap())
        .max_by_key(|(_, value)| *value)
        .unwrap();

    let mut total = cur_highest_index.1 * 10usize.pow(digits - 1);
    for i in 1..digits {
        let multiplier = 10usize.pow(digits - 1 - i);

        let next_highest_index = nums
            .iter()
            .enumerate()
            .rev()
            .skip((digits - 1 - i).try_into().unwrap())
            .rev()
            .skip(cur_highest_index.0 + 1)
            .rev()
            .max_by_key(|(_, value)| *value)
            .unwrap();
        total += next_highest_index.1 * multiplier;
        cur_highest_index = next_highest_index;
    }

    total
}

fn line_to_digits_vec(line: &str) -> Vec<usize> {
    line.chars()
        .map(|char| char.to_digit(10).unwrap().try_into().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_num() {
        assert_eq!(
            find_largest_digit_num(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2),
            98
        );
        assert_eq!(
            find_largest_digit_num(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2),
            89
        );
        assert_eq!(
            find_largest_digit_num(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2),
            78
        );
        assert_eq!(
            find_largest_digit_num(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2),
            92
        );
        assert_eq!(
            find_largest_digit_num(
                vec![
                    7, 2, 2, 5, 2, 7, 1, 2, 3, 4, 7, 2, 4, 2, 1, 2, 1, 4, 4, 3, 6, 2, 4, 1, 1, 2,
                    2, 3, 5, 3, 1, 2, 2, 1, 4, 1, 2, 2, 6, 4, 2, 6, 7, 2, 5, 5, 2, 4, 2, 2, 4, 2,
                    2, 2, 6, 5, 2, 6, 2, 6, 3, 6, 7, 4, 4, 7, 7, 4, 2, 2, 4, 3, 6, 2, 4, 6, 4, 1,
                    5, 6, 2, 5, 2, 2, 3, 8, 1, 1, 3, 8, 4, 5, 2, 3, 5, 2, 3, 5, 2, 7
                ],
                2
            ),
            88
        );
    }

    #[test]
    fn test_find_largest_twelve_digit_num() {
        assert_eq!(
            find_largest_digit_num(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
        assert_eq!(
            find_largest_digit_num(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
            811111111119
        );
        assert_eq!(
            find_largest_digit_num(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
            434234234278
        );
        assert_eq!(
            find_largest_digit_num(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
            888911112111
        );
    }

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("./input_test/day_03.txt"), 357);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("./input_test/day_03.txt"), 3121910778619);
    }
}
