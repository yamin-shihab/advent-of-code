fn main() {
    let input = include_str!("../../inputs/day04.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (winning, have) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let have = have.split_ascii_whitespace().collect::<Vec<&str>>();
            let score = winning
                .split_ascii_whitespace()
                .filter(|i| have.contains(i))
                .count();
            if score == 0 {
                0
            } else {
                u32::pow(2, score as u32 - 1)
            }
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let scores = input
        .lines()
        .map(|line| {
            let (winning, have) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            let have = have.split_ascii_whitespace().collect::<Vec<&str>>();
            winning
                .split_ascii_whitespace()
                .filter(|i| have.contains(i))
                .count() as u32
        })
        .collect::<Vec<u32>>();
    let mut visited = 0;
    let mut queue = (0..scores.len()).collect::<Vec<usize>>();
    while let Some(index) = queue.pop() {
        visited += 1;
        for win in 0..scores[index] {
            queue.push(index + win as usize + 1);
        }
    }
    visited
}
