use std::{path::Path, fs::read_to_string, env::args, collections::HashSet};

struct Seat {
    row: u16,
    column: u16,
}

impl Seat {
    fn from_code(code: &str) -> Option<Seat> {
        if code.len() != 10 {
            return None;
        }

        let mut row: u16 = 0;
        let mut column: u16 = 0;

        let mut row_partition: u16 = 64;
        for ch in code[..7].chars() {
            if ch == 'B' {
                row += row_partition;
            }

            row_partition >>= 1;
        }

        let mut column_partition: u16 = 4;
        for ch in code[7..].chars() {
            if ch == 'R' {
                column += column_partition;
            }

            column_partition >>= 1;
        }

        return Some(Seat { row, column });
    }

    fn seat_id(&self) -> u16 {
        return 8*self.row + self.column;
    }
}

fn load_seats(path: &Path) -> Vec<Seat>{
    return read_to_string(path)
        .expect("Input file not found")
        .split('\n')
        .map(Seat::from_code)
        .filter(|res| res.is_some())
        .map(|res| res.unwrap())
        .collect();
}

fn find_id_of_missing_seat(seats: &Vec<Seat>) -> u16 {
    let seen_seat_ids: HashSet<u16> = HashSet::from_iter(
        seats.iter().map(|seat| seat.seat_id())
    );

    for id in 1..(8*128 + 8) {
        if !seen_seat_ids.contains(&id)
                && seen_seat_ids.contains(&(id - 1))
                && seen_seat_ids.contains(&(id + 1)) {
            return id;
        }
    }

    return 0;
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);

    let seats = load_seats(path_to_input);

    println!("The greates seat ID is: {}", seats.iter().map(|seat| seat.seat_id()).max().unwrap_or(0));
    println!("The ID of the missing seat is: {}", find_id_of_missing_seat(&seats));
}
