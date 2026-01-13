//! Works for user input only, not for example.
//! Works by having identified that whenever the area of the region is
//! less than the total area of the presents then the presents will fit -
//! the regions are either maginally smaller or about 40% greater than the
//! corresponding total area of presents. There is no in between.

use std::fs;

const DAY: &str = "12";

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
type Regions = Vec<(usize, Vec<usize>)>;
type Parsed = (Vec<usize>, Regions);

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let input = input_source.input();
    let sections: Vec<&str> = input.split("\n\n").collect();
    let (presents, regions_) = sections.split_at(sections.len() - 1);
    let sizes: Vec<usize> = presents.iter().map(|present| present.as_bytes().into_iter().map(|b| usize::from(*b==b'#')).sum::<usize>()).collect();
    let lines = regions_[0].lines();
    let regions: Vec<(usize, Vec<usize>)> = lines.map(|line| {
        let (lhs, rhs) = line.split_once(":").unwrap();
        let area = lhs.split("x").map(|v| v.parse::<usize>().unwrap()).product();
        let nums: Vec<usize> = rhs.split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect();
        (area, nums)
    }).collect();
    (sizes, regions)
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
}

fn part1((sizes, regions): &Parsed) -> String {
    regions.iter().map(|(area, nums)| {
        let req_area: usize = nums.iter().zip(sizes).map(|(n, s)| n*s).sum::<usize>();
        usize::from(area > &req_area)
    }).sum::<usize>().to_string()
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
        // let parsed = parse(Input::Test);
        // assert_eq!("2", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

}

