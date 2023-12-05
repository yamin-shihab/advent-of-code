const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let iter = input.lines().map(|line| {
        let digits = line.chars().filter_map(|r#char| r#char.to_digit(10));
        let digits = digits.collect::<Vec<u32>>();
        digits.first().unwrap() * 10 + digits.last().unwrap()
    });
    iter.sum()
}

fn part_two(input: &str) -> u32 {
    let mut input = input.to_string();
    for (digit, word) in DIGIT_WORDS.iter().enumerate() {
        input = input.replace(word, &format!("{}{}{0}", word, digit + 1));
    }
    part_one(&input)
}
