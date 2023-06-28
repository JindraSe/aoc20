use std::{path::Path, fs::read_to_string, collections::{HashSet, HashMap}, env::args};

struct BagRule {
    container_bag: String,
    children: Vec<(String, usize)>,
}

impl BagRule {
    fn to_map(bags: &Vec<BagRule>) -> HashMap<String, Vec<(String, usize)>> {
        let mut result = HashMap::new();

        for rule in bags {
            result.insert(rule.container_bag.clone(), rule.children.clone());
        }

        return result;
    }
}

fn skip_last(string_slice: &str, count: usize) -> &str {
    return &string_slice[..string_slice.len() - count];
}

fn parse_child_bag(child_bag: &str) -> (String, usize) {
    let split_child_bag: Vec<&str> = child_bag.splitn(2, ' ').collect();

    if split_child_bag.len() != 2 {
        panic!("Wrong format of a child bag - no space!");
    }

    let count = str::parse::<usize>(
        split_child_bag[0]
    ).expect("Wrong format of a child bag - not a number!");

    if count == 1 {
        return (skip_last(&split_child_bag[1], 4).to_string().to_string(), count);
    }

    return (skip_last(&split_child_bag[1], 5).to_string(), count);
}

fn parse_bag_rule(line: &str) -> BagRule {
    let split_line: Vec<&str> = line.split(" contain ").collect();

    if split_line.len() != 2 {
        panic!("Wrong format of a bag rule - no space!");
    }

    let container_bag = skip_last(split_line[0], 5).to_string();
    let children = if split_line[1] == "no other bags." {
        Vec::new()
    } else {
        skip_last(split_line[1], 1).split(", ").map(parse_child_bag).collect()
    };

    return BagRule { container_bag, children };
}

fn load_rules(path: &Path) -> Vec<BagRule> {
    return read_to_string(path)
        .expect("File not found")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_bag_rule)
        .collect();
}

fn count_all_that_can_recursively_contain(bags: &Vec<BagRule>, name: &str) -> usize {
    let mut candidates: HashSet<String> = HashSet::new();
    let mut previous_size: usize = 0;

    candidates.insert(name.to_string());

    while candidates.len() > previous_size {
        previous_size = candidates.len();

        for rule in bags {
            if candidates.contains(&rule.container_bag) {
                continue;
            }

            if rule.children.iter().map(|(name, _)| name).any(|name| candidates.contains(name)) {
                candidates.insert(rule.container_bag.to_string());
            }
        }

    }

    return candidates.len() - 1;
}

fn all_contained_by(bags: &Vec<BagRule>, name: &str) -> usize {
    let bag_map = BagRule::to_map(bags);

    let mut total_count = 0;

    let mut current_layer: Vec<(String, usize)> = vec![(name.to_string(), 1)];
    let mut next_layer: Vec<(String, usize)> = Vec::new();

    while !current_layer.is_empty() {
        for (name, count) in current_layer {
            for (child_name, child_count) in bag_map[&name].iter() {
                next_layer.push((child_name.clone(), child_count * count));
            }
        }

        total_count += next_layer.iter().map(|(_, count)| count).sum::<usize>();
        current_layer = next_layer;
        next_layer = Vec::new();
    }

    return total_count;
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument");
    }

    let path_to_input = Path::new(&args[1]);

    let rules = load_rules(path_to_input);

    let containing_count = count_all_that_can_recursively_contain(&rules, "shiny gold");
    println!("The count of possible outer bag colors is: {}", containing_count);

    let contained_count = all_contained_by(&rules, "shiny gold");
    println!("The shiny gold bag will contain {} bags.", contained_count);
}
