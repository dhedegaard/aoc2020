use std::collections::HashSet;

pub fn get_input() -> String {
    include_str!("./input.txt").to_owned()
}

pub fn part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| c.is_alphabetic())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            let mut questions = HashSet::new();
            group.lines().enumerate().for_each(|(index, line)| {
                if index == 0 {
                    line.chars().for_each(|c| {
                        &questions.insert(c);
                    });
                } else {
                    let chars = line.chars().collect::<HashSet<_>>();
                    questions.retain(|c| chars.contains(c));
                }
            });
            questions.len()
        })
        .sum::<usize>()
        .to_string()
}

mod tests {
    const TEST_INPUT: &str = include_str!("./test_input.txt");

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&TEST_INPUT), "11");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&get_input()), "6686");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&TEST_INPUT), "6");
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&get_input()), "3476");
    }
}
