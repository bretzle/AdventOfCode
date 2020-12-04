//! Part 1: 250
//! Part 2: 1592662500

fn main() {
    let parsed_data = include_str!("../input.txt");

    println!("Part 1: {}", part1(&parsed_data, (3, 1)));
    println!("Part 2: {}", part2(&parsed_data));
}

fn part1(entries: &str, (x, y): (usize, usize)) -> usize {
    entries
        .lines()
        .step_by(y)
        .enumerate()
        .filter_map(|(idx, row)| match row.chars().cycle().nth(idx * x) {
            Some('.') => None,
            s => s,
        })
        .count()
}

fn part2(entries: &str) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes.iter().map(|slope| part1(entries, *slope)).product()
}
