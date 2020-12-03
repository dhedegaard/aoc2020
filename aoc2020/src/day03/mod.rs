use std::num::ParseIntError;

pub fn get_input() -> String {
    include_str!("./input.txt").to_owned()
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn part1(input: &str) -> String {
    let grid = parse_input(input);
    let mut count = 0;
    for y in 0..grid.len() {
        if grid[y][(y * 3) % grid[y].len()] {
            count += 1;
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test_input.txt");

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&TEST_INPUT), "7");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&get_input()), "232");
    }
}
