use aoc_utils::PuzzleInput;
use std::collections::HashSet;
const DAY: u8 = 6;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

#[derive(Eq, PartialEq, Clone)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::UP,
            '>' => Self::RIGHT,
            'v' => Self::DOWN,
            '<' => Self::LEFT,
            _ => panic!("Invalid direction char: {c}"),
        }
    }

    fn is_direction_char(c: char) -> bool {
        c == '^' || c == '>' || c == 'v' || c == '<'
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

#[derive(Clone)]
struct GuardMap {
    map: Vec<Vec<bool>>,
    guard_position: (usize, usize),
    guard_direction: Direction,
}

impl GuardMap {
    fn parse(input: &PuzzleInput) -> Self {
        let mut guard_position = (0, 0);
        let mut guard_direction = Direction::UP;

        let map = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if Direction::is_direction_char(c) {
                            guard_position = (x, y);
                            guard_direction = Direction::from_char(c);
                        }
                        c == '#'
                    })
                    .collect()
            })
            .collect();

        Self {
            map,
            guard_position,
            guard_direction,
        }
    }

    fn determine_next_position(&self) -> (usize, usize) {
        match self.guard_direction {
            Direction::UP => (self.guard_position.0, self.guard_position.1 - 1),
            Direction::RIGHT => (self.guard_position.0 + 1, self.guard_position.1),
            Direction::DOWN => (self.guard_position.0, self.guard_position.1 + 1),
            Direction::LEFT => (self.guard_position.0 - 1, self.guard_position.1),
        }
    }

    fn next_walk_out_of_map(&self) -> bool {
        self.guard_direction == Direction::UP && self.guard_position.1 == 0
            || self.guard_direction == Direction::RIGHT
                && self.guard_position.0 == self.map[0].len() - 1
            || self.guard_direction == Direction::DOWN
                && self.guard_position.1 == self.map.len() - 1
            || self.guard_direction == Direction::LEFT && self.guard_position.0 == 0
    }

    fn walk(&mut self) {
        self.guard_position = self.determine_next_position();
    }

    fn rotate_right_if_something_in_front(&mut self) -> bool {
        if !self.next_walk_out_of_map() {
            let next_position = self.determine_next_position();
            if self.map[next_position.1][next_position.0] {
                // next move would be into a obstacle, turn right
                self.guard_direction = self.guard_direction.turn_right();
                return true;
            }
        }
        false
    }

    fn walk_till_out_of_map(&mut self) -> usize {
        let mut visited_positions = HashSet::new();
        loop {
            if self.next_walk_out_of_map() {
                break;
            }

            self.walk();
            while self.rotate_right_if_something_in_front() {}
            visited_positions.insert(self.guard_position);
        }

        visited_positions.len()
    }

    fn contains_loop(&mut self) -> bool {
        let mut steps = vec![];
        loop {
            if self.next_walk_out_of_map() {
                return false;
            }

            self.walk();
            let step_pos = (
                self.guard_position.0,
                self.guard_position.1,
                self.guard_direction.clone(),
            );

            if steps.contains(&step_pos) {
                return true;
            }

            steps.push(step_pos);
            while self.rotate_right_if_something_in_front() {
                let step_pos = (
                    self.guard_position.0,
                    self.guard_position.1,
                    self.guard_direction.clone(),
                );

                if steps.contains(&step_pos) {
                    return true;
                }

                steps.push(step_pos);
            }
        }
    }

    fn obstacle_positions_causing_loop(&mut self) -> usize {
        let mut positions = 0;
        for y in 0..self.map.len() {
            println!("y: {y}/{}", self.map.len());
            for x in 0..self.map[0].len() {
                let pos = (x, y);
                if pos == self.guard_position || self.map[y][x] {
                    continue;
                }

                let mut m = self.clone();
                m.map[y][x] = true;
                if m.contains_loop() {
                    positions += 1
                }
            }
        }

        positions
    }
}

fn solve_a(input: &PuzzleInput) -> usize {
    let mut map = GuardMap::parse(input);
    map.walk_till_out_of_map()
}

fn solve_b(input: &PuzzleInput) -> usize {
    let mut map = GuardMap::parse(input);
    map.obstacle_positions_causing_loop()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        // solve_b(&input); too slow
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 41);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 6);
    }
}
