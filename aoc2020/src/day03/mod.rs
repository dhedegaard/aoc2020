pub fn get_input() -> String {
    include_str!("./input.txt").to_owned()
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn solve(grid: &[Vec<bool>], delta_x: usize, delta_y: usize) -> usize {
    (0..grid.len())
        .step_by(delta_y)
        .filter(|&step| grid[step][(step / delta_y * delta_x) % grid[step].len()])
        .count()
}

pub fn part1(input: &str) -> String {
    solve(&parse_input(input), 3, 1).to_string()
}

pub fn part2(input: &str) -> String {
    let grid = parse_input(input);
    [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        .iter()
        .map(|&[dx, dy]| solve(&grid, dx, dy))
        .fold(1, |acc, e| acc * e)
        .to_string()
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&TEST_INPUT), "336");
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&get_input()), "3952291680")
    }
}
