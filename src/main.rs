#![feature(start)]

use std::u128;
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

struct Grid<'a> {
    table: &'a [u8],
    width: i32,
    height: i32,
}

impl<'a> Grid<'a> {
    pub fn new(table: &'a [u8]) -> Self {
        let (mut width, mut height) = (0, 0);
        for i in 0..table.len() {
            if table[i] == '\n' as u8 {
                width = if width == 0 { (i + 1) as i32 } else { width };
                height += 1;
            }
        }
        Grid{ table: table, width: width, height: height - 1 }
    }
    pub fn at(&self, row: i32, col: i32) -> u8 {
        let pos = self.pos(row, col);
        if pos.is_none() { return '.' as u8; }
        let symbol = self.table[pos.unwrap()] as u8;
        return if symbol == '\n' as u8 { '.' as u8 } else { symbol };
    }
    pub fn pos(&self, row: i32, col: i32) -> Option<usize> {
        if row < 0 || col < 0 || row >= self.height || col >= self.width { return None::<usize>; }
        return Some((row * self.width + col) as usize);
    }
}

trait ByteExtensions {
    fn digit(&self) -> Option<i32>;
    fn dot(&self) -> bool;
}
impl ByteExtensions for u8 {
    fn digit(&self) -> Option<i32> { if '0' as u8 <= *self && *self <= '9' as u8 { Some((*self - '0' as u8) as i32) } else { None::<i32> } }
    fn dot(&self) -> bool { *self == '.' as u8 }
}
trait ByteSeqExtensions {
    fn number_span(&self, mid: usize) -> Option<i32>;
}
impl ByteSeqExtensions for &[u8] {
    fn number_span(&self, mid: usize) -> Option<i32> {
        if self[mid].digit().is_none() {
            return None::<i32>;
        }

        let mut current = mid;
        while current > 0 && self[current - 1].digit().is_some() {
            current -= 1;
        }
        let mut number = 0;
        while current < self.len() {
            match self[current].digit() {
                Some(digit) => { number = 10 * number + digit; },
                None => return Some(number),
            }
            current += 1;
        }
        return Some(number);
    }
}

fn day3_easy(input: &[u8]) {
    let grid = Grid::new(input);
    let mut sum = 0;
    for row in 0..grid.height {
        let mut col = 0;
        while col < grid.width {
            if grid.at(row, col).digit().is_none() {
                col += 1;
                continue;
            }
            let (mut number, mut is_part_number) = (0, !grid.at(row, col - 1).dot());
            while col < grid.width {
                let symbol = grid.at(row, col).digit();
                if symbol.is_none() {
                    break;
                }
                number = 10 * number + symbol.unwrap();
                for (drow, dcol) in [(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 0), (1, 1)] {
                    is_part_number |= !grid.at(row + drow, col + dcol).dot();
                }
                col += 1;
            }
            is_part_number = is_part_number || !grid.at(row, col).dot();
            if !is_part_number {
                continue;
            }
            sum += number;
        }
    }
    eprintln!("day3: {sum}");
}

struct Gear {
    drivers_count: usize,
    drivers: [i32; 2],
}

impl Gear {
    pub fn add_driver(&mut self, driver: Option<i32>) {
        if self.drivers_count >= 2 || driver.is_none() {
            return;
        }
        self.drivers[self.drivers_count] = driver.unwrap();
        self.drivers_count += 1;
    }
    pub fn power(&self) -> Option<i32> {
        if self.drivers_count == 2 { Some(self.drivers[0] * self.drivers[1]) }
        else { None::<i32> }
    }
}

fn day3_hard(input: &[u8]) {
    let grid = Grid::new(input);
    let mut sum = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.at(row, col) != '*' as u8 {
                continue;
            }
            let mut gear = Gear{drivers_count: 0, drivers: [0; 2]};
            if let Some(pos) = grid.pos(row, col - 1) {
                gear.add_driver(input.number_span(pos));
            }
            if let Some(pos) = grid.pos(row, col + 1) {
                gear.add_driver(input.number_span(pos));
            }
            for drow in [-1, 1] {
                if let Some(pos) = grid.pos(row + drow, col) {
                    let driver = input.number_span(pos);
                    gear.add_driver(driver);
                    if driver.is_some() {
                        continue;
                    }
                }
                if let Some(pos) = grid.pos(row + drow, col - 1) {
                    gear.add_driver(input.number_span(pos));
                }
                if let Some(pos) = grid.pos(row + drow, col + 1) {
                    gear.add_driver(input.number_span(pos));
                }
            }
            sum += gear.power().unwrap_or(0);
        }
    }
    eprintln!("day3: {sum}");
}

fn day4(input: &str) {
    let mut sum = 0;
    let mut scratchcards = 0;
    let mut copies_count = [1 as u128; 32];
    for (i, line) in input.lines().enumerate() {
        let (_, cards) = line.split_once(':').unwrap();
        let (winning, hand) = cards.split_once('|').unwrap();
        let mut winning_mask = 0 as u128;
        for winning_card in winning.split(' ') {
            let winning_card = winning_card.trim();
            if winning_card.len() == 0 {
                continue;
            }
            let number = winning_card.trim().parse::<u8>().unwrap();
            winning_mask |= (1 as u128) << number;
        }
        let mut wins_count = 0;
        for hand_card in hand.split(' ') {
            let hand_card = hand_card.trim();
            if hand_card.len() == 0 {
                continue;
            }
            let number = hand_card.parse::<u8>().unwrap();
            if winning_mask & ((1 as u128) << number) > 0 {
                wins_count += 1;
            }
        }
        scratchcards += copies_count[i % copies_count.len()];
        for next in 1..wins_count+1 {
            copies_count[(i + next) % copies_count.len()] += copies_count[i % copies_count.len()];
        }
        copies_count[i % copies_count.len()] = 1;

        sum += if wins_count > 0 { 1 << (wins_count - 1) } else { 0 };
    }
    eprintln!("day4: {sum}");
    eprintln!("day4: {scratchcards}");
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    day1(include_str!("inputs/input01.txt").trim(), DAY1_EASY_PATTERN);
    day1(include_str!("inputs/input01.txt").trim(), DAY1_HARD_PATTERN); 
    day2(include_str!("inputs/input02.txt").trim(), Bag{ red: 12, green: 13, blue: 14 });
    day3_easy(include_bytes!("inputs/input03.txt"));
    day3_hard(include_bytes!("inputs/input03.txt"));
    day4(include_str!("inputs/input04.txt").trim());
    0
}
