use std::{path::Path, fs::read_to_string, env::args};

struct AnswerSet {
    set: u32,
}

impl Clone for AnswerSet {
    fn clone(&self) -> Self {
        return AnswerSet { set: self.set.clone() };
    }
}

impl AnswerSet {
    fn add(&mut self, letter: char) {
        if !letter.is_ascii_lowercase() {
            panic!("Expected only lower case ascii letters as answers!");
        }

        self.set |= 0x1 << ((letter as u8) - ('a' as u8)); 
    }

    fn pair_union(&self, other: &AnswerSet) -> AnswerSet {
        return AnswerSet { set: self.set | other.set };
    }

    fn many_union(answer_sets: &Vec<AnswerSet>) -> AnswerSet {
        let mut result = AnswerSet { set: 0 };

        for answer_set in answer_sets {
            result = result.pair_union(answer_set);
        }

        return result;
    }

    fn pair_intersect(&self, other: &AnswerSet) -> AnswerSet {
        return AnswerSet { set: self.set & other.set };
    }

    fn many_intersect(answer_sets: &Vec<AnswerSet>) -> AnswerSet {
        if answer_sets.len() == 0 {
            return AnswerSet { set: 0 };
        }

        let mut result = answer_sets[0].clone();

        for answer_set in answer_sets {
            result = result.pair_intersect(answer_set);
        }

        return result;
    }

    fn count(&self) -> u8 {
        let mut result = 0;

        for i in 0..26 {
            if self.set & (0x1 << i) != 0 {
                result += 1;
            }
        }

        return result;
    }

    fn from_answers(answers: &str) -> AnswerSet {
        let mut result = AnswerSet { set: 0 };

        for ch in answers.chars() {
            result.add(ch);
        }

        return result;
    }
}

fn load_groups(path: &Path) -> Vec<Vec<AnswerSet>>{
    return read_to_string(path)
        .expect("File not found")
        .split("\n\n")
        .map(|group|
             group
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(AnswerSet::from_answers)
                .collect())
        .collect();
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);

    let groups = load_groups(&path_to_input);

    let group_union_count = groups.iter()
        .map(AnswerSet::many_union)
        .map(|answer| answer.count() as u32)
        .sum::<u32>();

    println!("The sum of union group counts is: {}", group_union_count);

    let group_intersect_count = groups.iter()
        .map(AnswerSet::many_intersect)
        .map(|answer| answer.count() as u32)
        .sum::<u32>();

    println!("The sum of intersection group counts is: {}", group_intersect_count);
}
