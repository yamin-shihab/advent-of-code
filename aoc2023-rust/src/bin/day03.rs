use std::collections::hash_map::HashMap;

fn main() {
    let input = include_str!("../../inputs/day03.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|line| {
            let mut line = line.chars().collect::<Vec<char>>();
            line.push('.');
            line
        })
        .collect::<Vec<Vec<char>>>();
    let mut numbers = HashMap::new();
    for y in 0..lines.len() {
        let mut current = None;
        let mut positions = Vec::new();
        for x in 0..lines[y].len() {
            let c = lines[y][x];
            if let Some(d) = c.to_digit(10) {
                if let Some(n) = current {
                    positions.push(x);
                    current = Some(n * 10 + d);
                } else {
                    positions.push(x);
                    current = Some(d);
                }
            } else {
                if let Some(n) = current {
                    for position in positions.clone() {
                        numbers.insert((position, y), n);
                    }
                }
                positions.clear();
                current = None;
            }
        }
    }
    let mut parts = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let c = lines[y][x];
            if !c.is_ascii_digit() && c != '.' {
                let mut adjacent = vec![
                    numbers.get(&(x + 1, y + 1)),
                    numbers.get(&(x + 1, y - 1)),
                    numbers.get(&(x + 1, y)),
                    numbers.get(&(x - 1, y + 1)),
                    numbers.get(&(x - 1, y - 1)),
                    numbers.get(&(x - 1, y)),
                    numbers.get(&(x, y + 1)),
                    numbers.get(&(x, y - 1)),
                ]
                .into_iter()
                .filter_map(|number| number)
                .collect::<Vec<&u32>>();
                adjacent.sort();
                adjacent.dedup();
                println!("{:?}", adjacent);
                for number in adjacent {
                    parts.push(*number);
                }
            }
        }
    }
    parts.into_iter().sum()
}

fn part_two(input: &str) -> u32 {
    let lines = input
        .lines()
        .map(|line| {
            let mut line = line.chars().collect::<Vec<char>>();
            line.push('.');
            line
        })
        .collect::<Vec<Vec<char>>>();
    let mut numbers = HashMap::new();
    for y in 0..lines.len() {
        let mut current = None;
        let mut positions = Vec::new();
        for x in 0..lines[y].len() {
            let c = lines[y][x];
            if let Some(d) = c.to_digit(10) {
                if let Some(n) = current {
                    positions.push(x);
                    current = Some(n * 10 + d);
                } else {
                    positions.push(x);
                    current = Some(d);
                }
            } else {
                if let Some(n) = current {
                    for position in positions.clone() {
                        numbers.insert((position, y), n);
                    }
                }
                positions.clear();
                current = None;
            }
        }
    }
    let mut parts = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let c = lines[y][x];
            if c == '*' {
                let mut adjacent = vec![
                    numbers.get(&(x + 1, y + 1)),
                    numbers.get(&(x + 1, y - 1)),
                    numbers.get(&(x + 1, y)),
                    numbers.get(&(x - 1, y + 1)),
                    numbers.get(&(x - 1, y - 1)),
                    numbers.get(&(x - 1, y)),
                    numbers.get(&(x, y + 1)),
                    numbers.get(&(x, y - 1)),
                ]
                .into_iter()
                .filter_map(|number| number)
                .collect::<Vec<&u32>>();
                adjacent.sort();
                adjacent.dedup();
                if adjacent.len() == 2 {
                    parts.push(adjacent.into_iter().product::<u32>());
                }
            }
        }
    }
    parts.into_iter().sum()
}
