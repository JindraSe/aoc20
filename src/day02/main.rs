use std::{fs::read_to_string, path::Path, env::args};

struct Rule {
    letter: char,
    low: u8,
    high: u8,
}

impl Rule {
    fn satisfied_by_sled(&self, word: &str) -> bool {
        let count: u8 = u8::try_from(
            word.chars().filter(|ch| ch.clone() == self.letter).count()
        ).expect("Password was to large");

        return count >= self.low && count <= self.high;
    }

    fn satisfied_by_toboggan(&self, word: &str) -> bool {
        let first_matches = word.chars().nth(usize::from(self.low) - 1) == Some(self.letter);
        let second_matches = word.chars().nth(usize::from(self.high) - 1) == Some(self.letter);

        return first_matches != second_matches;
    }
}

fn parse_low_high(range_str: &str) -> Option<(u8, u8)> {
    let split_str: Vec<&str> = range_str.split('-').collect();

    if split_str.len() != 2 {
        return None;
    }

    let low = split_str[0].parse::<u8>();
    let high = split_str[1].parse::<u8>();

    match (low, high) {
        (Err(_), _) => {
            return None;
        }

        (_, Err(_)) => {
            return None;
        }

        (Ok(low_value), Ok(high_value)) => {
            return Some((low_value, high_value));
        }

    }
}

fn parse_rule(rule_str: &str) -> Option<Rule> {
    let split_str: Vec<&str> = rule_str.split(' ').collect();

    if split_str.len() != 2 || split_str[1].len() != 1 {
        return None;
    }

    let low_high = parse_low_high(split_str[0]);
    let letter = split_str[1].chars().nth(0);

    match (low_high, letter) {
        (_, None) => {
            return None;
        }

        (None, _) => {
            return None;
        }

        (Some((low, high)), Some(l)) => {
            return Some(Rule {
                letter: l,
                low,
                high
            });
        }
    }
}

fn parse_line(line: &str) -> Option<(Rule, String)> {
    let split_line: Vec<&str> = line.split(": ").collect();

    if split_line.len() != 2 {
        return None;
    }

    let rule = parse_rule(split_line[0]);
    let word = split_line[1];

    match rule {
        Some(r) => {
            return Some((r, word.to_string()));
        }

        None => {
            return None;
        }
    }
}

fn read_lines(path: &Path) -> Vec<(Rule, String)> {
    return read_to_string(path)
        .expect("Input file Not Found")
        .lines()
        .map(parse_line)
        .filter(|line| line.is_some())
        .map(|line| line.unwrap())
        .collect();
}

fn count_sled_valid(rules_with_words: &Vec<(Rule, String)>) -> usize {
    return rules_with_words
        .iter()
        .filter(|(r, word)| r.satisfied_by_sled(word))
        .count();
}

fn count_toboggan_valid(rules_with_words: &Vec<(Rule, String)>) -> usize {
    return rules_with_words
        .iter()
        .filter(|(r, word)| r.satisfied_by_toboggan(word))
        .count();
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);

    let parsed_lines = read_lines(path_to_input);

    println!("Number of valid passwords (sled store): {}", count_sled_valid(&parsed_lines));
    println!("Number of valid passwords (toboggan): {}", count_toboggan_valid(&parsed_lines));
}
