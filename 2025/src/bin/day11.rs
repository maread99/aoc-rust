use std::collections::HashMap;
use std::fs;

const DAY: &str = "11";

enum Input {
    Test,
    Test2,
    User,
}

impl Input {
    fn input(&self) -> String {
        let dir = match self {
            Input::User => "input",
            Input::Test => "examples",
            Input::Test2 => "examples2",
        };
        let path = format!("{dir}/day{DAY}.txt");
        fs::read_to_string(path).unwrap().replace("\r\n", "\n")
    }
}

type Parsed = HashMap<String, Vec<String>>;
type Network = Parsed;

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let input = input_source.input();
    let mut network: Parsed = HashMap::new();
    input.lines().for_each(|line| {
        let (lhs, rhs_) = line.split_once(':').unwrap();
        let outs: Vec<String> =
            Vec::from_iter(rhs_.split_whitespace().map(|s| s.to_string()));
        network.insert(lhs.to_string(), outs);
    });
    network
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

fn num_paths<'a>(
    frm: &'a str,
    to: &'a str,
    network: &'a Network,
    memo: &mut HashMap<(&'a str, &'a str), isize>,
) -> isize {
    fn inner<'a>(
        frm: &'a str,
        to: &'a str,
        network: &'a Network,
        memo: &mut HashMap<(&'a str, &'a str), isize>,
    ) -> isize {
        if frm == to {
            return 1;
        };
        match network.get(frm) {
            Some(nodes) => nodes
                .iter()
                .map(|node| num_paths(node, to, network, memo))
                .sum(),
            _ => 0,
        }
    }
    match memo.get(&(frm, to)) {
        Some(res) => *res,
        _ => {
            let v = inner(frm, to, network, memo);
            memo.insert((frm, to), v);
            v
        }
    }
}

fn part1(network: &Network) -> String {
    let mut memo = HashMap::new();
    num_paths("you", "out", network, &mut memo).to_string()
}

fn part2(network: &Parsed) -> String {
    let mut memo = HashMap::new();
    (num_paths("svr", "fft", network, &mut memo)
        * num_paths("fft", "dac", network, &mut memo)
        * num_paths("dac", "out", network, &mut memo))
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
        assert_eq!("5", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

    #[test]
    fn two() {
        let parsed = parse(Input::Test2);
        assert_eq!("2", part2(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(2), part2(&parsed));
    }
}
