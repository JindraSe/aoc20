use std::{ ops::Index, fs::read_to_string, path::Path, env::args };

#[derive(PartialEq, Eq, Clone, Copy)]
enum SeatType {
    Floor,
    Empty,
    Occupied
}

impl SeatType {
    fn from_string(s: char) -> SeatType {
        match s {
            'L' => SeatType::Empty,
            '#' => SeatType::Occupied,
            '.' => SeatType::Floor,
            _ => panic!("Invalid input file")
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct SeatMap {
    width: isize,
    height: isize,
    vals: Vec<SeatType>
}

impl SeatMap {
    fn from_string(s: &str) -> SeatMap {
        let lines = s
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(SeatType::from_string));

        let mut height: isize = 0;
        let mut vals = Vec::new();

        for line in lines {
            for seat_type in line {
                vals.push(seat_type);
            }

            height += 1;
        }

        return SeatMap {
            width: isize::try_from(vals.len()).unwrap() / height,
            height,
            vals,
        }
    }

    fn count_occpuied(&self) -> usize {
        return self.vals
            .iter()
            .filter(|val| **val == SeatType::Occupied)
            .count();
    }

    fn in_bounds(&self, loc: (isize, isize)) -> bool {
        return loc.0 >= 0 && loc.1 >= 0
            && loc.0 < self.width && loc.1 < self.height;
    }

    fn count_occupied_neighbors(&self, loc: (isize, isize)) -> usize {
        return [
            (loc.0 - 1, loc.1 - 1), (loc.0, loc.1 - 1),
            (loc.0 + 1, loc.1 - 1), (loc.0 - 1, loc.1),
            (loc.0 + 1, loc.1), (loc.0 - 1, loc.1 + 1),
            (loc.0, loc.1 + 1), (loc.0 + 1, loc.1 + 1)
        ]   .iter()
            .filter(
                |neighbor| self.in_bounds(**neighbor)
                           && self[**neighbor] == SeatType::Occupied)
            .count();
    }

    fn run(&self, changed: &mut bool) -> SeatMap {
        *changed = false;

        let mut res = SeatMap {
            width: self.width,
            height: self.height,
            vals: Vec::new(),
        };

        res.vals.reserve(self.vals.len());

        for y in 0..self.height {
            for x in 0..self.width {
                res.vals.push(match self[(x, y)] {
                    SeatType::Floor => SeatType::Floor,

                    SeatType::Occupied => if self.count_occupied_neighbors((x, y)) >= 4 {
                        *changed = true;
                        SeatType::Empty
                    } else {
                        SeatType::Occupied
                    },

                    SeatType::Empty => if self.count_occupied_neighbors((x, y)) == 0 {
                        *changed = true;
                        SeatType::Occupied
                    } else {
                        SeatType::Empty
                    }
                })
            }
        }

        return res;
    }

    fn run_till_stable(&self) -> SeatMap {
        let mut changed = true;
        let mut current = self.run(&mut changed);

        while changed {
            current = current.run(&mut changed);
        }

        return current;
    }

    fn cast_vector(&self, from: (isize, isize), direction: (isize, isize)) -> Option<SeatType> {
        let mut current = (from.0 + direction.0, from.1 + direction.1);

        while self.in_bounds(current) {
            if self[current] != SeatType::Floor {
                return Some(self[current]);
            }

            current = (current.0 + direction.0, current.1 + direction.1);
        }

        return None;
    }

    fn count_far_occupied(&self, loc: (isize, isize)) -> usize {
        return [
            (-1,-1), (0, -1), (1,-1), (-1, 0),
            (1, 0), (-1, 1), (0, 1), (1, 1)
        ]   .iter()
            .map(|direction| self.cast_vector(loc, *direction))
            .filter(|maybe_seat| maybe_seat.is_some() && maybe_seat.unwrap() == SeatType::Occupied)
            .count();
    }

    fn run_far(&self, changed: &mut bool) -> SeatMap {
        *changed = false;

        let mut res = SeatMap {
            width: self.width,
            height: self.height,
            vals: Vec::new(),
        };

        res.vals.reserve(self.vals.len());

        for y in 0..self.height {
            for x in 0..self.width {
                res.vals.push(match self[(x, y)] {
                    SeatType::Floor => SeatType::Floor,

                    SeatType::Occupied => if self.count_far_occupied((x, y)) >= 5 {
                        *changed = true;
                        SeatType::Empty
                    } else {
                        SeatType::Occupied
                    },

                    SeatType::Empty => if self.count_far_occupied((x, y)) == 0 {
                        *changed = true;
                        SeatType::Occupied
                    } else {
                        SeatType::Empty
                    }
                })
            }
        }

        return res;
    }

    fn run_far_till_stable(&self) -> SeatMap {
        let mut changed = true;
        let mut current = self.run_far(&mut changed);

        while changed {
            current = current.run_far(&mut changed);
        }

        return current;

    }

}

impl Index<(isize, isize)> for SeatMap {
    type Output = SeatType;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        return &self.vals[usize::try_from(index.0 + index.1 * self.width).unwrap()];
    }
}

fn read_map(path: &Path) -> SeatMap {
    return SeatMap::from_string(
        &read_to_string(&path)
            .expect("File not found!")
    );
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument");
    }

    let path_to_input = Path::new(&args[1]);
    let map = read_map(&path_to_input);

    let stable = map.run_till_stable();
    println!("The number of occupied seats is: {}", stable.count_occpuied());

    let far_stable = map.run_far_till_stable();
    println!("The number of occupied seats when considering \
             far seats is {}", far_stable.count_occpuied());
}
