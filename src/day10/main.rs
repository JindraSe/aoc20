use std::{path::Path, env::args, fs::read_to_string};

// the parameter must be a sorted vector
fn count_differences(joltage_ratings: &Vec<u32>) -> [u32; 3] {
    let mut jolt_differences: [u32; 3] = [0, 0, 1];
    let mut last_rating = 0;

    for rating in joltage_ratings {
        let diff = rating - last_rating;
        if diff < 1 || diff > 3 {
            panic!("Invalid input!");
        }

        jolt_differences[diff as usize - 1] += 1;
        last_rating = rating.clone();
    }

    return jolt_differences;
}

fn count_arrangements(joltage_ratings: &Vec<u32>) -> u64 {
    let mut previous_three: [(u32, u64); 3] = [(0, 0), (0, 0), (0, 1)];

    for rating in joltage_ratings {
        let rating_arrangements: u64 = previous_three.iter().map(
            |(prev_rating, arrangements)| if rating - prev_rating < 4 {
                arrangements.clone()
            } else {
                0
            }
        ).sum();

        previous_three = [previous_three[1], previous_three[2], (rating.clone(), rating_arrangements)];
    }

    return previous_three[2].1;
}

fn load_joltages(path: &Path) -> Vec<u32> {
    let mut res = read_to_string(path)
        .expect("Input file not found")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    res.sort();

    return res;
}


fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument");
    }

    let path_to_input = Path::new(&args[1]);
    let ratings = load_joltages(&path_to_input);

    let diffs = count_differences(&ratings);
    println!("The answer to the first task is: {}", diffs[0] * diffs[2]);

    let arrangement_count = count_arrangements(&ratings);
    println!("The total number of possible arrangements is: {}", arrangement_count);
}
