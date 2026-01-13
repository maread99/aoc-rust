use std::fs;

const DAY: &str = "01";

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

fn part1(input: &String) -> i16 {
    let lines = input.lines();
    let mut total = 0;
    let mut pos = 50;

    for line in lines {
        let (l, r) = line.split_at(1);
        let n = r.parse::<i16>().unwrap();

        if l == "L" {
            pos -= n;
            if pos < 0 {
                pos = 100 + (pos % 100); // could have used  pos.rem_euclid(100)
            }
            if pos == 100 {
                pos = 0
            }
        } else {
            pos += n;
            if pos > 99 {
                pos = pos % 100;
            }
        }
        if pos == 0 {
            total += 1;
        }
    }
    total
}

fn part2(input: &String) -> i16 {
    let lines = input.lines();
    let mut total = 0;
    let mut pos = 50;

    for line in lines {
        let (l, r) = line.split_at(1);
        let n: i16 = r.parse().unwrap();

        if l == "L" {
            let was0: bool = pos == 0;
            pos -= n;
            if pos <= 0 {
                total += (if was0 { 0 } else { 1 }) - (pos / 100);
                pos = 100 + (pos % 100);
            }
            if pos == 100 {
                pos = 0
            }
        } else {
            pos += n;
            if pos > 99 {
                total += pos / 100;
                pos %= 100;
            }
        }
        assert!(0 <= pos && pos < 100);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input: String = Input::Test.input();
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn part_two() {
        let input: String = Input::Test.input();

        assert_eq!(6, part2(&input));
    }
}
