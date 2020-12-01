fn parse(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|e| e.trim().parse().unwrap())
        .collect()
}

pub fn get_input() -> String {
    include_str!("./input.txt").to_owned()
}

pub fn part1(input: &str) -> String {
    let numbers = parse(input);
    for e in &numbers {
        for f in &numbers {
            if e + f == 2020 {
                return (e * f).to_string();
            }
        }
    }
    unreachable!("No valid solution");
}

pub fn part2(input: &str) -> String {
    let numbers = parse(input);
    for e in &numbers {
        for f in &numbers {
            for g in &numbers {
                if e + f + g == 2020 {
                    return (e * f * g).to_string();
                }
            }
        }
    }
    unreachable!("No valid solution");
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test_input.txt");

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST_INPUT), "514579");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&get_input()), "889779");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT), "241861950");
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&get_input()), "76110336");
    }
}
