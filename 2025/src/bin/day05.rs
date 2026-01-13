use std::fs;

const DAY: &str = "05";

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

type Parsed = (Vec<[u64; 2]>, Vec<u64>);

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let input = input_source.input();
    let ranges_ids = input.split_once("\n\n").unwrap();
    let ranges: Vec<[u64; 2]> = ranges_ids
        .0
        .lines()
        .map(|line| {
            let (s, e) = line.split_once("-") .unwrap();
            [s.parse().unwrap(), e.parse().unwrap()]
        })
        .collect();
    let ids: Vec<u64> = ranges_ids
        .1
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (ranges, ids)
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

fn part1(parsed_input: &Parsed) -> String {
    let (ranges, ids) = parsed_input;
    let mut count = 0;
    for id in ids {
        for rng in ranges {
            if (rng[0]..=rng[1]).contains(id) {
                count += 1;
                break;
            }
        }
    }
    count.to_string()
}

fn part2(parsed_input: &Parsed) -> String {
    let (ranges_, _) = parsed_input;
    let mut ranges = ranges_.clone();
    ranges.sort();
    let mut ans: u64 = 0;
    let mut current = &ranges[0];
    for rng in &ranges[1..] {
        if current[1] < rng[0] {
            // no overlap, so add the range
            ans += current[1] - current[0] + 1;
        } else if current[0] < rng[0] && current[1] > rng[1] {
            continue; // current fully overlaps next range, so ignore the range 
        } else {
            ans += rng[0] - current[0]; // parital overlap, add only non-overlapping part 
        }
        current = rng;
    }
    ans += current[1] - current[0] + 1;
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
        assert_eq!("3", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

    #[test]
    fn two() {
        let parsed = parse(Input::Test);
        assert_eq!("14", part2(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(2), part2(&parsed));
    }
}
