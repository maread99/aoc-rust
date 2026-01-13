use std::collections::{HashMap, HashSet};
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

type Coord = [usize; 2];
type Parsed = (HashSet<Coord>, Coord, usize);

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let mut input = input_source.input();
    let x_limit = input.find('\n').unwrap() - 1;
    input = input.replace("\n", "");
    let y_limit = input.len() / (x_limit + 1) - 1;
    let mut splitters: HashSet<Coord> = HashSet::new();
    for (i, c) in input.char_indices() {
        if c == '^' {
            splitters.insert([i % (x_limit + 1), i / (x_limit + 1)]);
        }
    }
    (splitters, [input.find('S').unwrap(), 0_usize], y_limit)
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

fn part1(parsed_input: &Parsed) -> String {
    let (splitters, start, y_limit) = parsed_input;

    let mut seen: HashSet<Coord> = HashSet::new();
    let mut stack = vec![*start];

    let mut total: isize = 0;
    while let Some(pos) = stack.pop() {
        let mut np = [pos[0], pos[1] + 1];
        while np[1] < *y_limit {
            if splitters.contains(&np) {
                if seen.contains(&np) {
                    break;
                }
                total += 1;
                seen.insert(np);
                stack.push([np[0] - 1, np[1] + 1]);
                stack.push([np[0] + 1, np[1] + 1]);
                break;
            }
            np[1] += 1
        }
    }
    total.to_string()
}

fn part2(parsed_input: &Parsed) -> String {
    let (splitters, start, y_limit) = parsed_input;
    let mut memo: HashMap<Coord, u128> = HashMap::new();

    fn count_paths(
        pos: Coord,
        memo: &mut HashMap<Coord, u128>,
        splitters: &HashSet<Coord>,
        y_limit: usize,
    ) -> u128 {
        let inner = |pos: Coord, memo: &mut HashMap<Coord, u128>| -> u128 {
            let [x, y] = pos;
            let mut np = [x, y + 1];
            while np[1] < y_limit {
                if splitters.contains(&np) {
                    return {
                        count_paths([np[0] - 1, np[1] + 1], memo, splitters, y_limit)
                            + count_paths([np[0] + 1, np[1] + 1], memo, splitters, y_limit)
                    };
                }
                np[1] += 1
            }
            1
        };

        if let Some(v) = memo.get(&pos) {
            *v
        } else {
            let v = inner(pos, memo);
            memo.insert(pos, v);
            v
        }
    }
    let res = count_paths(*start, &mut memo, splitters, *y_limit);
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
