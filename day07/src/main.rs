use aoc_utils::PuzzleInput;
use itertools::Itertools;
const DAY: u8 = 7;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

struct Calculation {
    expected_output: u64,
    inputs: Vec<u64>,
}

impl Calculation {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(": ");
        Self {
            expected_output: parts.next().unwrap().parse().unwrap(),
            inputs: parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|p| p.parse().unwrap())
                .collect(),
        }
    }

    fn can_be_calculated_from_inputs(&self, include_concat: bool) -> bool {
        Calculation::possible_outputs(&self.inputs, include_concat).contains(&self.expected_output)
    }

    fn execute_operator(op_id: usize, input_a: u64, input_b: u64) -> u64 {
        match op_id {
            0 => input_a + input_b,
            1 => input_a * input_b,
            2 => (input_a.to_string() + &input_b.to_string())
                .parse()
                .unwrap(),
            _ => panic!("invalid op"),
        }
    }

    fn possible_outputs(values: &[u64], include_concat: bool) -> Vec<u64> {
        let max_op_id = if include_concat { 2 } else { 1 };
        let op_sequences: Vec<Vec<usize>> = (1..values.len())
            .map(|_| (0..=max_op_id))
            .multi_cartesian_product()
            .collect();

        let mut possible_results = vec![];

        for op_seq in op_sequences {
            let mut current_value = values[0];
            for (i, op) in op_seq.iter().enumerate() {
                current_value = Calculation::execute_operator(*op, current_value, values[i+1]);
            }
            possible_results.push(current_value);
        }

        possible_results
    }
}

fn solve(input: &PuzzleInput, include_concat: bool) -> u64 {
    input
        .lines()
        .map(|l| Calculation::parse(&l))
        .filter(|c| c.can_be_calculated_from_inputs(include_concat))
        .map(|c| c.expected_output)
        .sum()
}

fn solve_a(input: &PuzzleInput) -> u64 {
    solve(input, false)
}

fn solve_b(input: &PuzzleInput) -> u64 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        // solve_b(&input); too slow
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 3749);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 11387);
    }
}
