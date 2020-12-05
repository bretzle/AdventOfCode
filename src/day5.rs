//! Part 1: 919
//! Part 2: 642

fn find_seat(input: &str) -> (usize, usize) {
    let row_str = &input[0..7];
	let col_str = &input[7..];

    let mut row = 0..127;
    let mut col = 0..7;

    row_str.chars().for_each(|c| {
        let m = row.start + (row.end - row.start) / 2;
        match c {
            'F' => row.end = m - 1,
            'B' => row.start = m + 1,
            _ => unreachable!(),
        }
    });

    col_str.chars().for_each(|c| {
        let m = col.start + (col.end - col.start) / 2;
        match c {
            'L' => col.end = m - 1,
            'R' => col.start = m + 1,
            _ => unreachable!(),
        }
    });

    (row.start, col.start)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|line| find_seat(line))
        .map(|(r, c)| r * 8 + c)
        .max()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> Option<usize> {
    const EMPTY_SEAT: char = '.';
    const FULL_SEAT: char = 'X';

    let mut storage = vec![vec![EMPTY_SEAT; 8]; 115];

    input
        .lines()
        .map(|line| find_seat(line))
        .for_each(|(r, c)| storage[r][c] = FULL_SEAT);

    for (idx, row) in storage.iter().enumerate() {
        if row.iter().all(|&c| c == EMPTY_SEAT) {
            continue;
        }

        let seat = row.iter().position(|&c| c == EMPTY_SEAT);

        if seat.is_some() {
            return Some(idx * 8 + seat.unwrap());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_seat_finder() {
        assert_eq!(find_seat("BFFFBBFRRR"), (70, 7));
        assert_eq!(find_seat("FFFBBBFRRR"), (14, 7));
        assert_eq!(find_seat("BBFFBBFRLL"), (102, 4));
    }
}
