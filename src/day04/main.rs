use std::{path::Path, fs::read_to_string, env::args};

struct CredentialsFieldsPresence {
    birth: bool,
    issue: bool,
    expires: bool,
    height: bool,
    hair: bool,
    eyes: bool,
    passport: bool,
    country: bool,
}

impl CredentialsFieldsPresence {
    fn should_accept(&self) -> bool {
        return self.birth && self.issue && self.expires && self.height
                && self.hair && self.eyes && self.passport;
    }
}

fn validate_year(year: &str, min_year: i16, max_year: i16) -> bool {
    if year.len() != 4 {
        return false;
    }

    let parsed_birth = year.parse::<i16>();

    if parsed_birth.is_err() {
        return false;
    }

    let unwrapped_birth = parsed_birth.unwrap();

    return unwrapped_birth >= min_year && unwrapped_birth <= max_year;
}

fn validate_height(height: &str) -> bool {
    if height.len() == 4 {
        let (value, unit) = height.split_at(2);

        if unit != "in" {
            return false;
        }

        let parsed_value = value.parse::<u8>();

        if parsed_value.is_err() {
            return false;
        }

        let unwrapped_value = parsed_value.unwrap();

        return unwrapped_value >= 59 && unwrapped_value <= 76;
    }

    if height.len() != 5 {
        return false;
    }

    let (value, unit) = height.split_at(3);

    if unit != "cm" {
        return false;
    }

    let parsed_value = value.parse::<u8>();

    if parsed_value.is_err() {
        return false;
    }

    let unwrapped_value = parsed_value.unwrap();

    return unwrapped_value >= 150 && unwrapped_value <= 193;
}

fn validate_hair_color(color: &str) -> bool {
    if color.len() != 7 {
        return false;
    }

    if color.chars().nth(0) != Some('#') {
        return false;
    }

    return color[1..].chars().all(|ch| ch.is_ascii_digit() || (ch >= 'a' && ch <= 'f'));
}

fn validate_eye_color(color: &str) -> bool {
    return color == "amb" || color == "blu" || color == "brn"
        || color == "gry" || color == "grn" || color == "hzl"
        || color == "oth";
}

fn validate_passport(passport: &str) -> bool {
    return passport.len() == 9 && passport.chars().all(|ch| ch.is_ascii_digit());
}

fn check_for_fields(record: &str, validate: bool) -> CredentialsFieldsPresence {
    let mut result = CredentialsFieldsPresence {
        birth: false, issue: false, expires: false,
        height: false, hair: false, eyes: false,
        passport: false, country: false
    };

    for field in record.split_whitespace() {
        let split_field: Vec<&str> = field.split(":").collect();

        if split_field.len() != 2 {
            panic!("Malformed input file.");
        }

        match split_field[0] {
            "byr" => {
                result.birth = !validate || validate_year(split_field[1], 1920, 2002);
            }
            "iyr" => {
                result.issue = !validate || validate_year(split_field[1], 2010, 2020);
            }
            "eyr" => {
                result.expires = !validate || validate_year(split_field[1], 2020, 2030);
            }
            "hgt" => {
                result.height = !validate || validate_height(split_field[1]);
            }
            "hcl" => {
                result.hair = !validate || validate_hair_color(split_field[1]);
            }
            "ecl" => {
                result.eyes = !validate || validate_eye_color(split_field[1]);
            }
            "pid" => {
                result.passport = !validate || validate_passport(split_field[1]);
            }
            "cid" => {
                result.country = true;
            }
            _ => {}
        }
    }

    return result;
}

fn count_records(path: &Path, validate: bool) -> usize {
    return read_to_string(path)
        .expect("Input file not found")
        .split("\n\n")
        .map(|record| check_for_fields(record, validate))
        .filter(|cfp| cfp.should_accept())
        .count();
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);

    let count_without_validation = count_records(path_to_input, false);
    let count_with_validation = count_records(path_to_input, true);

    println!("Number of good passports: {}", count_without_validation);
    println!("Number of good passports (validated): {}", count_with_validation);
}
