use std::collections::HashMap;

type Passport = HashMap<String, String>;

const FIELDS: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

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
        .filter(|&passport| {
            !FIELDS
                .iter()
                .filter(|&&e| e != "cid")
                .any(|&field| !passport.contains_key(field))
        })
        .count()
        .to_string()
}

fn part2_passport_valid(passport: &Passport) -> bool {
    !FIELDS.iter().any(|&field| {
        if field == "cid" {
            return !true;
        }
        if !passport.contains_key(field) {
            return !false;
        }
        println!("{:?}", passport);
        println!("field: {}", field);
        let value = passport.get(field).unwrap();
        !match field {
            "cid" => true,
            "byr" => {
                let birthyear = value.parse::<i32>().unwrap();
                birthyear >= 1920 && birthyear <= 2002
            }
            "iyr" => {
                let issueyear = value.parse::<i32>().unwrap();
                issueyear >= 2010 && issueyear <= 2020
            }
            "eyr" => {
                let expirationyear = value.parse::<i32>().unwrap();
                expirationyear >= 2020 && expirationyear <= 2030
            }
            "hgt" => {
                let height = value[..value.len() - 2].parse::<i32>().unwrap();
                match &value[(value.len() - 2)..] {
                    "cm" => height >= 150 && height <= 193,
                    "in" => height >= 59 && height <= 76,
                    _ => false,
                }
            }
            "hcl" => {
                value.chars().next().unwrap() == '#'
                    && value.len() == 7
                    && value[1..].chars().all(|c| c.is_digit(16))
            }
            "ecl" => EYE_COLORS.contains(&value.as_str()),
            "pid" => value.len() == 9 && !value.chars().any(|c| !c.is_digit(10)),

            _ => unreachable!("Unhandled case !"),
        }
    })
}

pub fn part2(input: &str) -> String {
    parse_input(input)
        .iter()
        .filter(|&passport| part2_passport_valid(passport))
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

    #[test]
    fn part2_invalid_passports() {
        let passports = parse_input(include_str!("./part2_invalid_passports.txt"));
        assert_eq!(passports.len(), 4);
        assert_eq!(
            passports
                .iter()
                .filter(|passport| part2_passport_valid(passport))
                .count(),
            0
        )
    }

    #[test]
    fn part2_valid_passport_examples() {
        let passports = parse_input(include_str!("./part2_valid_passports.txt"));
        assert_eq!(passports.len(), 4);
        assert_eq!(
            passports
                .iter()
                .filter(|passport| part2_passport_valid(passport))
                .count(),
            passports.len()
        );
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&get_input()), "147");
    }
}
