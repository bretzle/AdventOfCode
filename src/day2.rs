//! Part 1: 538
//! Part 2: 489

pub struct Entry {
    range: (usize, usize),
    character: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();

            match parts.as_slice() {
                [range, character, password] => {
                    let bounds: Vec<_> = range.split('-').collect();
                    let min: usize = bounds[0].parse().unwrap();
                    let max: usize = bounds[1].parse().unwrap();

                    let c = character.chars().next().unwrap();

                    let password = password.to_string();

                    Entry {
                        range: (min, max),
                        character: c,
                        password,
                    }
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Entry]) -> u32 {
    let mut valid = 0;

    for entry in input {
        let count = entry.password.matches(entry.character).count();

        if count >= entry.range.0 && count <= entry.range.1 {
            valid += 1;
        }
    }

    valid
}

#[aoc(day2, part2)]
pub fn part2(input: &[Entry]) -> u32 {
    let mut valid = 0;

    for entry in input {
        let first = entry.password.chars().nth(entry.range.0 - 1) == Some(entry.character);
        let second = entry.password.chars().nth(entry.range.1 - 1) == Some(entry.character);
        if first ^ second {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
		1-3 a: abcde
		1-3 b: cdefg
		2-9 c: ccccccccc
	"};

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 1);
    }
}
