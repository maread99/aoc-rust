use std::fs;

const DAY: &str = "02";

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
    let mut total: u64 = 0;
    for rng in input.split(',') {
        let limits: Vec<&str> = rng.split('-').collect();
        let s = limits[0].parse::<u64>().unwrap();
        let e = limits[1].parse::<u64>().unwrap();
        for v in s..=e {
            let v_ = v.to_string();
            if v_.len() % 2 == 1 {
                continue;
            }
            let (l, r) = v_.split_at(v_.len() / 2);
            if l == r {
                total += v
            }
        }
    }
    total.to_string()
}

fn part2(input: &String) -> String {
    let mut total: u64 = 0;
    for rng in input.split(',') {
        let limits: Vec<&str> = rng.split('-').collect();
        let s = limits[0].parse::<u64>().unwrap();
        let e = limits[1].parse::<u64>().unwrap();
        for v in s..=e {
            let v_ = v.to_string();
            let ln = v_.len();
            for i in 1..=(ln / 2) {
                if ln % i == 0 {
                    let pat = &v_[..i];
                    let leftovers = v_.trim_end_matches(pat);
                    if leftovers.is_empty() {
                        total += v;
                        break;
                    }
                }
            }
        }
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
        assert_eq!("1227775554", part1(&input));
        
        let input: String = Input::User.input();
        assert_eq!(get_answer(1), part1(&input));
    }

    #[test]
    fn two() {
        let input: String = Input::Test.input();
        assert_eq!("4174379265", part2(&input));
        let input: String = Input::User.input();
        assert_eq!(get_answer(2), part2(&input));
    }
}
