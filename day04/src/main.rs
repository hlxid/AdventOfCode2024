use aoc_utils::PuzzleInput;
const DAY: u8 = 4;

fn main() {
    let input = PuzzleInput::get_input(DAY);
    println!("A: {}", solve_a(&input));
    println!("B: {}", solve_b(&input));
}

fn check_xmas(x: usize, y: usize, map: &[Vec<char>], offset: (i8, i8)) -> bool {
    let mut chars = vec![map[y][x]];
    let (mut x, mut y) = (x as isize, y as isize);
    for _ in 1..4 {
        x += offset.0 as isize;
        y += offset.1 as isize;
        // out of bounds checks
        if x < 0 || x >= map[0].len() as isize {
            return false;
        }
        if y < 0 || y >= map.len() as isize {
            return false;
        }

        chars.push(map[y as usize][x as usize]);
    }

    chars == vec!['X', 'M', 'A', 'S']
}

fn check_mas(x: usize, y: usize, map: &[Vec<char>], direction: (i8, i8)) -> bool {
    let (x, y) = (x as isize, y as isize);
    map[(y + direction.1 as isize) as usize][(x + direction.0 as isize) as usize] == 'M'
        && map[y as usize][x as usize] == 'A'
        && map[(y - direction.1 as isize) as usize][(x - direction.0 as isize) as usize] == 'S'
}

fn check_x_dash_mas(x: usize, y: usize, map: &[Vec<char>]) -> bool {
    let mut counter = 0;

    for x_offset in -1..=1 {
        for y_offset in -1..=1 {
            if x_offset != 0 && y_offset != 0 && check_mas(x, y, map, (x_offset, y_offset)) {
                counter += 1;
            }
        }
    }

    counter == 2
}

fn solve_a(input: &PuzzleInput) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut occurences = 0;
    let (rows, columns) = (map.len(), map[0].len());

    for y in 0..rows {
        for x in 0..columns {
            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    if (x_offset != 0 || y_offset != 0)
                        && check_xmas(x, y, &map, (x_offset, y_offset))
                    {
                        occurences += 1;
                    }
                }
            }
        }
    }

    occurences
}

fn solve_b(input: &PuzzleInput) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut occurences = 0;
    let (rows, columns) = (map.len(), map[0].len());

    for y in 1..(rows - 1) {
        for x in 1..(columns - 1) {
            if check_x_dash_mas(x, y, &map) {
                occurences += 1;
            }
        }
    }

    occurences
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_no_panic() {
        let input = PuzzleInput::get_input(DAY);
        solve_a(&input);
        solve_b(&input);
    }

    #[test]
    fn test_solve_a() {
        assert_eq!(solve_a(&PuzzleInput::new(TEST_INPUT)), 18);
    }

    #[test]
    fn test_solve_b() {
        assert_eq!(solve_b(&PuzzleInput::new(TEST_INPUT)), 9);
    }
}
