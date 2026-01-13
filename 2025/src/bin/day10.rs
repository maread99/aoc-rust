//! part 2 based on this gem: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
//! Executes in a shade over 2 seconds.

use std::collections::HashMap;
use std::fs;

const DAY: &str = "10";

enum Input {
    Test,
    User,
}

impl Input {
    fn input(&self) -> String {
        let dir = match self {
            Input::User => "input",
            Input::Test => "examples",
        };
        let path = format!("{dir}/day{DAY}.txt");
        fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    }
}

#[derive(Debug)]
struct Data {
    lights: i16,
    buttons: Vec<i16>,
    counters: Vec<i16>,
}

type Parsed = Vec<Data>;

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let input = input_source.input();
    input
        .lines()
        .map(move |line| {
            let (lights_, rhs) = line[1..].split_once(']').unwrap();
            let lights = lights_
                .as_bytes()
                .iter()
                .fold(0, |acc, b| acc << 1 | i16::from(*b == b'#')); // lights as bitmasks
            let (buttons_, counters_) = rhs[..rhs.len() - 1].split_once('{').unwrap();
            let buttons: Vec<i16> = buttons_
                .split_whitespace()
                .map(|bs| {
                    let width = lights_.len() as u32;
                    bs[1..bs.len() - 1]
                        .split(',')
                        .map(|v| 2_i16.pow(width - 1 - v.parse::<u32>().unwrap()))
                        .sum()
                })
                .collect(); // buttons as bitmasks
            let counters: Vec<i16> = counters_
                .split(',')
                .map(|v| v.parse::<i16>().unwrap())
                .collect();
            Data {
                lights,
                buttons,
                counters,
            }
        })
        .collect()
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

/// Get all combinations of integers in a vector.
/// (used to get all combinations of buttons).
fn combinations(v: &Vec<i16>) -> Vec<Vec<i16>> {
    fn add_nxt(
        v: &Vec<i16>,
        frm: usize,
        cur: &mut Vec<i16>,
        combos: &mut Vec<Vec<i16>>,
    ) {
        for i in (frm..v.len()) {
            cur.push(v[i]);
            combos.push(cur.clone());
            add_nxt(v, i + 1, cur, combos);
            cur.pop();
        }
    }
    let mut cur: Vec<i16> = Vec::new();
    let mut combos: Vec<Vec<i16>> = Vec::new();
    // include option to not press a button
    combos.insert(0, vec![0]);
    add_nxt(v, 0, &mut cur, &mut combos);
    combos
}

/// Realising that it makes no sense to press the same button more than once,
/// the answer has to come from pressing between one and all buttons once each.
fn part1(data: &Parsed) -> String {
    let mut tot = 0;
    for d in data {
        let combos: Vec<Vec<i16>> = combinations(&d.buttons);
        let mut min = usize::MAX;
        for combo in combos {
            if combo.len() > min {
                continue;
            };
            let res = combo.iter().fold(d.lights, |acc, v| acc ^ v);
            if res == 0 {
                min = min.min(combo.len())
            }
        }
        tot += min;
    }
    tot.to_string()
}

fn part2(data: &Parsed) -> String {
    fn get_fewest_presses(
        counters: Vec<i16>,
        all_combos: &Vec<Vec<i16>>,
        memo: &mut HashMap<Vec<i16>, i64>,
    ) -> i64 {
        fn inner(
            counters: &[i16],
            all_combos: &Vec<Vec<i16>>,
            memo: &mut HashMap<Vec<i16>, i64>,
        ) -> i64 {
            if counters.iter().any(|c| *c < 0) {
                return 1_000_000;
            }
            if counters.iter().sum::<i16>() == 0 {
                return 0;
            };
            // bitmask as part1 'light pattern' that would reduce all counters to even
            // values, for example if counters 3547 then bitmask would be 0b1101 (as i16),
            // i.e. need to toggle the lights at indexes 0, 1 and 3 in order to reduce
            // counters to values that are all even.
            let bitmask = counters
                .iter()
                .fold(0, |acc, c| (acc << 1) | i16::from(c % 2 != 0));
            all_combos
                .iter()
                .filter_map(|combo| {
                    if combo.iter().fold(bitmask, |acc, v| acc ^ v) != 0 {
                        return None;
                    }
                    let mut new_counters: Vec<i16> = counters.to_vec();
                    if **combo != vec![0] {
                        combo.iter().for_each(|button| {
                            let width = counters.len();
                            (0..width).rev().for_each(|i| {
                                new_counters[width - i - 1] -= (button >> i) & 1;
                            })
                        })
                    };
                    new_counters = new_counters.iter().map(|c| c / 2).collect();
                    let a = if **combo == vec![0] {
                        0 // No button pressed, rather all counters were already even
                    } else {
                        combo.len() as i64
                    };
                    let b = get_fewest_presses(new_counters, all_combos, memo) * 2;
                    Some(a + b)
                })
                .min()
                .unwrap_or(1_000_000)
        }

        if let Some(v) = memo.get(&counters) {
            *v
        } else {
            let v = inner(&counters, all_combos, memo);
            memo.insert(counters, v);
            v
        }
    }

    let mut res: i64 = 0;
    for d in data.iter() {
        let all_combos = combinations(&d.buttons);
        let mut memo: HashMap<Vec<i16>, i64> = HashMap::new();
        let v = get_fewest_presses(d.counters.clone(), &all_combos, &mut memo);
        res += v;
    }
    res.to_string()
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
        assert_eq!("7", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

    #[test]
    fn two() {
        let parsed = parse(Input::Test);
        assert_eq!("33", part2(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(2), part2(&parsed));
    }
}
