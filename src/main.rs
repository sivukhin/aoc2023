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
    eprintln!("day1: {sum}");
}

struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

fn day2(input: &str, bag: Bag) {
    let mut count_valid = 0;
    let mut power_sum = 0;
    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        let game = line.strip_prefix("Game ").unwrap();
        let (game_id, rounds) = game.split_once(':').unwrap();
        let mut valid = true;
        let mut bag_prediction = Bag{ red: 0, green: 0, blue: 0 };

        for round in rounds.split(';') {
            let mut round_bag = Bag{red: 0, green: 0, blue: 0};
            for cube in round.trim().split(',') {
                let (count, color) = cube.trim().split_once(' ').unwrap();
                let field = match color.trim() {
                    "red" => &mut round_bag.red,
                    "green" => &mut round_bag.green,
                    "blue" => &mut round_bag.blue,
                    _ => panic!("unexpected color: {color}")
                };
                *field += count.parse::<i32>().unwrap();
            }
            if bag.red < round_bag.red || bag.green < round_bag.green || bag.blue < round_bag.blue {
                valid = false;
            }
            bag_prediction.red = bag_prediction.red.max(round_bag.red);
            bag_prediction.green = bag_prediction.green.max(round_bag.green);
            bag_prediction.blue = bag_prediction.blue.max(round_bag.blue);
        }
        power_sum += bag_prediction.red * bag_prediction.green * bag_prediction.blue;
        if valid {
            count_valid += game_id.parse::<i32>().unwrap();
        }
    }
    eprintln!("day2: {count_valid}");
    eprintln!("day2: {power_sum}");
}

fn main() {
    day1(include_str!("inputs/input01.txt"), DAY1_EASY_PATTERN);
    day1(include_str!("inputs/input01.txt"), DAY1_HARD_PATTERN); 
    day2(include_str!("inputs/input02.txt"), Bag{ red: 12, green: 13, blue: 14 });
}
