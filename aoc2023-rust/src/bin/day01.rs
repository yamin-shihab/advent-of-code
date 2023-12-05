const WORD_DIGIT_CONVERSIONS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let digits_iter = input.lines().map(|line| {
        line.chars()
            .filter_map(|r#char| r#char.to_digit(10))
            .collect::<Vec<u32>>()
    });
    let first_last = digits_iter.map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap());
    first_last.sum::<u32>()
}

fn part_two(input: &str) -> u32 {
    let mut input = input.to_string();
    for (digit, word) in WORD_DIGIT_CONVERSIONS.iter().enumerate() {
        input = input.replace(word, &format!("{}{}{0}", word, digit + 1));
    }
    part_one(&input)
}
