use std::collections::BTreeSet;

pub fn get_input() -> String {
    include_str!("./input.txt").to_string()
}

fn lineToSeatId(line: &str) -> i32 {
    let mut rows = [0, 127];
    let mut columns = [0, 7];
    line.chars().for_each(|c| match c {
        'B' => rows[0] += (rows[1] - rows[0] + 1) / 2,
        'F' => rows[1] -= (rows[1] - rows[0] + 1) / 2,
        'R' => columns[0] += (columns[1] - columns[0] + 1) / 2,
        'L' => columns[1] -= (columns[1] - columns[0] + 1) / 2,
        _ => unreachable!(),
    });
    rows[0] * 8 + columns[0]
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| lineToSeatId(line))
        .max()
        .unwrap()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let seat_ids = input
        .lines()
        .map(|line| lineToSeatId(line))
        .collect::<BTreeSet<_>>();
    (*seat_ids.iter().min().unwrap()..*seat_ids.iter().max().unwrap())
        .find(|&seat_id| !seat_ids.contains(&seat_id))
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&"FBFBBFFRLR"), "357");
        assert_eq!(part1(&"BFFFBBFRRR"), "567");
        assert_eq!(part1(&"FFFBBBFRRR"), "119");
        assert_eq!(part1(&"BBFFBBFRLL"), "820");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&get_input()), "976");
    }

    #[test]
    fn part2_result() {
        assert_eq!(part2(&get_input()), "685");
    }
}
