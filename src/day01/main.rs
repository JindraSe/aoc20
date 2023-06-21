use std::{fs::read_to_string, path::Path, env::args};


fn read_numbers(input_path: &Path) -> Vec<u32> {
    return read_to_string(input_path)
        .expect("Input file Not Found.")
        .lines()
        .map(String::from)
        .filter(|s| !s.is_empty())
        .map(|bytes| { String::from(bytes).parse::<u32>()
        .expect("Expected a number!") })
        .collect();
}

fn find_two(nums: &Vec<u32>) -> Option<(u32, u32)> {
    for (i, num) in nums.iter().enumerate()  {
        for other_num in nums[i..].iter() {
            if num + other_num == 2020 {
                return Some((num.clone(), other_num.clone()));
            }
        }
    }

    return None;
}

fn find_three(nums: &Vec<u32>) -> Option<(u32, u32, u32)> {

    for (i, first_num) in nums.iter().enumerate()  {
        for (j, second_num) in nums[i..].iter().enumerate() {
            for third_num in nums[j..].iter() {
                if first_num + second_num + third_num == 2020 {
                    return Some((first_num.clone(),
                                 second_num.clone(),
                                 third_num.clone()));
                }
            }
        }
    }

    return None;
}

fn find_summing_to(nums: &[u32], count: u32, total: u32) -> Option<Vec<u32>> {
    if count == 0 && total == 0 {
        return Some(Vec::new());
    }

    if count == 0 {
        return None;
    }

    for (i, num) in nums.iter().filter(|n| n <= &&total).enumerate() {
        match find_summing_to(&nums[i+1..], count - 1, total - num) {
            None => {
                continue;
            }

            Some(mut v) => {
                v.push(num.clone());
                return Some(v);
            }
        }
    }

    return None;
}


fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);

    let nums = read_numbers(path_to_input);

    match find_summing_to(&nums, 2, 2020) {
        None => {
            println!("Did not find any such numbers :-(");
        }

        Some(v) => {
            println!("Found numbers {} and {}, their product={}",
                     v[0], v[1], v[0] * v[1]);
        }
    }

    match find_summing_to(&nums, 3, 2020) {
        None => {
            println!("Did not find any such numbers :-(");
        }

        Some(v) => {
            println!("Found numbers {}, {} and {}, their product={}",
                     v[0], v[1], v[2], v[0] * v[1] * v[2]);
        }
    }
}
