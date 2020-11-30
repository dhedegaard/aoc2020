pub fn part1(input: &str) -> String {
    input.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_input() {
        assert_eq!(part1(&"test"), "test");
    }
}
