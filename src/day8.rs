//! Part 1: 1489
//! Part 2: 1539

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    ACC,
    JMP,
    NOP,
}

type Data = Vec<(Opcode, isize)>;

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Data {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let opcode = match parts.next().unwrap() {
                "acc" => Opcode::ACC,
                "jmp" => Opcode::JMP,
                "nop" => Opcode::NOP,
                _ => unreachable!(),
            };

            let arg = match opcode {
                Opcode::ACC | Opcode::JMP => {
                    let arg = parts.next().unwrap();
                    arg.parse().unwrap()
                }
                Opcode::NOP => 0,
            };

            (opcode, arg)
        })
        .collect()
}

fn execute_program(program: &Data) -> Result<isize, isize> {
    let mut accumulator = 0;
    let mut idx = 0;
    let mut visited = HashSet::new();

    loop {
        let instruction = match program.get(idx as usize) {
            Some(&instruction) => instruction,
            None => return Ok(accumulator),
        };

        match instruction.0 {
            Opcode::ACC => {
                accumulator += instruction.1;
                idx += 1
            }
            Opcode::JMP => idx += instruction.1,
            Opcode::NOP => idx += 1,
        }

        if !visited.insert(idx) {
            return Err(accumulator);
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(data: &Data) -> isize {
    match execute_program(&data) {
        Ok(val) => val,
        Err(val) => val,
    }
}

#[aoc(day8, part2)]
pub fn part2(data: &Data) -> isize {
    data.iter()
        .enumerate()
        .filter_map(|(idx, &instruction)| match instruction.0 {
            Opcode::ACC => None,
            _ => Some(idx),
        })
        .map(|idx| {
            let mut program = data.clone();
            let replacement = match data[idx] {
                (Opcode::JMP, value) => (Opcode::NOP, value),
                (Opcode::NOP, value) => (Opcode::JMP, value),
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
