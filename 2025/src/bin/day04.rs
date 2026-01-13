use std::collections::HashSet;
use std::fs;

const DAY: &str = "04";

enum Input {
    Test,
    User,
}

impl Input {
    fn input(self) -> String {
        let dir = match self {
            Input::User => "input",
            Input::Test => "examples",
        };
        let path = format!("{dir}/day{DAY}.txt");
        fs::read_to_string(path).unwrap()
    }
}

fn parse(input_source: Input) -> Vec<Vec<char>> {
    let input = input_source.input();
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    rows
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

pub type Coord = (isize, isize);

pub const VECS8: [Coord; 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (1, 1),
    (-1, 1),
    (1, -1),
];

/// Get HashSet of cells representing a boundary around a n x n grid.
pub fn get_boundary(n: isize) -> HashSet<Coord> {
    let mut boundary = HashSet::new();
    for i in -1..=n {
        boundary.insert((-1, i));
        boundary.insert((n, i));
        boundary.insert((i, -1));
        boundary.insert((i, n));
    }
    boundary
}

/// Get HastSet containing all cells not containing a paper roll
/// including a boundary around the edge of the grid
fn get_empty(rows: &Vec<Vec<char>>) -> HashSet<Coord> {
    assert_eq!(rows.len(), rows[0].len());
    let dim = rows.len();
    let mut empty = get_boundary(dim as isize);
    for j in 0..dim {
        for i in 0..dim {
            if rows[j][i] == '.' {
                empty.insert((j as isize, i as isize));
            }
        }
    }
    empty
}

/// Get HastSet containing all cells with paper rolls that can be removed
fn get_can_remove(empty: &HashSet<Coord>, n: isize) -> HashSet<Coord> {
    let mut can_remove = HashSet::new();
    for j in 0..n {
        for i in 0..n {
            if empty.contains(&(j, i)) {
                continue;
            };
            let mut count = 0;
            for (dj, di) in VECS8 {
                if !empty.contains(&(j + dj, i + di)) {
                    count += 1;
                    if count > 3 {
                        break;
                    };
                };
            }
            if count < 4 {
                let _ = can_remove.insert((j, i));
            };
        }
    }
    can_remove
}

fn part1(rows: &Vec<Vec<char>>) -> String {
    let mut empty = get_empty(rows);
    let can_remove = get_can_remove(&empty, rows.len() as isize);
    can_remove.len().to_string()
}

fn part2(rows: &Vec<Vec<char>>) -> String {
    let mut empty = get_empty(rows);
    let n = rows.len() as isize;
    let mut can_remove = get_can_remove(&empty, n);
    let mut ans = can_remove.len();
    while can_remove.len() > 0 {
        empty = empty.union(&can_remove).cloned().collect();
        can_remove = get_can_remove(&empty, n);
        ans += can_remove.len();
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_answer(part: u8) -> String {
        assert!(part == 1 || part == 2);
        let path = format!("answers/day{DAY}.txt");
        let contents = fs::read_to_string(path).unwrap();
        let mut lines = contents.lines();
        if part == 2 {
            lines.next();
        };
        lines.next().unwrap().to_string()
    }

    #[test]
    fn one() {
        let parsed = parse(Input::Test);
        assert_eq!("13", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

    #[test]
    fn two() {
        let parsed = parse(Input::Test);
        assert_eq!("43", part2(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(2), part2(&parsed));
    }
}
