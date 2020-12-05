pub fn get_input() -> String {
    include_str!("./input.txt").to_string()
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut rows = [0, 127];
            let mut columns = [0, 7];
            line.chars().for_each(|c| {
                println!("rows: {:?} -- columns: {:?}", rows, columns);
                match c {
                    'B' => rows[0] += (rows[1] - rows[0] + 1) / 2,
                    'F' => rows[1] -= (rows[1] - rows[0] + 1) / 2,
                    'R' => columns[0] += (columns[1] - columns[0] + 1) / 2,
                    'L' => columns[1] -= (columns[1] - columns[0] + 1) / 2,
                    _ => unreachable!(),
                }
            });
            rows[0] * 8 + columns[0]
        })
        .max()
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
}
