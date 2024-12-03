use aoc_utils::PuzzleInput;
use regex::Regex;

const DAY: u8 = 3;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn calculate_score(input: &PuzzleInput, do_dont_enabled: bool) -> u32 {
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do\\(\\)()()|don't\\(\\)()()").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for m in re.captures_iter(&input.raw_input) {
        let (match_str, [num_1, num_2]): (&str, [&str; 2]) = m.extract();
        if match_str.starts_with("do(") {
            enabled = true;
        } else if match_str.starts_with("don't(") {
            enabled = false;
        } else if enabled || !do_dont_enabled {
            sum += num_1.parse::<u32>().unwrap() * num_2.parse::<u32>().unwrap();
        }
    }

    sum
}

fn solve_a(input: &PuzzleInput) -> u32 {
    calculate_score(input, false)
}

fn solve_b(input: &PuzzleInput) -> u32 {
    calculate_score(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_A: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_B: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT_A)), 161);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT_B)), 48);
    }
}
