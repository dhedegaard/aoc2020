use std::collections::HashMap;

type Passport = HashMap<String, String>;

const FIELDS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

pub fn get_input() -> String {
    include_str!("./input.txt").to_owned()
}

fn parse_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|lines| {
            let mut passport = Passport::new();

            lines.split_whitespace().for_each(|e| {
                let mut iter = e.split(":").take(2).map(|e| e.to_owned());
                passport.insert(iter.next().unwrap(), iter.next().unwrap());
            });

            passport
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    parse_input(input)
        .iter()
        .filter(|&password| {
            !FIELDS
                .iter()
                .filter(|&&e| e != "cid")
                .any(|&field| !password.contains_key(field))
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
        assert_eq!(part1(TEST_INPUT), "2");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&get_input()), "213");
    }
}
