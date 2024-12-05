use aoc_utils::PuzzleInput;
const DAY: u8 = 5;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Clone)]
struct PageOrderingRule {
    lower_page_num: usize,
    higher_page_num: usize,
}

impl PageOrderingRule {
    fn parse(s: &str) -> Self {
        let mut parts = s.split("|");
        Self {
            lower_page_num: parts.next().unwrap().parse().unwrap(),
            higher_page_num: parts.next().unwrap().parse().unwrap(),
        }
    }
}

struct PageUpdate {
    numbers: Vec<usize>,
}

impl PageUpdate {
    fn parse(s: &str) -> Self {
        Self {
            numbers: s.split(",").map(|n| n.parse().unwrap()).collect(),
        }
    }

    fn get_update_relevant_rules(&self, rules: &[PageOrderingRule]) -> Vec<PageOrderingRule> {
        rules
            .iter()
            .filter(|r| {
                self.numbers.contains(&r.lower_page_num)
                    && self.numbers.contains(&r.higher_page_num)
            })
            .cloned()
            .collect()
    }

    fn is_rule_followed(&self, rule: &PageOrderingRule) -> bool {
        self.numbers.iter().position(|n| *n == rule.lower_page_num)
            < self.numbers.iter().position(|n| *n == rule.higher_page_num)
    }

    fn are_all_rules_followed(&self, rules: &[PageOrderingRule]) -> bool {
        self.get_update_relevant_rules(rules)
            .iter()
            .all(|rule| self.is_rule_followed(rule))
    }

    fn follow_rules(&mut self, rules: &[PageOrderingRule]) {
        let relevant_rules = self.get_update_relevant_rules(rules);
        while !self.are_all_rules_followed(&relevant_rules) {
            for rule in relevant_rules.iter() {
                if !self.is_rule_followed(rule) {
                    let lower_expected_pos = self
                        .numbers
                        .iter()
                        .position(|n| *n == rule.lower_page_num)
                        .unwrap();
                    let higher_expected_pos = self
                        .numbers
                        .iter()
                        .position(|n| *n == rule.higher_page_num)
                        .unwrap();

                    self.numbers.remove(lower_expected_pos);
                    self.numbers
                        .insert(higher_expected_pos, rule.lower_page_num);
                }
            }
        }
    }

    fn get_middle_number(&self) -> usize {
        self.numbers[self.numbers.len() / 2]
    }
}

fn parse_inputs(input: &PuzzleInput) -> (Vec<PageOrderingRule>, Vec<PageUpdate>) {
    let mut input_parts = input.raw_input.split("\n\n");
    let rules = input_parts
        .next()
        .unwrap()
        .split("\n")
        .map(PageOrderingRule::parse)
        .collect();
    let page_updates = input_parts
        .next()
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(PageUpdate::parse)
        .collect();
    (rules, page_updates)
}

fn solve_a(input: &PuzzleInput) -> usize {
    let (rules, page_updates) = parse_inputs(input);
    page_updates
        .iter()
        .filter(|update| update.are_all_rules_followed(&rules))
        .map(|update| update.get_middle_number())
        .sum()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let (rules, mut page_updates) = parse_inputs(input);
    let mut invalid_page_updates: Vec<&mut PageUpdate> = page_updates
        .iter_mut()
        .filter(|update| !update.are_all_rules_followed(&rules))
        .collect();
    for page_update in invalid_page_updates.iter_mut() {
        page_update.follow_rules(&rules);
    }
    invalid_page_updates
        .iter()
        .map(|update| update.get_middle_number())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 143);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 123);
    }
}
