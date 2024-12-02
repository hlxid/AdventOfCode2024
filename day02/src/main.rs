use aoc_utils::PuzzleInput;
const DAY: u8 = 2;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn parse_input(input: &PuzzleInput) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn is_safe_report(row: &[usize]) -> bool {
    let mut it = row.iter();

    let mut previous_value = it.next().unwrap();
    let direction = row[0] < row[1]; // true: up, false: down

    for v in it {
        let distance = v.abs_diff(*previous_value);
        let distance_mistake = !(1..=3).contains(&distance);
        let direction_mistake =
            (v > previous_value && !direction) || (v < previous_value && direction);

        if distance_mistake || direction_mistake {
            return false;
        }

        previous_value = v;
    }

    true
}

fn solve(input: &PuzzleInput, allow_single_bad_row: bool) -> usize {
    let input = parse_input(input);

    input
        .iter()
        .filter(|row| {
            if is_safe_report(row) {
                return true;
            }

            if allow_single_bad_row {
                for drop_i in 0..row.len() {
                    let mut row_clone = row.to_vec();
                    row_clone.remove(drop_i);
                    if is_safe_report(&row_clone) {
                        return true;
                    }
                }
            }

            false
        })
        .count()
}

fn solve_a(input: &PuzzleInput) -> usize {
    solve(input, false)
}

fn solve_b(input: &PuzzleInput) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 2);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 4);
    }
}
