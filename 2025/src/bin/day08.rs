use std::fs;

const DAY: &str = "08";

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

    fn part1_n(&self) -> usize {
        match self {
            Input::User => 1000,
            Input::Test => 10,
        }
    }
}

type Parsed = (Vec<(i64, (usize, usize))>, Input, Vec<[i64; 3]>);

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let input = input_source.input();
    let boxes: Vec<[i64; 3]> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let mut dists = Vec::new();
    for (i, [x1, y1, z1]) in boxes.iter().enumerate() {
        for (j, [x2, y2, z2]) in (boxes[i + 1..]).iter().enumerate() {
            let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2);
            dists.push((dist, (i, j + i + 1)))
        }
    }
    dists.sort();
    (dists, input_source, boxes)
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

fn part1(parsed_input: &Parsed) -> String {
    let (dists, input_source, _) = parsed_input;
    let mut networks: Vec<Vec<usize>> = Vec::new();
    for (_, (a, b)) in dists.iter().take(input_source.part1_n()) {
        let a_in = (0..networks.len()).find(|&i| networks[i].contains(a));
        let b_in = (0..networks.len()).find(|&i| networks[i].contains(b));
        match (a_in, b_in) {
            (None, None) => networks.push(vec![*a, *b]),
            (None, Some(i)) => {
                networks[i].push(*a);
            }
            (Some(i), None) => {
                networks[i].push(*b);
            }
            (Some(i1), Some(i2)) if i1 != i2 => {
                let n = networks[i2].clone();
                networks[i1].extend(n);
                networks.remove(i2);
            }
            _ => {}
        }
    }
    networks.sort_by_key(|n| n.len());
    networks[networks.len() - 3..]
        .iter()
        .map(|n| n.len())
        .product::<usize>()
        .to_string()
}

fn part2(parsed_input: &Parsed) -> String {
    let (dists, _, boxes) = parsed_input;
    let mut networks: Vec<Vec<usize>> = Vec::new();
    let mut i = 0;
    while networks.len() != 1 || networks[0].len() != boxes.len() {
        let (_, (a, b)) = dists[i];
        let a_in = (0..networks.len()).find(|&i| networks[i].contains(&a));
        let b_in = (0..networks.len()).find(|&i| networks[i].contains(&b));
        match (a_in, b_in) {
            (None, None) => networks.push(vec![a, b]),
            (None, Some(i)) => {
                networks[i].push(a);
            }
            (Some(i), None) => {
                networks[i].push(b);
            }
            (Some(i1), Some(i2)) if i1 != i2 => {
                let n = networks[i2].clone();
                networks[i1].extend(n);
                networks.remove(i2);
            }
            _ => {}
        };
        i += 1;
    }
    let (_, (a, b)) = dists[i - 1];
    (boxes[a][0] * boxes[b][0]).to_string()
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
        assert_eq!("40", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

    #[test]
    fn two() {
        let parsed = parse(Input::Test);
        assert_eq!("25272", part2(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(2), part2(&parsed));
    }
}
