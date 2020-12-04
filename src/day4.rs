//! Part 1: 254
//! Part 2: 184

use std::collections::HashMap;

pub struct Passport {
    eyr: i32,
    byr: i32,
    iyr: i32,
    hgt: (i32, String),
    ecl: String,
    hcl: String,
    pid: String,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|record| {
            record
                .split_whitespace()
                .filter_map(|field| {
                    let mut s = field.splitn(2, ':');
                    s.next().and_then(|n| s.next().map(|v| (n, v)))
                })
                .collect::<HashMap<_, _>>()
        })
        .filter_map(|map| {
            let hgt = map.get("hgt")?;
            let unit_pos = hgt
                .chars()
                .position(|c| c.is_alphabetic())
                .unwrap_or_else(|| hgt.len());
            let (val, unit) = map.get("hgt")?.split_at(unit_pos);
            let height = (val.parse().ok()?, unit.to_string());
            Some(Passport {
                eyr: map.get("eyr")?.parse().ok()?,
                byr: map.get("byr")?.parse().ok()?,
                iyr: map.get("iyr")?.parse().ok()?,
                ecl: map.get("ecl")?.to_string(),
                hgt: height,
                hcl: map.get("hcl")?.to_string(),
                pid: map.get("pid")?.to_string(),
            })
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(inputs: &[Passport]) -> usize {
    inputs.len()
}

#[aoc(day4, part2)]
pub fn part2(inputs: &[Passport]) -> usize {
    const VALID_ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    inputs
        .iter()
        .filter(|passport| {
            let mut hcl_iter = passport.hcl.chars();
            passport.byr >= 1920
                && passport.byr <= 2002
                && passport.iyr >= 2010
                && passport.iyr <= 2020
                && passport.eyr >= 2020
                && passport.eyr <= 2030
                && {
                    let hgt = (passport.hgt.0, passport.hgt.1.as_str());
                    match hgt {
                        (val, "cm") => val >= 150 && val <= 193,
                        (val, "in") => val >= 59 && val <= 76,
                        _ => false,
                    }
                }
                && hcl_iter.next() == Some('#')
                && hcl_iter.all(|c| (c.is_digit(10) || c.is_lowercase()) && c.is_digit(16))
                && VALID_ECL.contains(&passport.ecl.as_str())
                && passport.pid.len() == 9
                && passport.pid.chars().all(|c| c.is_digit(10))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
		ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
		byr:1937 iyr:2017 cid:147 hgt:183cm

		iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
		hcl:#cfa07d byr:1929

		hcl:#ae17e1 iyr:2013
		eyr:2024
		ecl:brn pid:760753108 byr:1931
		hgt:179cm

		hcl:#cfa07d eyr:2025 pid:166559648
		iyr:2011 ecl:brn hgt:59in
	"};

    #[test]
    pub fn test1() {
        assert_eq!(part1(&generator(SAMPLE)), 2);
    }

    #[test]
    pub fn test2() {
        assert_eq!(part2(&generator(SAMPLE)), 2);
    }
}
