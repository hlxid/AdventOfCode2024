use aoc_utils::PuzzleInput;
use regex::Regex;

const DAY: u8 = 3;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

struct MulInstruction(i32, i32);

fn parse_instructions(input: &PuzzleInput, do_dont_enabled: bool) -> Vec<MulInstruction> {
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do\\(\\)()()|don't\\(\\)()()").unwrap();
    let mut instrs = vec![];
    let mut enabled = true;

    for m in re.captures_iter(&input.raw_input) {
        let (match_str, captures): (&str, [&str; 2]) = m.extract();
        if match_str.starts_with("do(") {
            enabled = true;
        } else if match_str.starts_with("don't(") {
            enabled = false;
        } else {
            if enabled || !do_dont_enabled {
                instrs.push(MulInstruction(
                    captures.get(0).unwrap().parse().unwrap(),
                    captures.get(1).unwrap().parse().unwrap(),
                ));
            }
        }
    }

    instrs
}

fn solve_a(input: &PuzzleInput) -> i32 {
    parse_instructions(input, false)
        .iter()
        .map(|i| i.0 * i.1)
        .sum()
}

fn solve_b(input: &PuzzleInput) -> i32 {
    parse_instructions(input, true)
        .iter()
        .map(|i| i.0 * i.1)
        .sum()
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
