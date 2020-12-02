use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let values: Vec<i32> = {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|l| l.unwrap())
            .map(|s| s.parse().unwrap())
            .collect()
    };

    println!("Part 1: {}", part1(&values));
    println!("Part 2: {}", part2(&values));
}

fn part1(values: &Vec<i32>) -> i32 {
    for (i, x) in values.iter().enumerate() {
        for (j, y) in values.iter().enumerate() {
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

fn part2(values: &Vec<i32>) -> i32 {
    for (i, x) in values.iter().enumerate() {
        for (j, y) in values.iter().enumerate() {
            for (k, z) in values.iter().enumerate() {
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
