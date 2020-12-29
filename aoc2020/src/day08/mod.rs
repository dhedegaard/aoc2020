use std::collections::HashSet;

pub fn get_input() -> String {
    include_str!("./input.txt").to_owned()
}

pub struct Program {
    pc: i32,
    acc: i32,
    instructions: Vec<String>,
    seen_pcs: HashSet<i32>,
}

impl Program {
    pub fn init(instructions: &[String]) -> Program {
        Program {
            pc: 0,
            acc: 0,
            instructions: instructions.to_vec(),
            seen_pcs: HashSet::with_capacity(instructions.len()),
        }
    }

    pub fn exec(self: &mut Program) {
        if self.pc < 0 || (self.pc as usize) > self.instructions.len() - 1 {
            unreachable!("program counter out of bounds");
        }
        let inst = &self.instructions[self.pc as usize];
        let parts = inst.split(' ').collect::<Vec<_>>();
        match parts[0] {
            "acc" => self.acc += parts[1].parse::<i32>().unwrap(),
            "jmp" => self.pc += parts[1].parse::<i32>().unwrap() - 1,
            "nop" => {}
            _ => unreachable!("Weird operation: {}", parts[0]),
        }
        self.seen_pcs.insert(self.pc);
        self.pc += 1;
    }

    pub fn exec_until_same_instruction_hit_twice(self: &mut Program) {
        while !self.seen_pcs.contains(&self.pc) {
            self.exec()
        }
    }
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|e| e.to_owned()).collect()
}

pub fn part1(input: &str) -> String {
    let mut program = Program::init(&parse_input(input));
    program.exec_until_same_instruction_hit_twice();
    program.acc.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT_INPUT: &str = include_str!("./test_input.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEXT_INPUT), "5");
    }

    #[test]
    fn part1_result() {
        assert_eq!(part1(&get_input()), "1810");
    }
}
