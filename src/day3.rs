//! Part 1: 250
//! Part 2: 1592662500

fn solve(input: &str, (x, y): (usize, usize)) -> usize {
    input
        .lines()
        .step_by(y)
        .enumerate()
        .filter_map(|(idx, row)| match row.chars().cycle().nth(idx * x) {
            Some('.') => None,
            s => s,
        })
        .count()
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, (3, 1))
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes.iter().map(|slope| solve(input, *slope)).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
		..##.......
		#...#...#..
		.#....#..#.
		..#.#...#.#
		.#...##..#.
		..#.##.....
		.#.#.#....#
		.#........#
		#.##...#...
		#...##....#
		.#..#...#.#
	"};

    #[test]
    pub fn test1() {
        assert_eq!(part1(&SAMPLE), 7);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&SAMPLE), 336);
    }
}
