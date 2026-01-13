use std::fs;

const DAY: &str = "06";

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

type Parsed = (Vec<char>, Vec<String>);

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let input = input_source.input();
    let mut lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();
    let ops: Vec<char> = lines.pop().unwrap().split_ascii_whitespace().map(|s| s.chars().next().unwrap()).collect();    
    (ops, lines)
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

fn part1(parsed_input: &Parsed) -> String {
    let (ops, lines_) = parsed_input;
    let lines: Vec<Vec<u128>> = lines_.iter().map(|line| {
        line.split_whitespace().map(|v| v.parse::<u128>().unwrap()).collect()
    }).collect();
    let mut total: u128 = 0;
    for i in 0..lines[0].len() {
        let mut col_total = if ops[i] == '+' {0} else {1};
        for line in &lines {
            if ops[i] == '+' { col_total += line[i] } else {col_total *= line[i]};
        };
        total += col_total;
    }
    total.to_string()
}

fn part2(parsed_input: &Parsed) -> String {
    let (ops_, lines) = parsed_input;
    let mut ops = ops_.iter();
    let mut total: u64 = 0;
    let mut vals: Vec<u64> = Vec::new();
    let mut v = String::with_capacity(lines.len());
    for i in 0..lines[0].len() {
        v.clear();
        for line in lines {  // compile value in column
            v += &line[i..=i];
        }
        if v.trim().is_empty() {
            if *ops.next().unwrap() == '+' {
                total += vals.iter().sum::<u64>();
            } else { total += vals.iter().product::<u64>(); };
            vals = Vec::new();
            continue;
        }
        vals.push(v.trim().parse::<u64>().unwrap());
    }
    // don't forget the last one...
    if *ops.next().unwrap() == '+' {
        total += vals.iter().sum::<u64>();
    } else { total += vals.iter().product::<u64>(); };

    total.to_string()
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
        assert_eq!("4277556", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

    #[test]
    fn two() {
        let parsed = parse(Input::Test);
        assert_eq!("3263827", part2(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(2), part2(&parsed));
    }
}

