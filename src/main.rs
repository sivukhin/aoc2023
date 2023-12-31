#![feature(start)]

use std::collections::{BTreeSet, HashMap};
use std::i64;

use rand::{
    rngs::SmallRng,
    Rng, SeedableRng,
};

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
    eprintln!("day01: {sum}");
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
        let mut bag_prediction = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };

        for round in rounds.split(';') {
            let mut round_bag = Bag {
                red: 0,
                green: 0,
                blue: 0,
            };
            for cube in round.trim().split(',') {
                let (count, color) = cube.trim().split_once(' ').unwrap();
                let field = match color.trim() {
                    "red" => &mut round_bag.red,
                    "green" => &mut round_bag.green,
                    "blue" => &mut round_bag.blue,
                    _ => panic!("unexpected color: {color}"),
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
    eprintln!("day02: {count_valid}");
    eprintln!("day02: {power_sum}");
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}

const DIRECTIONS_4: &[Point] = &[
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];
const DIRECTIONS_6: &[Point] = &[
    Point { x: 1, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: -1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
];

impl Point {
    fn mult(&self, k: i32) -> Point {
        Point {
            x: self.x * k,
            y: self.y * k,
        }
    }
    fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn sub(&self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
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
        Grid {
            table,
            width,
            height: height - 1,
        }
    }
    pub fn at(&self, position: Point) -> u8 {
        let pos = self.pos(position);
        if pos.is_none() {
            return '.' as u8;
        }
        let symbol = self.table[pos.unwrap()] as u8;
        return if symbol == '\n' as u8 {
            '.' as u8
        } else {
            symbol
        };
    }
    pub fn pos(&self, Point { x: row, y: col }: Point) -> Option<usize> {
        if row < 0 || col < 0 || row >= self.height || col >= self.width {
            return None::<usize>;
        }
        return Some((row * self.width + col) as usize);
    }
    pub fn index(&self, index: i32) -> Option<Point> {
        if index < 0 || index >= self.table.len() as i32 {
            return None;
        }
        return Some(Point {
            x: index / self.width,
            y: index % self.width,
        });
    }
}

trait ByteExtensions {
    fn digit(&self) -> Option<i32>;
    fn dot(&self) -> bool;
}
impl ByteExtensions for u8 {
    fn digit(&self) -> Option<i32> {
        if '0' as u8 <= *self && *self <= '9' as u8 {
            Some((*self - '0' as u8) as i32)
        } else {
            None::<i32>
        }
    }
    fn dot(&self) -> bool {
        *self == '.' as u8
    }
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
                Some(digit) => {
                    number = 10 * number + digit;
                }
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
            if grid.at(Point { x: row, y: col }).digit().is_none() {
                col += 1;
                continue;
            }
            let (mut number, mut is_part_number) =
                (0, !grid.at(Point { x: row, y: col - 1 }).dot());
            while col < grid.width {
                let symbol = grid.at(Point { x: row, y: col }).digit();
                if symbol.is_none() {
                    break;
                }
                number = 10 * number + symbol.unwrap();
                let position = Point { x: row, y: col };
                for &direction in DIRECTIONS_6 {
                    is_part_number |= !grid.at(position.add(direction)).dot();
                }
                col += 1;
            }
            is_part_number = is_part_number || !grid.at(Point { x: row, y: col }).dot();
            if !is_part_number {
                continue;
            }
            sum += number;
        }
    }
    eprintln!("day03: {sum}");
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
        if self.drivers_count == 2 {
            Some(self.drivers[0] * self.drivers[1])
        } else {
            None::<i32>
        }
    }
}

fn day3_hard(input: &[u8]) {
    let grid = Grid::new(input);
    let mut sum = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if grid.at(Point { x: row, y: col }) != '*' as u8 {
                continue;
            }
            let mut gear = Gear {
                drivers_count: 0,
                drivers: [0; 2],
            };
            if let Some(pos) = grid.pos(Point { x: row, y: col - 1 }) {
                gear.add_driver(input.number_span(pos));
            }
            if let Some(pos) = grid.pos(Point { x: row, y: col + 1 }) {
                gear.add_driver(input.number_span(pos));
            }
            for drow in [-1, 1] {
                if let Some(pos) = grid.pos(Point {
                    x: row + drow,
                    y: col,
                }) {
                    let driver = input.number_span(pos);
                    gear.add_driver(driver);
                    if driver.is_some() {
                        continue;
                    }
                }
                if let Some(pos) = grid.pos(Point {
                    x: row + drow,
                    y: col - 1,
                }) {
                    gear.add_driver(input.number_span(pos));
                }
                if let Some(pos) = grid.pos(Point {
                    x: row + drow,
                    y: col + 1,
                }) {
                    gear.add_driver(input.number_span(pos));
                }
            }
            sum += gear.power().unwrap_or(0);
        }
    }
    eprintln!("day03: {sum}");
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
        assert!(wins_count < copies_count.len());
        scratchcards += copies_count[i % copies_count.len()];
        for next in 1..wins_count + 1 {
            copies_count[(i + next) % copies_count.len()] += copies_count[i % copies_count.len()];
        }
        copies_count[i % copies_count.len()] = 1;

        sum += if wins_count > 0 {
            1 << (wins_count - 1)
        } else {
            0
        };
    }
    eprintln!("day04: {sum}");
    eprintln!("day04: {scratchcards}");
}

fn split_exact<const N: usize>(input: &str, delimiter: char) -> Result<[&str; N], &str> {
    let mut elements = [""; N];
    let mut index = 0;
    for element in input.split(delimiter) {
        if index >= N {
            return Err("too many elements for split");
        }
        elements[index] = element;
        index += 1;
    }
    if index < N {
        return Err("too few elements for split");
    }
    return Ok(elements);
}

fn day5(input: &str) {
    let mut min_location = None::<i64>;
    let seeds_line = input.lines().next().unwrap();
    let (_, seeds) = seeds_line.split_once(':').unwrap();
    for seed in seeds.trim().split(' ') {
        let mut seed = seed.trim().parse::<i64>().unwrap();
        let mut mapped = false;
        for line in input.lines().skip(2) {
            if line.contains(':') {
                mapped = false;
                continue;
            }
            if line.len() == 0 {
                continue;
            }
            let [dst_range_start, src_range_start, range_len] =
                split_exact::<3>(line.trim(), ' ').unwrap();
            let (dst_range_start, src_range_start, range_len) = (
                dst_range_start.parse::<i64>().unwrap(),
                src_range_start.parse::<i64>().unwrap(),
                range_len.parse::<i64>().unwrap(),
            );
            if !mapped && src_range_start <= seed && seed < src_range_start + range_len {
                seed = dst_range_start + (seed - src_range_start);
                mapped = true;
            }
        }
        min_location = Some(min_location.unwrap_or(seed).min(seed));
    }

    let min_location = min_location.unwrap();
    eprintln!("day05: {min_location}");
}

fn count_winning_options(time: i64, distance: i64) -> i64 {
    assert!(time * time > 4 * distance);
    let (time, distance) = (time as f64, distance as f64);
    let min_duration = time / 2.0 - (time * time - 4.0 * distance).sqrt() / 2.0;
    let max_duration = time / 2.0 + (time * time - 4.0 * distance).sqrt() / 2.0;
    return (max_duration.ceil() - min_duration.floor() - 1.0) as i64;
}

fn day6(input: &str) {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_once(':').unwrap().1;
    let distances = lines.next().unwrap().split_once(':').unwrap().1;
    let mut time_iterator = times.trim().split(' ');
    let mut distance_iterator = distances.trim().split(' ');
    let mut mult = 1;
    let (mut mega_time, mut mega_distance) = (0 as i64, 0 as i64);
    loop {
        let mut time = time_iterator.next();
        while time.is_some() && time.unwrap().len() == 0 {
            time = time_iterator.next();
        }
        let mut distance = distance_iterator.next();
        while distance.is_some() && distance.unwrap().len() == 0 {
            distance = distance_iterator.next();
        }
        if time.is_none() && distance.is_none() {
            break;
        }
        for c in time.unwrap().as_bytes() {
            mega_time = mega_time * 10 + c.digit().unwrap() as i64;
        }
        for c in distance.unwrap().as_bytes() {
            mega_distance = mega_distance * 10 + c.digit().unwrap() as i64;
        }
        let time = time.unwrap().parse::<i64>().unwrap();
        let distance = distance.unwrap().parse::<i64>().unwrap();
        mult *= count_winning_options(time, distance);
    }
    eprintln!("day06: {mult}");
    let mega_options = count_winning_options(mega_time, mega_distance);
    eprintln!("day06: {mega_options}");
}

#[derive(PartialEq, PartialOrd, Debug)]
enum CombinationType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    OnePair,
    High,
}
struct Hand {
    combination: CombinationType,
    cards: [u8; 5],
}

const SIMPLE_CARDS: &[u8] = &[
    b'A', b'K', b'Q', b'J', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2',
];
const JOKER_CARDS: &[u8] = &[
    b'A', b'K', b'Q', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2', b'J',
];
trait GameRules {
    fn card_order(card: u8) -> usize;
    fn parse_hand(hand: &[u8]) -> Hand;
}

struct SimpleGame;
impl GameRules for SimpleGame {
    fn card_order(card: u8) -> usize {
        for i in 0..SIMPLE_CARDS.len() {
            if SIMPLE_CARDS[i] == card {
                return i;
            }
        }
        panic!("unexpected card");
    }
    fn parse_hand(hand: &[u8]) -> Hand {
        let mut original_cards = [0 as u8; 5];
        original_cards.copy_from_slice(hand);

        let mut sorted_cards = original_cards;
        sorted_cards.sort();
        let mut runs = 0;
        let mut lone_cards = 0;
        for i in 0..sorted_cards.len() {
            runs += if i == 0 || sorted_cards[i - 1] != sorted_cards[i] {
                1
            } else {
                0
            };
            lone_cards += if i != 0 && sorted_cards[i - 1] == sorted_cards[i]
                || i + 1 < sorted_cards.len() && sorted_cards[i + 1] == sorted_cards[i]
            {
                0
            } else {
                1
            };
        }

        return if sorted_cards[0] == sorted_cards[4] {
            Hand {
                combination: CombinationType::Five,
                cards: original_cards,
            }
        } else if sorted_cards[0] == sorted_cards[3] || sorted_cards[1] == sorted_cards[4] {
            Hand {
                combination: CombinationType::Four,
                cards: original_cards,
            }
        } else if sorted_cards[0] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4]
            || sorted_cards[0] == sorted_cards[1] && sorted_cards[2] == sorted_cards[4]
        {
            Hand {
                combination: CombinationType::FullHouse,
                cards: original_cards,
            }
        } else if runs == 3 && lone_cards == 2 {
            Hand {
                combination: CombinationType::Three,
                cards: original_cards,
            }
        } else if runs == 3 {
            Hand {
                combination: CombinationType::TwoPair,
                cards: original_cards,
            }
        } else if runs == 4 {
            Hand {
                combination: CombinationType::OnePair,
                cards: original_cards,
            }
        } else {
            Hand {
                combination: CombinationType::High,
                cards: original_cards,
            }
        };
    }
}

struct JokerGame;
impl GameRules for JokerGame {
    fn card_order(card: u8) -> usize {
        for i in 0..JOKER_CARDS.len() {
            if JOKER_CARDS[i] == card {
                return i;
            }
        }
        panic!("unexpected card");
    }
    fn parse_hand(hand: &[u8]) -> Hand {
        if !hand.contains(&b'J') {
            return SimpleGame::parse_hand(hand);
        }

        let mut original_cards = [0 as u8; 5];
        original_cards.copy_from_slice(hand);

        let mut sorted_cards = original_cards;
        sorted_cards.sort();
        let mut jokers_count = 0;
        let mut runs = 0;
        let mut lone_cards = 0;
        for i in 0..sorted_cards.len() {
            jokers_count += if sorted_cards[i] == b'J' { 1 } else { 0 };
            runs += if i == 0 || sorted_cards[i - 1] != sorted_cards[i] {
                1
            } else {
                0
            };
            lone_cards += if i != 0 && sorted_cards[i - 1] == sorted_cards[i]
                || i + 1 < sorted_cards.len() && sorted_cards[i + 1] == sorted_cards[i]
            {
                0
            } else {
                1
            };
        }
        assert!(jokers_count > 0);

        return if runs <= 2 {
            Hand {
                combination: CombinationType::Five,
                cards: original_cards,
            }
        } else if runs == 3 && !(jokers_count == 1 && lone_cards == 1) {
            Hand {
                combination: CombinationType::Four,
                cards: original_cards,
            }
        } else if runs == 3 && jokers_count == 1 && lone_cards == 1 {
            Hand {
                combination: CombinationType::FullHouse,
                cards: original_cards,
            }
        } else if runs == 4 {
            Hand {
                combination: CombinationType::Three,
                cards: original_cards,
            }
        } else {
            Hand {
                combination: CombinationType::OnePair,
                cards: original_cards,
            }
        };
    }
}

fn beats<T: GameRules>(left: &Hand, right: &Hand) -> bool {
    if left.combination != right.combination {
        return left.combination < right.combination;
    }
    for i in 0..5 {
        if left.cards[i] != right.cards[i] {
            return T::card_order(left.cards[i]) < T::card_order(right.cards[i]);
        }
    }
    return false;
}

fn parse_bid<T: GameRules>(line: &[u8]) -> (Hand, i32, &[u8]) {
    let mut bid = 0;
    let hand = T::parse_hand(&line[..5]);

    let mut i = 6;
    while i < line.len() && line[i] != b'\n' {
        bid = 10 * bid + (line[i] - b'0') as i32;
        i += 1;
    }
    return (hand, bid, &line[(i + 1).min(line.len())..]);
}

fn parse_bid_str<T: GameRules>(line: &str) -> (Hand, i32) {
    let (hand, bid) = line.split_once(' ').unwrap();
    let hand = T::parse_hand(hand.as_bytes());

    return (hand, bid.parse::<i32>().unwrap());
}

// intentionally quadratic because I'm still afraid of allocations, sorry
fn day7<T: GameRules>(input: &[u8]) {
    let mut sum = 0;
    let mut first_line = input;
    while first_line.len() > 1 {
        let (current_hand, bid, remainder) = parse_bid::<T>(first_line);
        first_line = remainder;

        let mut position = 1;
        let mut second_line = input;
        while second_line.len() > 1 {
            let (other_hand, _, remainder) = parse_bid::<T>(second_line);
            second_line = remainder;
            if beats::<T>(&current_hand, &other_hand) {
                position += 1;
            }
        }
        sum += position * bid;
    }
    eprintln!("day07: {sum}");
}

fn day7_str<T: GameRules>(input: &str) {
    #![allow(unused)]
    let mut sum = 0;
    let mut first_line = input;
    for first in input.lines() {
        let (current_hand, bid) = parse_bid_str::<T>(first);
        let mut position = 1;
        for second in input.lines() {
            let (other_hand, _) = parse_bid_str::<T>(second);
            if beats::<T>(&current_hand, &other_hand) {
                position += 1;
            }
        }
        sum += position * bid;
    }
    eprintln!("day07: {sum}");
}

trait NodeExtensions {
    fn code(&self) -> usize;
}

impl NodeExtensions for &str {
    fn code(&self) -> usize {
        let bytes = self.as_bytes();
        return ((bytes[0] - b'A') as usize) * 26 * 26
            + ((bytes[1] - b'A') as usize) * 26
            + ((bytes[2] - b'A') as usize);
    }
}

fn day8(input: &str) {
    const TRIM_CHARS: &[char] = &[' ', '(', ')'];
    let mut transitions = [(0, 0); 26 * 26 * 26];

    let mut lines = input.lines();
    let commands = lines.next().unwrap().as_bytes();
    _ = lines.next().unwrap();

    let mut ghosts = [0; 16];
    let mut ghost_id = 0;

    for line in lines.clone() {
        let (source, targets) = line.split_once('=').unwrap();
        let (source, targets) = (source.trim().code(), targets.trim_matches(TRIM_CHARS));
        let (left, right) = targets.split_once(',').unwrap();
        let (left, right) = (left.trim().code(), right.trim().code());
        transitions[source] = (left, right);
        if source % 26 == 0 {
            assert!(ghost_id < ghosts.len(), "too many ghosts");
            ghosts[ghost_id] = source;
            ghost_id += 1;
        }
    }

    let ghosts = &mut ghosts[..ghost_id];

    let mut jumps = [0; 26 * 26 * 26];
    for line in lines.clone() {
        let (source, targets) = line.split_once('=').unwrap();
        let (source, targets) = (source.trim().code(), targets.trim_matches(TRIM_CHARS));
        let (left, right) = targets.split_once(',').unwrap();
        let (left, right) = (left.trim().code(), right.trim().code());
        for node in [source, left, right] {
            let mut current = node;
            for command in commands {
                current = if *command == b'L' {
                    transitions[current].0
                } else {
                    transitions[current].1
                };
            }
            jumps[node] = current;
        }
    }

    {
        let mut steps = 0;
        let mut node = 0;
        while node != 26 * 26 * 26 - 1 {
            let command = commands[steps % commands.len()];
            node = if command == b'L' {
                transitions[node].0
            } else {
                transitions[node].1
            };
            steps += 1;
        }
        eprintln!("day08: {steps}");
    }
    {
        for _ in 0..input.len() {
            for i in 0..ghosts.len() {
                ghosts[i] = jumps[ghosts[i]];
            }
        }
        let mut steps = commands.len() as i128;
        for i in 0..ghosts.len() {
            let mut cycle_len = 0;

            let (start, mut current) = (ghosts[i], ghosts[i]);
            loop {
                cycle_len += 1;
                current = jumps[current];
                if current == start {
                    break;
                }
            }
            steps *= cycle_len;
        }
        eprintln!("day08: {steps}");
    }
}

fn day8_hard_brute_force(input: &str) {
    #![allow(unused)]
    const TRIM_CHARS: &[char] = &[' ', '(', ')'];
    const MASK_SIZE: usize = 8;
    let mut transitions = [(0, 0); 26 * 26 * 26];

    let mut lines = input.lines();
    let commands = lines.next().unwrap().as_bytes();
    _ = lines.next().unwrap();

    let mut ghosts = [0; 16];
    let mut ghost_id = 0;

    for line in lines.clone() {
        let (source, targets) = line.split_once('=').unwrap();
        let (source, targets) = (source.trim().code(), targets.trim_matches(TRIM_CHARS));
        let (left, right) = targets.split_once(',').unwrap();
        let (left, right) = (left.trim().code(), right.trim().code());
        transitions[source] = (left, right);
        if source % 26 == 0 {
            assert!(ghost_id < ghosts.len(), "too many ghosts");
            ghosts[ghost_id] = source;
            ghost_id += 1;
        }
    }

    let ghosts = &mut ghosts[..ghost_id];

    let mut jumps = [0; 26 * 26 * 26];
    let mut masks = [[0 as u64; MASK_SIZE]; 26 * 26 * 26];
    for line in lines.clone() {
        let (source, targets) = line.split_once('=').unwrap();
        let (source, targets) = (source.trim().code(), targets.trim_matches(TRIM_CHARS));
        let (left, right) = targets.split_once(',').unwrap();
        let (left, right) = (left.trim().code(), right.trim().code());
        for node in [source, left, right] {
            let mut current = node;
            for (i, command) in commands.iter().enumerate() {
                if current % 26 == 25 {
                    masks[node][i / 64] |= 1 << (i % 64);
                }
                current = if *command == b'L' {
                    transitions[current].0
                } else {
                    transitions[current].1
                };
            }
            jumps[node] = current;
        }
    }

    {
        let mut steps_logged = 0;
        let mut steps = 0;
        'fast_loop: loop {
            if steps - steps_logged > 1_000_000_000 {
                steps_logged = steps;
                dbg!(steps);
            }
            'mask_loop: for i in 0..MASK_SIZE {
                let mut intersection = masks[ghosts[0]][i];
                for s in 1..ghosts.len() {
                    intersection &= masks[ghosts[s]][i];
                    if intersection == 0 {
                        continue 'mask_loop;
                    }
                }
                break 'fast_loop;
            }
            for i in 0..ghosts.len() {
                ghosts[i] = jumps[ghosts[i]];
            }
            steps += commands.len();
        }
        loop {
            let mut finished = true;
            for i in 0..ghosts.len() {
                finished &= ghosts[i] % 26 == 25;
            }
            if finished {
                break;
            }
            let command = commands[steps % commands.len()];
            steps += 1;
            for i in 0..ghosts.len() {
                ghosts[i] = if command == b'L' {
                    transitions[ghosts[i]].0
                } else {
                    transitions[ghosts[i]].1
                };
            }
        }
        eprintln!("day8 (bruteforce): {steps}");
    }
}

fn get_c(n: usize, k: usize) -> i128 {
    let mut result: i128 = 1;
    for i in n - k + 1..n + 1 {
        result *= i as i128;
    }
    for i in 1..k + 1 {
        result /= i as i128;
    }
    return result;
}

fn day9(input: &str) {
    let mut forward_prediction = 0;
    let mut backward_prediction = 0;
    for line in input.lines() {
        let count = line.split(' ').count();
        for (i, number) in line.split(' ').rev().enumerate() {
            let number = number.parse::<i128>().unwrap();
            forward_prediction += get_c(count, i + 1) * number * (if i % 2 == 0 { 1 } else { -1 });
        }
        for (i, number) in line.split(' ').enumerate() {
            let number = number.parse::<i128>().unwrap();
            backward_prediction += get_c(count, i + 1) * number * (if i % 2 == 0 { 1 } else { -1 });
        }
    }
    eprintln!("day09: {forward_prediction}");
    eprintln!("day09: {backward_prediction}");
}

trait GridPipes {
    fn can_go(&self, position: Point, direction: Point) -> bool;
}

type Pipe = i32;
const NONE: Pipe = 0b0000;
const ALL: Pipe = 0b1111;
const L: Pipe = 0b0001;
const T: Pipe = 0b0010;
const R: Pipe = 0b0100;
const B: Pipe = 0b1000;
const LR: Pipe = 0b0101;
const TB: Pipe = 0b1010;
const LT: Pipe = 0b0011;
const LB: Pipe = 0b1001;
const RT: Pipe = 0b0110;
const RB: Pipe = 0b1100;

fn pipe_from(symbol: u8) -> Pipe {
    return match symbol {
        b'S' => ALL,
        b'|' => TB,
        b'-' => LR,
        b'L' => RT,
        b'F' => RB,
        b'J' => LT,
        b'7' => LB,
        _ => NONE,
    };
}

impl<'a> GridPipes for Grid<'a> {
    fn can_go(&self, position: Point, direction: Point) -> bool {
        let (from, to) = (
            pipe_from(self.at(position)),
            pipe_from(self.at(position.add(direction))),
        );
        return match direction {
            Point { x: -1, y: 0 } => from & T > 0 && to & B > 0,
            Point { x: 1, y: 0 } => from & B > 0 && to & T > 0,
            Point { x: 0, y: -1 } => from & L > 0 && to & R > 0,
            Point { x: 0, y: 1 } => from & R > 0 && to & L > 0,
            _ => panic!("unexpected symbol"),
        };
    }
}

#[derive(Copy, Clone)]
struct GridCursor {
    previous: Point,
    current: Point,
}

trait GridWalk {
    fn walk(&self, cursor: GridCursor) -> Option<GridCursor>;
}

impl<'a> GridWalk for Grid<'a> {
    fn walk(&self, cursor: GridCursor) -> Option<GridCursor> {
        for &direction in DIRECTIONS_4 {
            if self.can_go(cursor.current, direction)
                && cursor.current.add(direction) != cursor.previous
            {
                return Some(GridCursor {
                    current: cursor.current.add(direction),
                    previous: cursor.current,
                });
            }
        }
        return None;
    }
}

fn day10(input: &[u8]) {
    let grid = Grid::new(input);
    let mut start = grid
        .index(input.iter().position(|&x| x == b'S').unwrap() as i32)
        .unwrap();
    let mut cursor = GridCursor {
        current: start,
        previous: start,
    };
    let mut step = 0;
    let mut corner = cursor;
    loop {
        cursor = grid.walk(cursor).unwrap();
        if cursor.current < corner.current {
            corner = cursor;
        }
        step += 1;
        if cursor.current == start {
            break;
        }
    }

    let mut signed_area = 0;
    let mut delta = 0;
    let mut corner_type = 1;
    (cursor, start) = (corner, corner.current);
    loop {
        let next = grid.walk(cursor).unwrap();
        let sign_in = cursor.current.y - cursor.previous.y;
        let sign_out = next.current.y - next.previous.y;
        signed_area += cursor.current.x * sign_out;
        if cursor.current.sub(cursor.previous) == next.current.sub(next.previous) {
            delta += 2;
        } else {
            if cursor.current != corner.current
                && next.current.sub(next.previous) == corner.current.sub(corner.previous)
            {
                corner_type = 4 - corner_type;
            }
            delta += corner_type;
            corner = cursor;
        }

        cursor = next;
        if cursor.current == start {
            break;
        }
    }
    let area = signed_area.abs() - delta / 4;
    eprintln!("day10: {}", step / 2);
    eprintln!("day10: {area}");
}

fn day11(input: &[u8]) {
    let grid = Grid::new(input);
    let galaxies = input.iter().filter(|&&b| b == b'#').count();
    let (mut distance_sum_small, mut distance_sum_large) = (0, 0);
    let (mut top_count, mut bottom_count) = (0, galaxies);
    for row in 0..grid.height {
        let mut row_count = 0;
        for col in 0..grid.width {
            if grid.at(Point { x: row, y: col }) == b'#' {
                row_count += 1;
            }
        }
        top_count += row_count;
        bottom_count -= row_count;
        distance_sum_small += top_count * bottom_count * if row_count == 0 { 2 } else { 1 };
        distance_sum_large += top_count * bottom_count * if row_count == 0 { 1000000 } else { 1 };
    }
    let (mut left_count, mut right_count) = (0, galaxies);
    for col in 0..grid.width {
        let mut col_count = 0;
        for row in 0..grid.height {
            if grid.at(Point { x: row, y: col }) == b'#' {
                col_count += 1;
            }
        }
        left_count += col_count;
        right_count -= col_count;
        distance_sum_small += left_count * right_count * if col_count == 0 { 2 } else { 1 };
        distance_sum_large += left_count * right_count * if col_count == 0 { 1000000 } else { 1 };
    }
    eprintln!("day11: {distance_sum_small}");
    eprintln!("day11: {distance_sum_large}");
}

struct RIter {
    sum: i32,
    count: i32,
    previous_index: i32,
    current_index: i32,
    current_count: i32,
}

impl RIter {
    fn new(sum: i32, count: i32) -> RIter {
        RIter {
            sum,
            count,
            previous_index: 0,
            current_index: 1,
            current_count: 0,
        }
    }
    fn valid(&mut self, rng: &mut SmallRng) -> bool {
        return self.current_count == self.count && self.next(rng).is_none();
    }
    fn next(&mut self, rng: &mut SmallRng) -> Option<i32> {
        while self.current_index <= self.sum + self.count {
            let chance = rng.gen_range(0..self.sum + self.count);
            if chance >= self.count {
                self.current_index += 1;
                continue;
            }
            let result = self.current_index - self.previous_index - 1;
            self.previous_index = self.current_index;
            self.current_index += 1;
            self.current_count += 1;
            return Some(result);
        }
        return None;
    }
}

fn validate_blocks(
    rng: &mut rand::rngs::SmallRng,
    records: &[u8],
    blocks: &str,
    distances: &mut RIter,
) -> bool {
    let mut valid = true;
    let mut current = 0;
    let mut lengths_iter = blocks.split(',').map(|x| x.parse::<i32>().unwrap());
    for i in 0..distances.count {
        let distance = distances.next(rng);
        if distance.is_none() {
            return false;
        }
        if !valid {
            continue;
        }

        let distance = distance.unwrap() as usize;
        valid &= !records[current..current + distance]
            .iter()
            .any(|&x| x == b'#');
        current += distance;

        let length = lengths_iter.next().unwrap() as usize;
        valid &= !records[current..current + length]
            .iter()
            .any(|&x| x == b'.');
        current += length;

        if i < distances.count - 1 {
            valid &= records[current] != b'#';
            current += 1;
        }
    }
    if records[current..].iter().any(|&x| x == b'#') {
        valid = false;
    }
    return valid;
}

fn day12(input: &str, precision: f64) {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut sum = 0;
    for line in input.lines() {
        let (records, blocks) = line.split_once(' ').unwrap();
        let blocks_count = blocks.split(',').count() as i32;
        let blocks_sum: i32 = blocks.split(',').map(|x| x.parse::<i32>().unwrap()).sum();
        let distances = records.len() as i32 - (blocks_count - 1) - blocks_sum;
        let (mut valid_samples, mut total_samples) = (0, 0);
        let total_count = get_c((distances + blocks_count) as usize, blocks_count as usize);
        loop {
            let mut riter = RIter::new(distances, blocks_count);
            let valid = validate_blocks(&mut rng, records.as_bytes(), blocks, &mut riter);
            if !riter.valid(&mut rng) {
                continue;
            }
            total_samples += 1;
            if valid {
                valid_samples += 1;
            }
            if total_samples % 1024 == 0 {
                let p = valid_samples as f64 / total_samples as f64;
                let stddev = (1.0 / total_samples as f64 * p * (1.0 - p)).sqrt();
                if total_samples > total_count && (stddev * total_count as f64) < precision {
                    break;
                }
            }
        }
        let valid_count = (valid_samples * total_count + total_samples / 2) / total_samples;
        sum += valid_count;
    }
    eprintln!("day12: ~{sum}");
}

// finally, we are starting alloc era!

fn day17_easy(input: &[u8]) {
    let grid = Grid::new(input);
    let mut visited = HashMap::new();
    let mut positions = BTreeSet::new();
    let mut distances = HashMap::new();
    let mut parents = HashMap::new();
    let (left_top, right_bottom) = (Point{ x: 0, y: 0 }, Point { x: grid.height - 1, y: grid.width - 2 });
    for &direction in DIRECTIONS_4 {
        positions.insert((0, left_top, direction));
    }
    while positions.len() > 0 {
        let (mut distance, point, direction) = positions.pop_first().unwrap();
        if visited.contains_key(&(point, direction)) {
            continue;
        }
        visited.insert((point, direction), ());
        let mut next_point = point;
        for _ in 0..3 {
            next_point = next_point.add(direction);
            if let Some(cost) = grid.at(next_point).digit() { distance += cost; } else { break };
            for &turn in DIRECTIONS_4.iter().filter(|&&x| x != direction && x.add(direction) != Point{x: 0, y: 0}) {
                let next = distances.get(&(next_point, turn));
                if next.is_none() || distance < *next.unwrap() {
                    distances.insert((next_point, turn), distance);
                    positions.insert((distance, next_point, turn));
                    parents.insert((next_point, turn), (point, direction));
                }
            }
        }
    }
//    let mut visual = Vec::new();
//    for _ in 0..right_bottom.y+1 {
//        let mut row = Vec::new();
//        for _ in 0..right_bottom.x+1 {
//            row.push(b'.');
//        }
//        visual.push(row);
//    }
//    let (mut current, mut d) = (right_bottom, DIRECTIONS_4[1]);
//    let mut i = 0;
//    let mut kek = 0;
//    while current != left_top {
//        dbg!(current);
//        let previous;
//        (previous, d) = *parents.get(&(current, d)).unwrap();
//        while previous != current {
//            let cell = &mut visual[current.x as usize][current.y as usize];
//            kek += grid.at(current).digit().unwrap();
//            if *cell == b'.' {
//                *cell = b'0' + (i % 10);
//            }
//            current = current.sub(d);
//            i += 1;
//        }
//    }
//    dbg!(kek);
//    for row in visual {
//        eprintln!("{}", String::from_utf8(row).unwrap());
//    }
    let best_distance = DIRECTIONS_4.iter().map(|&x| *distances.get(&(right_bottom, x)).unwrap()).min().unwrap();
    eprintln!("day17: {best_distance}");
}

fn day17_hard(input: &[u8]) {
    let grid = Grid::new(input);
    let mut visited = HashMap::new();
    let mut positions = BTreeSet::new();
    let mut distances = HashMap::new();
    let mut parents = HashMap::new();
    let (left_top, right_bottom) = (Point{ x: 0, y: 0 }, Point { x: grid.height - 1, y: grid.width - 2 });
    for &direction in DIRECTIONS_4 {
        positions.insert((0, left_top, direction));
    }
    while positions.len() > 0 {
        let (mut distance, point, direction) = positions.pop_first().unwrap();
        if visited.contains_key(&(point, direction)) {
            continue;
        }
        visited.insert((point, direction), ());
        let mut next_point = point;
        for i in 0..10 {
            next_point = next_point.add(direction);
            if let Some(cost) = grid.at(next_point).digit() { distance += cost; } else { break };
            if i < 3 { continue; }
            for &turn in DIRECTIONS_4.iter().filter(|&&x| x != direction && x.add(direction) != Point{x: 0, y: 0}) {
                let next = distances.get(&(next_point, turn));
                if next.is_none() || distance < *next.unwrap() {
                    distances.insert((next_point, turn), distance);
                    positions.insert((distance, next_point, turn));
                    parents.insert((next_point, turn), (point, direction));
                }
            }
        }
    }
    let best_distance = DIRECTIONS_4.iter().map(|&x| *distances.get(&(right_bottom, x)).unwrap()).min().unwrap();
    eprintln!("day17: {best_distance}");
}

#[derive(Debug)]
struct Trench {
    direction: Point,
    distance: i32,
}

fn paint(x: i32, y: i32, grid: &mut Vec<Vec<i32>>) {
    if x < 0 || y < 0 || x as usize >= grid.len() || y as usize>= grid[0].len() || grid[x as usize][y as usize] != 0 {
        return;
    }
    grid[x as usize][y as usize] = 2;
    for d in DIRECTIONS_4 {
        paint(x + d.x, y + d.y, grid);
    }
}

fn day18_easy(input: &'static str) {
    let mut trenches = Vec::new();
    for line in input.lines() {
        let mut tokens_iter = line.split(' ');
        let direction_char = tokens_iter.next().unwrap();
        let distance = tokens_iter.next().unwrap().parse::<i32>().unwrap();
        let direction = match direction_char {
            "U" => DIRECTIONS_4[0],
            "R" => DIRECTIONS_4[1],
            "D" => DIRECTIONS_4[2],
            "L" => DIRECTIONS_4[3],
            _ => unreachable!("invalid direction"),
        };
        trenches.push(Trench{ direction, distance });
    }
    let mut current_p = Point { x: 0, y: 0 };
    let (mut min_p, mut max_p) = (current_p, current_p);
    for trench in &trenches {
        current_p = current_p.add(trench.direction.mult(trench.distance)); 
        min_p.x = min_p.x.min(current_p.x);
        min_p.y = min_p.y.min(current_p.y);
        max_p.x = max_p.x.max(current_p.x);
        max_p.y = max_p.y.max(current_p.y);
    }
    let mut grid = Vec::new();
    for _ in min_p.x-1..=max_p.x+1 {
        grid.push(vec![0; (max_p.y - min_p.y + 3) as usize]);
    }
    let (min_x, min_y) = (min_p.x - 1, min_p.y - 1);

    current_p = Point { x: 0, y: 0 };
    for trench in &trenches {
        grid[(current_p.x - min_x) as usize][(current_p.y - min_y) as usize] = 1;
        for _ in 0..trench.distance {
            current_p = current_p.add(trench.direction);
            grid[(current_p.x - min_x) as usize][(current_p.y - min_y) as usize] = 1; 
        }
    }
    paint(0, 0, &mut grid);
    let mut area = 0;
    for row in grid {
        area += row.iter().filter(|&&x| x != 2).count();
    }
    eprintln!("day18: {area}");
}

fn day18_hard(input: &'static str) {
    let mut trenches = Vec::new();
    for line in input.lines() {
        let mut tokens_iter = line.split(' ');
        _ = tokens_iter.next().unwrap();
        _ = tokens_iter.next().unwrap().parse::<i32>().unwrap();
        let color = tokens_iter.next().unwrap();
        let color = &color[2..color.len() - 1];
        let distance = i32::from_str_radix(&color[..5], 16).unwrap();
        let direction = DIRECTIONS_4[((i64::from_str_radix(&color[5..], 16).unwrap() + 1) % 4) as usize];
        trenches.push(Trench{ direction, distance });
    }
     let mut current_p = Point { x: 0, y: 0 };
     let (mut xs, mut ys) = (vec![-1, 0, 1], vec![-1, 0, 1]);
     for trench in &trenches {
         current_p = current_p.add(trench.direction.mult(trench.distance)); 
         for d in -1..=1 {
             xs.push(current_p.x + d);
             ys.push(current_p.y + d);
         }
     }
     xs.sort(); xs.dedup();
     ys.sort(); ys.dedup();

     let mut grid = Vec::new();
     for _ in 0..xs.len() {
         grid.push(vec![0; ys.len()]);
     }
     current_p = Point { x: 0, y: 0 };
     for trench in &trenches {
         let mut start_x = xs.binary_search(&current_p.x).unwrap() as i32;
         let mut start_y = ys.binary_search(&current_p.y).unwrap() as i32;
         current_p = current_p.add(trench.direction.mult(trench.distance));
         let end_x = xs.binary_search(&current_p.x).unwrap() as i32;
         let end_y = ys.binary_search(&current_p.y).unwrap() as i32;
         while start_x != end_x || start_y != end_y {
             grid[start_x as usize][start_y as usize] = 1;
             start_x += (end_x - start_x).signum();
             start_y += (end_y - start_y).signum();
         }
     }
     paint(0, 0, &mut grid);
     let mut area = 0;
     for (x, row) in grid.iter().enumerate() {
         for (y, &cell) in row.iter().enumerate() {
             if cell == 2 {
                 continue;
             }
             area += (xs[x + 1] - xs[x]) as i64 * (ys[y + 1] - ys[y]) as i64;
         }
     }
     eprintln!("day18: {area}");
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    day1(include_str!("inputs/input01.txt").trim(), DAY1_EASY_PATTERN);
    day1(include_str!("inputs/input01.txt").trim(), DAY1_HARD_PATTERN);
    day2(
        include_str!("inputs/input02.txt").trim(),
        Bag {
            red: 12,
            green: 13,
            blue: 14,
        },
    );
    day3_easy(include_bytes!("inputs/input03.txt"));
    day3_hard(include_bytes!("inputs/input03.txt"));
    day4(include_str!("inputs/input04.txt").trim());
    day5(include_str!("inputs/input05.txt").trim());
    day6(include_str!("inputs/input06.txt").trim());
    day7::<SimpleGame>(include_bytes!("inputs/input07.txt"));
    day7::<JokerGame>(include_bytes!("inputs/input07.txt"));
    day7_str::<SimpleGame>(include_str!("inputs/input07.txt").trim());
    day8(include_str!("inputs/input08.txt").trim()); // day8_hard_brute_force(include_str!("inputs/input08.txt").trim());
    day9(include_str!("inputs/input09.txt").trim());
    day10(include_bytes!("inputs/input10.txt"));
    day11(include_bytes!("inputs/input11.txt"));
    day12(include_str!("inputs/input12.txt").trim(), 0.5);
    day17_easy(include_bytes!("inputs/input17.txt"));
    day17_hard(include_bytes!("inputs/input17.txt"));
    day18_easy(include_str!("inputs/input18.txt").trim());
    day18_hard(include_str!("inputs/example18.txt").trim());
    0
}
