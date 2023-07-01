use std::{path::Path, fs::read_to_string, collections::{HashSet, VecDeque}, env::args, str::FromStr};

#[derive(Clone, Copy, PartialEq)]
enum Operation {
    Accumulate,
    Jump,
    NoOperation,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "acc" => Ok(Operation::Accumulate),
            "jmp" => Ok(Operation::Jump),
            "nop" => Ok(Operation::NoOperation),
            _ => Err(format!("Invalid Operation Name: {}", s))
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Instruction {
    op: Operation,
    arg: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_str: Vec<&str> = s.split(' ').collect();

        if split_str.len() != 2 {
            return Err(format!("The instruction is malformed: {}", s));
        }

        let maybe_op = Operation::from_str(&split_str[0]);
        let maybe_arg = split_str[1].parse::<i32>();

        if maybe_op.is_err() {
            return Err(maybe_op.err().unwrap());
        }

        if maybe_arg.is_err() {
            return Err(format!("Could not parse number: {}", split_str[1]));
        }

        return Ok(Instruction {
            op: maybe_op.ok().unwrap(),
            arg: maybe_arg.ok().unwrap()
        });
    }
}

#[derive(Clone)]
struct MachineState {
    accumulator: i32,
    program_counter: i32,
}

impl MachineState {
    fn new() -> MachineState {
        return MachineState { accumulator: 0, program_counter: 0 }
    }

    fn execute_instruction(&self, instruction: &Instruction) -> MachineState {
        return match instruction.op {
            Operation::Accumulate => MachineState {
                accumulator: self.accumulator + instruction.arg,
                program_counter: self.program_counter + 1,
            },
            Operation::Jump => MachineState {
                accumulator: self.accumulator,
                program_counter: self.program_counter + instruction.arg
            },
            Operation::NoOperation => MachineState {
                accumulator: self.accumulator,
                program_counter: self.program_counter + 1
            }
        }
    }

    fn run_program_until_loop(program: &Vec<Instruction>) -> MachineState {
        let mut seen_instructions: HashSet<i32> = HashSet::new();
        let mut state = MachineState::new();

        while !seen_instructions.contains(&state.program_counter) {
            // println!("Current state: {:?}", state);
            seen_instructions.insert(state.program_counter);
            state = state.execute_instruction(&program[state.program_counter as usize]);
        }

        return state;
    }

    fn run_program_and_try_fixing(program: &Vec<Instruction>) -> MachineState {
        #[derive(Clone)]
        struct PossibleState {
            state: MachineState,
            seen: HashSet<i32>,
            flipped: bool,
        }

        let mut possible_states: VecDeque<PossibleState> = VecDeque::new();
        let mut current_state = PossibleState {
            state: MachineState::new(),
            seen: HashSet::new(),
            flipped: false,
        };

        while current_state.state.program_counter as isize != program.len() as isize {
            if current_state.seen.contains(&current_state.state.program_counter) {
                // this state is stuck in a loop - we can get rid of it
                current_state = possible_states.pop_front().expect(
                    "The program cannot be fixed!"
                );
                continue;
            }
            current_state.seen.insert(current_state.state.program_counter);

            let current_instruction: Instruction = program[
                current_state.state.program_counter as usize
            ];

            if current_instruction.op == Operation::Accumulate || current_state.flipped {
                current_state.state = current_state.state.execute_instruction(&current_instruction);
                possible_states.push_back(current_state);
                current_state = possible_states.pop_front().unwrap(); // the queue will never be
                                                                      // empty here
                continue;
            }

            let jump_instruction = Instruction { op: Operation::Jump, arg: current_instruction.arg };
            let next_state_jump = PossibleState {
                state: current_state.state.execute_instruction(&jump_instruction),
                seen: current_state.seen.clone(),
                flipped: current_instruction.op != Operation::Jump,
            };

            let nop_instruction = Instruction { op: Operation::NoOperation, arg: current_instruction.arg };
            let next_state_nop = PossibleState {
                state: current_state.state.execute_instruction(&nop_instruction),
                seen: current_state.seen,
                flipped: current_instruction.op != Operation::NoOperation,
            };

            possible_states.push_back(next_state_jump);
            possible_states.push_back(next_state_nop);
            current_state = possible_states.pop_front().unwrap();
        }

        return current_state.state;
    }
}


fn load_instructions(path: &Path) -> Vec<Instruction> {
    return read_to_string(path)
        .expect("Input file not found")
        .split('\n')
        .map(Instruction::from_str)
        .filter(|maybe_instruction| maybe_instruction.is_ok())
        .map(|maybe_instruction| maybe_instruction.ok().unwrap())
        .collect();
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument");
    }

    let path_to_input = Path::new(&args[1]);
    let program = load_instructions(path_to_input);

    let state_after_loop = MachineState::run_program_until_loop(&program);
    println!(
        "The state of the accumulator \
        before an instruction is executed twice is {}",
        state_after_loop.accumulator
    );

    let termination_state = MachineState::run_program_and_try_fixing(&program);
    println!(
        "The state of the accumulator of the fixed program after termination is {}",
        termination_state.accumulator
    );
}
