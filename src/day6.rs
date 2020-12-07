//! Part 1: 6551
//! Part 2: 3358

use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| c != '\n')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().filter(|&c| c != '\n').collect::<HashSet<_>>())
                .fold_first(|one, other| &one & &other)
                .unwrap()
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
		abc

		a
		b
		c

		ab
		ac

		a
		a
		a
		a

		b
	"};

    #[test]
    fn test_input_process_one() {
        assert_eq!(part1(SAMPLE), 11);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(SAMPLE), 6);
    }
}
