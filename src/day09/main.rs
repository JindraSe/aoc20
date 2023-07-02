use std::{path::Path, fs::read_to_string, env::args};

fn is_sum_of_previous_n(nums: &Vec<u64>, index: usize, n: usize) -> bool {
    for i in 1..n+1 {
        for j in 1..i+1 {
            if nums[index] == nums[index - i] + nums[index - j] {
                return true;
            }
        }
    }

    return false;
}

fn find_first_violating(nums: &Vec<u64>, n: usize) -> Option<u64> {
    for i in n..nums.len() {
        if !is_sum_of_previous_n(nums, i, n) {
            return Some(nums[i]);
        }
    }

    return None;
}

fn find_range_summing_to(nums: &Vec<u64>, total: u64) -> Option<&[u64]> {
    for range_size in 2..nums.len() - 2 {
        for i in 0..nums.len() - range_size {
            if nums[i..i + range_size].iter().sum::<u64>() == total {
                return Some(&nums[i..i + range_size]);
            }
        }
    }

    return None;
}

fn load_numbers(path: &Path) -> Vec<u64> {
    return read_to_string(path)
        .expect("Input file not found")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);
    let numbers = load_numbers(&path_to_input);

    let first_violating = find_first_violating(&numbers, 25);
    println!(
        "The first number which violated the \
        required property is: {}", first_violating.expect("No such number found!")
    );

    let range_summing_to = find_range_summing_to(&numbers, first_violating.unwrap());
    let weakness = range_summing_to.unwrap().iter().copied().min().unwrap()
                    + range_summing_to.unwrap().iter().copied().max().unwrap();

    println!(
        "The weekness of this encrypted list of numbers is: {}",
        weakness
    );
}
