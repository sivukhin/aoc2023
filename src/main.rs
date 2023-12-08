#![feature(start)]

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
        Grid{ table, width, height: height - 1 }
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
        assert!(wins_count < copies_count.len());
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
            let [dst_range_start, src_range_start, range_len] = split_exact::<3>(line.trim(), ' ').unwrap();
            let (dst_range_start, src_range_start, range_len) = (
                dst_range_start.parse::<i64>().unwrap(), 
                src_range_start.parse::<i64>().unwrap(), 
                range_len.parse::<i64>().unwrap()
            );
            if !mapped && src_range_start <= seed && seed < src_range_start + range_len {
                seed = dst_range_start + (seed - src_range_start);
                mapped = true;
            }
        }
        min_location = Some(min_location.unwrap_or(seed).min(seed));
    }

    let min_location = min_location.unwrap();
    eprintln!("day5: {min_location}");
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
    eprintln!("day6: {mult}");
    let mega_options = count_winning_options(mega_time, mega_distance);
    eprintln!("day6: {mega_options}");
}

#[derive(PartialEq, PartialOrd, Debug)]
enum CombinationType { Five, Four, FullHouse, Three, TwoPair, OnePair, High }
struct Hand {
    combination: CombinationType,
    cards: [u8; 5]
}

const SIMPLE_CARDS: &[u8] = &[b'A', b'K', b'Q', b'J', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2'];
const JOKER_CARDS: &[u8] = &[b'A', b'K', b'Q', b'T', b'9', b'8', b'7', b'6', b'5', b'4', b'3', b'2', b'J'];
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
            runs += if i == 0 || sorted_cards[i - 1] != sorted_cards[i] { 1 } else { 0 };
            lone_cards += if i != 0 && sorted_cards[i - 1] == sorted_cards[i] || i + 1 < sorted_cards.len() && sorted_cards[i + 1] == sorted_cards[i] { 0 } else { 1 };
        }

        return if sorted_cards[0] == sorted_cards[4] {
            Hand { combination: CombinationType::Five, cards: original_cards }
        } else if sorted_cards[0] == sorted_cards[3] || sorted_cards[1] == sorted_cards[4] {
            Hand { combination: CombinationType::Four, cards: original_cards }
        } else if sorted_cards[0] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4] || 
                  sorted_cards[0] == sorted_cards[1] && sorted_cards[2] == sorted_cards[4] {
            Hand { combination: CombinationType::FullHouse, cards: original_cards }
        } else if runs == 3 && lone_cards == 2 {
            Hand { combination: CombinationType::Three, cards: original_cards }
        } else if runs == 3 {
            Hand { combination: CombinationType::TwoPair, cards: original_cards }
        } else if runs == 4 {
            Hand { combination: CombinationType::OnePair, cards: original_cards }
        } else {
            Hand { combination: CombinationType::High, cards: original_cards }
        }
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
            return SimpleGame::parse_hand(hand)
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
            runs += if i == 0 || sorted_cards[i - 1] != sorted_cards[i] { 1 } else { 0 };
            lone_cards += if i != 0 && sorted_cards[i - 1] == sorted_cards[i] || i + 1 < sorted_cards.len() && sorted_cards[i + 1] == sorted_cards[i] { 0 } else { 1 };
        }
        assert!(jokers_count > 0);

        return if runs <= 2 {
            Hand { combination: CombinationType::Five, cards: original_cards }
        } else if runs == 3 && !(jokers_count == 1 && lone_cards == 1) {
            Hand { combination: CombinationType::Four, cards: original_cards }
        } else if runs == 3 && jokers_count == 1 && lone_cards == 1 {
            Hand { combination: CombinationType::FullHouse, cards: original_cards }
        } else if runs == 4 {
            Hand { combination: CombinationType::Three, cards: original_cards }
        } else {
            Hand { combination: CombinationType::OnePair, cards: original_cards }
        }
    }
}

fn beats<T: GameRules>(left: &Hand, right: &Hand) -> bool {
    if left.combination != right.combination {
        return left.combination < right.combination;
    }
    for i in 0..5 {
        if left.cards[i] != right.cards[i] {
            return T::card_order(left.cards[i]) < T::card_order(right.cards[i])
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
    return (hand, bid, &line[(i+1).min(line.len())..]);
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
    eprintln!("day7: {sum}");
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
    eprintln!("day7: {sum}");
}

trait NodeExtensions {
    fn code(&self) -> usize;
}

impl NodeExtensions for &str {
    fn code(&self) -> usize {
        let bytes = self.as_bytes();
        return ((bytes[0] - b'A') as usize) * 26 * 26 + 
               ((bytes[1] - b'A') as usize) * 26 + 
               ((bytes[2] - b'A') as usize);
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
                current = if *command == b'L' { transitions[current].0 } else { transitions[current].1 }; 
            }
            jumps[node] = current;
        }
    }

    {
        let mut steps = 0;
        let mut node = 0;
        while node != 26 * 26 * 26 - 1 {
            let command = commands[steps % commands.len()];
            node = if command == b'L' { transitions[node].0 } else { transitions[node].1 };
            steps += 1;
        }
        eprintln!("day8: {steps}");
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
        eprintln!("day8: {steps}");
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    day1(include_str!("inputs/input01.txt").trim(), DAY1_EASY_PATTERN);
    day1(include_str!("inputs/input01.txt").trim(), DAY1_HARD_PATTERN); 
    day2(include_str!("inputs/input02.txt").trim(), Bag{ red: 12, green: 13, blue: 14 });
    day3_easy(include_bytes!("inputs/input03.txt"));
    day3_hard(include_bytes!("inputs/input03.txt"));
    day4(include_str!("inputs/input04.txt").trim());
    day5(include_str!("inputs/input05.txt").trim());
    day6(include_str!("inputs/input06.txt").trim());
    day7::<SimpleGame>(include_bytes!("inputs/input07.txt"));
    day7::<JokerGame>(include_bytes!("inputs/input07.txt"));
    day7_str::<SimpleGame>(include_str!("inputs/input07.txt").trim());
    day8(include_str!("inputs/input08.txt").trim());
    0
}
