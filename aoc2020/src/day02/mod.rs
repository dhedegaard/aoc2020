#[derive(Debug)]
struct Line {
    from: usize,
    to: usize,
    char: String,
    password: String,
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.trim().split(' ');
            let mut intervals = parts.next().unwrap().split('-');
            let from = intervals.next().unwrap().parse::<usize>().unwrap();
            let to = intervals.next().unwrap().parse::<usize>().unwrap();
            let char = parts.next().unwrap().replace(':', "");
            let password = parts.next().unwrap().to_owned();
            Line {
                from,
                to,
                char,
                password,
            }
        })
        .collect()
}

pub fn get_input() -> String {
    include_str!("./input.txt").to_owned()
}

pub fn part1(input: &str) -> String {
    parse(input)
        .iter()
        .filter(|&line| {
            let char_count = line.password.matches(&line.char).count();
            line.from <= char_count && char_count <= line.to
        })
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    parse(input)
        .iter()
        .filter(|&line| {
            println!("{:?}", line);
            let char = line.char.chars().next().unwrap();
            println!("char: {}", char);
            let from_match = line.password.chars().nth(line.from - 1).unwrap() == char;
            println!("fm: {}", from_match);
            let to_match = line.password.chars().nth(line.to - 1).unwrap() == char;
            println!("tm: {}", to_match);
            from_match ^ to_match
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = include_str!("./test_input.txt");

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&TEST_INPUT), "2");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&get_input()), "560");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&TEST_INPUT), "1");
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&get_input()), "303");
    }
}
