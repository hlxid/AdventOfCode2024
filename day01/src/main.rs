use aoc_utils::PuzzleInput;
const DAY: u8 = 1;

struct ListPair {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl ListPair {
    fn parse(input: &PuzzleInput) -> Self {
        let pairs: Vec<_> = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let mut parts = l.split_whitespace();
                let left_num = parts.next().unwrap().parse().unwrap();
                let right_num = parts.next().unwrap().parse().unwrap();
                (left_num, right_num)
            })
            .collect();
        let mut left: Vec<_> = pairs.iter().map(|p| p.0).collect();
        let mut right: Vec<_> = pairs.iter().map(|p| p.1).collect();

        left.sort();
        right.sort();

        Self { left, right }
    }

    fn score_a(&self) -> u64 {
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(a, b)| if a > b { a - b } else { b - a })
            .sum()
    }

    fn score_b(&self) -> u64 {
        self.left
            .iter()
            .map(|l_num| {
                let occurence = self.right.iter().filter(|r_num| l_num == *r_num).count() as u64;
                *l_num * occurence
            })
            .sum()
    }
}

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn solve_a(input: &PuzzleInput) -> u64 {
    ListPair::parse(input).score_a()
}

fn solve_b(input: &PuzzleInput) -> u64 {
    ListPair::parse(input).score_b()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 11);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 31);
    }
}
