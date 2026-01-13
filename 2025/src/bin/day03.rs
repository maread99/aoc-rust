use std::fs;

const DAY: &str = "03";

#[derive(PartialEq)]
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

fn main() {
    let input: String = Input::User.input();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn part1(input: &String) -> String {
    let mut total = 0_u32;
    let lines = input.lines();
    for line in lines {
        let max1 = line[..line.len() - 1].chars().max().unwrap();
        let idx = line.find(max1).unwrap();
        let max2 = line[idx + 1..].chars().max().unwrap();
        let mut vs = String::from(max1);
        vs.push(max2);
        total += vs.parse::<u32>().unwrap();
    }
    total.to_string()
}

fn part2(input: &String) -> String {
    let mut total = 0_u64;
    let lines = input.lines();
    for line in lines {
        let mut val = line[line.len() - 12..].to_string();
        for c in line[..line.len() - 12].chars().rev() {
            let mut vals: Vec<u64> = Vec::new();
            vals.push(val.parse::<u64>().unwrap());
            for i in 0..val.len() {
                let mut ns = String::from(c) + &val.clone();
                ns.remove(i + 1);
                vals.push(ns.parse::<u64>().unwrap());
            }
            val = vals.iter().max().unwrap().to_string();
        }
        total += val.parse::<u64>().unwrap();
    }
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
        let input: String = Input::Test.input();
        assert_eq!("357", part1(&input));

        let input: String = Input::User.input();
        assert_eq!(get_answer(1), part1(&input));
    }

    #[test]
    fn two() {
        let input: String = Input::Test.input();
        assert_eq!("3121910778619", part2(&input));

        let input: String = Input::User.input();
        assert_eq!(get_answer(2), part2(&input));
    }
}
