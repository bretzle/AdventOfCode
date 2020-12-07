//! Part 1: 370
//! Part 2: 29547

use regex::Regex;
use std::collections::HashMap;

pub type Data = HashMap<String, Vec<(usize, String)>>;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Data {
    let regex = Regex::new(r"(\d) (\w+ \w+)").expect("Invalid regex");

    input
        .lines()
        .map(|line| {
            let vec: Vec<_> = regex
                .captures_iter(line)
                .map(|capture| {
                    let num: usize = capture[1].parse().unwrap();
                    let color = capture[2].to_string();

                    (num, color)
                })
                .collect();

            let holder: String = {
                let mut iter = line.split(' ').take(2);
                format!("{} {}", iter.next().unwrap(), iter.next().unwrap())
            };

            (holder, vec)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &Data) -> usize {
    fn contains_gold(key: &String, data: &Data) -> bool {
        if key == "shiny gold" {
            true
        } else {
            match data.get(key) {
                Some(vec) => vec.iter().any(|(_, key)| contains_gold(key, data)),
                None => false,
            }
        }
    }

    input.keys().filter(|&k| contains_gold(k, input)).count() - 1
}

#[aoc(day7, part2)]
pub fn part2(input: &Data) -> usize {
    fn total_bags(map: &Data, bag: &str) -> usize {
        map[bag]
            .iter()
            .map(|(c, b)| c * total_bags(map, b))
            .sum::<usize>()
            + 1
    }

    total_bags(input, "shiny gold") - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
		light red bags contain 1 bright white bag, 2 muted yellow bags.
		dark orange bags contain 3 bright white bags, 4 muted yellow bags.
		bright white bags contain 1 shiny gold bag.
		muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
		shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
		dark olive bags contain 3 faded blue bags, 4 dotted black bags.
		vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
		faded blue bags contain no other bags.
		dotted black bags contain no other bags.
	"};
    const SAMPLE2: &str = indoc::indoc! {"
		shiny gold bags contain 2 dark red bags.
		dark red bags contain 2 dark orange bags.
		dark orange bags contain 2 dark yellow bags.
		dark yellow bags contain 2 dark green bags.
		dark green bags contain 2 dark blue bags.
		dark blue bags contain 2 dark violet bags.
		dark violet bags contain no other bags.
	"};

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 4);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 32);
        assert_eq!(part2(&generator(SAMPLE2)), 126);
    }
}
