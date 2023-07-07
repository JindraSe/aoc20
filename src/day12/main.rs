use std::{ env::args, path::Path, fs::read_to_string };

#[derive(Clone, Copy)]
enum ShipAction {
    North(u16),
    South(u16),
    East(u16),
    West(u16),
    Left(u16),
    Right(u16),
    Forward(u16),
}

impl ShipAction {
    fn from_string(s: &str) -> ShipAction {
        let (command, value) = s.split_at(1);
        let parsed_value = value.parse::<u16>().expect("Badly formed input file!");

        return match command {
            "N" => ShipAction::North(parsed_value),
            "S" => ShipAction::South(parsed_value),
            "E" => ShipAction::East(parsed_value),
            "W" => ShipAction::West(parsed_value),
            "L" => ShipAction::Left(parsed_value),
            "R" => ShipAction::Right(parsed_value),
            "F" => ShipAction::Forward(parsed_value),
            _ => panic!("Badly formed input file!")
        }
    }
}

struct ShipStatus {
    direction: u16,
    location: (i32, i32),
}

impl ShipStatus {
    fn new() -> ShipStatus {
        return ShipStatus { direction: 0, location: (0, 0) };
    }

    fn do_action(&self, action: ShipAction) -> ShipStatus {
        // finally some good pattern matching 😩
        return match action {
            ShipAction::North(by) => ShipStatus {
                direction: self.direction,
                location: (self.location.0, self.location.1 + by as i32)
            },

            ShipAction::South(by) => ShipStatus {
                direction: self.direction,
                location: (self.location.0, self.location.1 - by as i32)
            },

            ShipAction::East(by) => ShipStatus {
                direction: self.direction,
                location: (self.location.0 + by as i32, self.location.1)
            },

            ShipAction::West(by) => ShipStatus {
                direction: self.direction,
                location: (self.location.0 - by as i32, self.location.1)
            },

            ShipAction::Left(by) => ShipStatus {
                direction: (self.direction + by) % 360,
                location: self.location,
            },

            ShipAction::Right(by) => ShipStatus {
                direction: (self.direction + (360 - by)) % 360,
                location: self.location,
            },

            ShipAction::Forward(by) => ShipStatus {
                direction: self.direction,
                location: match self.direction {
                    0   => (self.location.0 + by as i32, self.location.1),
                    90  => (self.location.0, self.location.1 + by as i32),
                    180 => (self.location.0 - by as i32, self.location.1),
                    270 => (self.location.0, self.location.1 - by as i32),
                    _   => panic!("Invalid angle, going diagonally: {}!", self.direction)
                }
                
            }
        }
    }
}

struct ShipWaypointStatus {
    waypoint: (i32, i32),
    location: (i32, i32),
}

impl ShipWaypointStatus {
    fn new() -> ShipWaypointStatus {
        return ShipWaypointStatus { waypoint: (10, 1), location: (0, 0) }
    }

    fn do_action(&self, action: ShipAction) ->ShipWaypointStatus {
        return match action {
            ShipAction::North(by) => ShipWaypointStatus {
                waypoint: (self.waypoint.0, self.waypoint.1 + by as i32),
                location: self.location
            },

            ShipAction::South(by) => ShipWaypointStatus {
                waypoint: (self.waypoint.0, self.waypoint.1 - by as i32),
                location: self.location,
            },

            ShipAction::East(by) => ShipWaypointStatus {
                waypoint: (self.waypoint.0 + by as i32, self.waypoint.1),
                location: self.location,
            },

            ShipAction::West(by) => ShipWaypointStatus {
                waypoint: (self.waypoint.0 - by as i32, self.waypoint.1),
                location: self.location
            },

            ShipAction::Left(by) => ShipWaypointStatus {
                waypoint: match by % 360 {
                    0   => self.waypoint,
                    90  => (-self.waypoint.1, self.waypoint.0),
                    180 => (-self.waypoint.0, -self.waypoint.1),
                    270 => (self.waypoint.1, -self.waypoint.0),
                    _   => panic!("Invalid angle, going diagonally: {}!", by)
                },
                location: self.location,
            },

            ShipAction::Right(by) => ShipWaypointStatus {
                waypoint: match by % 360 {
                    0   => self.waypoint,
                    90  => (self.waypoint.1, -self.waypoint.0),
                    180 => (-self.waypoint.0, -self.waypoint.1),
                    270 => (-self.waypoint.1, self.waypoint.0),
                    _   => panic!("Invalid angle, going diagonally: {}!", by)
                },
                location: self.location,
            },

            ShipAction::Forward(by) => ShipWaypointStatus {
                waypoint: self.waypoint,
                location: (self.location.0 + self.waypoint.0*(by as i32),
                           self.location.1 + self.waypoint.1*(by as i32))
                
            }
        }
 
    }
}

fn read_actions(path: &Path) -> Vec<ShipAction> {
    return read_to_string(&path)
        .expect("Input file not found!")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(ShipAction::from_string)
        .collect();
}


fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument!");
    }

    let path_to_input = Path::new(&args[1]);
    let actions = read_actions(&path_to_input);

    let initial_status = ShipStatus::new();
    let final_status = actions.iter().fold(initial_status, |status, action| status.do_action(*action));

    println!("The Manhattan distance between the original and \
              final location is {}", final_status.location.0.abs() + final_status.location.1.abs());

    let initial_waypoint_status = ShipWaypointStatus::new();
    let final_waypoint_status = actions.iter().fold(
        initial_waypoint_status, |status, action| status.do_action(*action)
    );

    println!("The Manhattan distance between the original and the final \
              location using the waypoint is: {}", 
              final_waypoint_status.location.0.abs() + final_waypoint_status.location.1.abs())
}
