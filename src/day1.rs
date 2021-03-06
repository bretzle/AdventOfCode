//! Part 1: 793524
//! Part 2: 61515678

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    for (i, x) in input.iter().enumerate() {
        for (j, y) in input.iter().enumerate() {
            if i == j {
                continue;
            }
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    panic!()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    for (i, x) in input.iter().enumerate() {
        for (j, y) in input.iter().enumerate() {
            for (k, z) in input.iter().enumerate() {
                if i == j && j == k {
                    continue;
                }
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
		1721
		979
		366
		299
		675
		1456
	"};

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 514579);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 241861950);
    }
}
