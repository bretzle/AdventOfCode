use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

fn main() {
    let lines: Vec<String> = {
        let file = File::open("aoc02/input.txt").unwrap();
        let reader = BufReader::new(file);
        reader.lines().map(|l| l.unwrap()).collect()
    };

    // let lines = vec![
    //     "1-3 a: abcde".to_string(),
    //     "1-3 b: cdefg".to_string(),
    //     "2-9 c: ccccccccc".to_string(),
    // ];

    let parsed_data = parse(&lines);

    println!("Part 1:");
    part1(&parsed_data);

    println!("\nPart 2:");
    part2(&parsed_data);
}

fn part1(entries: &Vec<Entry>) {
    let mut valid = 0;

    for entry in entries {
        let count = entry.password.matches(entry.character).count();

        if count >= entry.range.0 && count <= entry.range.1 {
            valid += 1;
        }
    }

    println!("Valid password count: {}", valid);
}

fn part2(entries: &Vec<Entry>) {
    let mut valid = 0;

    for entry in entries {
        let first = entry.password.chars().nth(entry.range.0 - 1) == Some(entry.character);
        let second = entry.password.chars().nth(entry.range.1 - 1) == Some(entry.character);
        if first ^ second {
            valid += 1;
        }
    }

    println!("Valid password count: {}", valid);
}

struct Entry<'a> {
    range: (usize, usize),
    character: char,
    password: &'a str,
}

fn parse(lines: &Vec<String>) -> Vec<Entry> {
    let mut ret = vec![];

    for line in lines {
        let parts = line.split(' ').collect::<Vec<_>>();

        match parts.as_slice() {
            [range, character, password] => {
                let bounds: Vec<_> = range.split('-').collect();
                let min = bounds[0].parse().unwrap();
                let max = bounds[1].parse().unwrap();

                let c = character.chars().next().unwrap();

                let password = password;

                ret.push(Entry {
                    range: (min, max),
                    character: c,
                    password,
                })
            }
            _ => unreachable!(),
        }
    }

    ret
}
