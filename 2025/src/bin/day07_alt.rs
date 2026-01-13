// Very heavily insprired by:
//     https://github.com/DuroCodes/advent-of-code/blob/main/2025/src/days/day07

use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use std::fs;

const DAY: &str = "07";

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
        fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    }
}

type Grid = Vec<Vec<char>>;
type Parsed = (Grid, usize);

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let input = input_source.input();
    let s = input.find("S").unwrap();
    let grid: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    // dbg!(&grid);
    (grid, s)
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

fn part1(parsed_input: &Parsed) -> String {
    let (grid, s) = parsed_input;
    (0..grid.len() - 1)
        .fold((HashSet::from([*s]), 0), |(yons, count), row| {
            let num_splits = yons
                .iter()
                .filter(|&&yon| grid[row + 1][yon] == '^')
                .count();
            let next_yons = yons
                .into_iter()
                .flat_map(|yon| {
                    if grid[row + 1][yon] == '^' {
                        vec![yon - 1, yon + 1]
                    } else {
                        vec![yon]
                    }
                })
                .collect();
            (next_yons, num_splits + count)
        })
        .1
        .to_string()
}

fn part2(parsed_input: &Parsed) -> String {
    let (grid, s) = parsed_input;
    (0..grid.len() - 1)
        .fold(HashMap::from([(*s, 1)]), |yons, row| {
            yons
                .into_iter()
                .flat_map(|(col, count)| {
                    if grid[row + 1][col] == '^' {
                        vec![(col - 1, count), (col + 1, count)]
                    } else {
                        vec![(col, count)]
                    }
                })
                .into_grouping_map()
                .sum()
        })
        .values()
        .sum::<u64>()
        .to_string()
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
        assert_eq!("21", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

    #[test]
    fn two() {
        let parsed = parse(Input::Test);
        assert_eq!("40", part2(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(2), part2(&parsed));
    }
}
