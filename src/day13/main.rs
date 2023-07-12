use std::{ env::args, path::Path, fs::read_to_string, cmp::{ max, min } };


fn find_closest_gt_multiple(to: u32, of: u32) -> u32 {
    return of - to % of;
}

fn load_bus_info_task1(path: &Path) -> (u32, Vec<u32>) {
    let input = read_to_string(&path).expect("File not found");
    let lines: Vec<&str> = input
        .split('\n')
        .collect();

    if lines.len() < 2 {
        panic!("Badly formed input file!")
    }

    return (
        lines[0].parse().unwrap(),
        lines[1]
            .split(',')
            .filter(|symbol| *symbol != "x")
            .map(|symbol| symbol.parse().expect("File has wrong format!"))
            .collect()
    )
}

fn find_bezout_coefs(p: i128, q: i128) -> (i128, i128) {
    let (mut old_rem, mut new_rem) = (max(p, q), min(p, q));
    let (mut old_bcoef, mut new_bcoef) = (1, 0);
    let (mut old_smcoef, mut new_smcoef) = (0, 1);

    while new_rem != 0 {
        let quotient = old_rem / new_rem;
        (old_rem, new_rem) = (new_rem, old_rem - new_rem*quotient);
        (old_bcoef, new_bcoef) = (new_bcoef, old_bcoef - new_bcoef*quotient);
        (old_smcoef, new_smcoef) = (new_smcoef, old_smcoef - new_smcoef*quotient);
    }

    return if p > q {
        (old_bcoef, old_smcoef)
    } else {
        (old_smcoef, old_bcoef)
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Congruence {
    modulo: i128,
    value: i128
}

impl Congruence {
    fn new(modulo: i128, value: i128) -> Congruence {
        return Congruence {
            modulo,
            value: if value < 0 {
                modulo + (value % modulo)
            } else {
                value % modulo
            }
        };
    }
}

fn solve_congruence_system(x: Congruence, y: Congruence) -> Congruence {
    let (x_bezout, _) = find_bezout_coefs(x.modulo, y.modulo);
    let modulo = x.modulo * y.modulo;
    return Congruence::new(
        modulo,
        x.value + (y.value - x.value) * x.modulo * x_bezout,
    );
}

fn load_bus_info_task2(path: &Path) -> Vec<Congruence> {
    return read_to_string(&path)
        .expect("File not found!")
        .split('\n')
        .nth(1)
        .expect("File has wrong format!")
        .split(',')
        .enumerate()
        .map(|(i, symbol)| Congruence::new(
            match symbol {
                "x" => 1,
                num => num.parse().expect("File has wrong format!")
            },
            -(i as i128)
        ))
        .filter(|congruence| congruence.modulo != 1)
        .collect();
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);

    let (departure_timestamp, buses) = load_bus_info_task1(&path_to_input);
    let best_bus = buses.iter()
        .map(|bus| (bus, find_closest_gt_multiple(departure_timestamp, *bus)))
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .unwrap();

    println!("The answer to the first task is: {}", best_bus.0 * best_bus.1);

    let congruences = load_bus_info_task2(&path_to_input);
    let solution = congruences.iter().fold(
        Congruence { modulo: 1, value: 0 },
        |x, y| solve_congruence_system(x, *y)
    );

    println!("The answer to the second task is: {}", solution.value);
}
