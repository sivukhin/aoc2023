const DAY1_EASY_PATTERN: &[&[&str]; 10] = &[
    &["0"],
    &["1"],
    &["2"],
    &["3"],
    &["4"],
    &["5"],
    &["6"],
    &["7"],
    &["8"],
    &["9"],
];
const DAY1_HARD_PATTERN: &[&[&str]; 10] = &[
    &["0"],
    &["1", "one"],
    &["2", "two"],
    &["3", "three"],
    &["4", "four"],
    &["5", "five"],
    &["6", "six"],
    &["7", "seven"],
    &["8", "eight"],
    &["9", "nine"],
];

fn day1(input: &str, patterns: &[&[&str]; 10]) {
    let mut sum = 0;
    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        let (mut first_digit, mut last_digit) = (None::<i32>, None::<i32>);
        for i in 0..line.len() {
            'digit: for digit in 0..10 {
                for pattern in patterns[digit] {
                    if line[i..].starts_with(pattern) {
                        first_digit = if first_digit.is_none() {
                            Some(digit as i32)
                        } else {
                            first_digit
                        };
                        last_digit = Some(digit as i32);
                        break 'digit;
                    }
                }
            }
        }
        let value = first_digit.unwrap() * 10 + last_digit.unwrap();
        sum += value;
    }
    println!("day1: {sum}");
}

fn main() {
    day1(include_str!("inputs/input01.txt"), DAY1_EASY_PATTERN);
    day1(include_str!("inputs/input01.txt"), DAY1_HARD_PATTERN);
}
