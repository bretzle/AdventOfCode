//! Part 1: 1489
//! Part 2: 1539

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Opcode> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            match parts.next().unwrap() {
                "acc" => Opcode::ACC(parts.next().unwrap().parse().unwrap()),
                "jmp" => Opcode::JMP(parts.next().unwrap().parse().unwrap()),
                "nop" => Opcode::NOP(parts.next().unwrap().parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn execute_program(program: &[Opcode]) -> Result<isize, isize> {
    let mut accumulator = 0;
    let mut idx = 0;
    let mut visited = HashSet::new();

    loop {
        let instruction = match program.get(idx as usize) {
            Some(&instruction) => instruction,
            None => return Ok(accumulator),
        };

        match instruction {
            Opcode::ACC(val) => {
                accumulator += val;
                idx += 1
            }
            Opcode::JMP(val) => idx += val,
            Opcode::NOP(_) => idx += 1,
        }

        if !visited.insert(idx) {
            return Err(accumulator);
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(data: &Vec<Opcode>) -> isize {
    match execute_program(&data) {
        Ok(val) => val,
        Err(val) => val,
    }
}

#[aoc(day8, part2)]
pub fn part2(data: &Vec<Opcode>) -> isize {
    data.iter()
        .enumerate()
        .rev()
        .filter_map(|(idx, &instruction)| match instruction {
            Opcode::ACC(_) => None,
            _ => Some(idx),
        })
        .map(|idx| {
            let mut program = data.clone();
            let replacement = match data[idx] {
                Opcode::JMP(value) => Opcode::NOP(value),
                Opcode::NOP(value) => Opcode::JMP(value),
                _ => unreachable!(),
            };
            program[idx] = replacement;
            execute_program(&program)
        })
        .find(|v| v.is_ok())
        .map(|v| v.ok())
        .flatten()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
		nop +0
		acc +1
		jmp +4
		acc +3
		jmp -3
		acc -99
		acc +1
		jmp -4
		acc +6
	"};

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 5);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 8);
    }
}
