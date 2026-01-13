use std::collections::{HashMap, HashSet};
use std::fs;

const DAY: &str = "09";

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

type Parsed = Vec<[u64; 2]>;

/// parse input as required.
fn parse(input_source: Input) -> Parsed {
    let input = input_source.input();
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn main() {
    let parsed = parse(Input::User);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}

fn part1(tiles: &Parsed) -> String {
    let mut max: u64 = 0;
    for (i, [x, y]) in tiles[0..tiles.len() - 1].iter().enumerate() {
        for [x2, y2] in tiles[i + 1..].iter() {
            max = max.max((x.abs_diff(*x2) + 1) * (y.abs_diff(*y2) + 1));
        }
    }
    max.to_string()
}

fn part2(tiles_: &Parsed) -> String {
    let mut tiles = tiles_.clone();
    let top_left = tiles.iter().min().unwrap();
    let nxt = tiles.iter().skip_while(|&t| t != top_left).nth(1).unwrap();
    if top_left[1] != nxt[1] {
        // if not travelling around in a clockwise direction
        tiles.reverse() // then change to clockwise
    }

    // evlauate the cells of the boundary that are immediately either side of each tile
    let mut in_boundary: HashSet<(u64, u64)> = HashSet::new();
    for ([x1, y1], [x2, y2]) in tiles.iter().zip(tiles[1..].iter().cycle()) {
        if y1 == y2 {
            in_boundary.insert((x1.min(x2) + 1, *y1));
            in_boundary.insert((x1.max(x2) - 1, *y1));
        } else {
            in_boundary.insert((*x1, y1.min(y2) + 1));
            in_boundary.insert((*x1, y1.max(y2) - 1));
        }
    }

    // create HashMaps in each direction mapping column/row to the cells in that col/row
    // that lie immediately to the exterior of the boundary.
    let mut xs: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut ys: HashMap<u64, Vec<u64>> = HashMap::new();
    for ([x1, y1], [x2, y2]) in tiles.iter().zip(tiles[1..].iter().cycle()) {
        if y1 == y2 {
            let y = if x2 > x1 { y1 - 1 } else { y1 + 1 };
            for x in *x1.min(x2)..=*x1.max(x2) {
                // can ignore intricacies of inner and outer corners by considering
                // all cells adjacent to the boundary section and then removing any
                // at the edges that are actually in the boundary.
                if in_boundary.contains(&(x, y)) {
                    continue;
                }
                xs.entry(x)
                    .and_modify(|v: &mut Vec<u64>| v.push(y))
                    .or_insert(Vec::from([y]));
            }
        } else {
            let x = if y2 > y1 { x1 + 1 } else { x1 - 1 };
            for y in *y1.min(y2)..=*y1.max(y2) {
                if in_boundary.contains(&(x, y)) {
                    continue;
                }
                ys.entry(y)
                    .and_modify(|v: &mut Vec<u64>| v.push(x))
                    .or_insert(Vec::from([x]));
            }
        }
    }

    xs.iter_mut().for_each(|(_k, v)| v.sort());
    ys.iter_mut().for_each(|(_k, v)| v.sort());

    let mut max: u64 = 0;
    for (i, [x, y]) in tiles[0..tiles.len() - 1].iter().enumerate() {
        for [x2, y2] in tiles[i + 1..].iter() {
            let area = (x.abs_diff(*x2) + 1) * (y.abs_diff(*y2) + 1);
            if area <= max {
                continue;
            }
            // if, for each edge of the rectangle, the corners do not
            // bisect the description of the exterior at the same point
            // then the edge crosses the exterior and can be rejected.
            if ys[y].binary_search(x) != ys[y].binary_search(x2) {
                continue;
            }
            if ys[y2].binary_search(x) != ys[y2].binary_search(x2) {
                continue;
            }
            if xs[x].binary_search(y) != xs[x].binary_search(y2) {
                continue;
            }
            if xs[x2].binary_search(y) != xs[x2].binary_search(y2) {
                continue;
            }
            max = area;
        }
    }
    max.to_string()
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
        assert_eq!("50", part1(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(1), part1(&parsed));
    }

    #[test]
    fn two() {
        let parsed = parse(Input::Test);
        assert_eq!("24", part2(&parsed));

        let parsed = parse(Input::User);
        assert_eq!(get_answer(2), part2(&parsed));
    }
}
