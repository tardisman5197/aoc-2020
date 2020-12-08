use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

// Op defines the possible Operations
// which can take place.
#[derive(Debug)]
pub enum Op {
    // NOP has no operation.
    NOP(i64),
    // ACC increase the accumulator
    // by a specified amount.
    ACC(i64),
    // JMP progress the instructions
    // to a position relative to the
    // current position by the amount
    // specified,
    JMP(i64),
    // Exit is the final operation
    // which signals the program that
    // the instructions have finished.
    Exit(),
}

// parse_instructions takes an input file and creates
// a map of each operation associated with its position.
pub fn parse_instructions(
    lines: &Vec<String>,
) -> Result<HashMap<usize, Op>, Box<dyn std::error::Error>> {
    let mut instructions: HashMap<usize, Op> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        // Ignore blank lines
        if line == "" {
            continue;
        }

        // Split the Operations into Name, Value
        // An example Operation could be `jmp +10`
        let args: Vec<String> = line.split_whitespace().map(String::from).collect();
        if args.len() != 2 {
            return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                "Invalid Operation",
            )));
        }
        // Create a Op from the input line
        match args[0].as_str() {
            "nop" => instructions.insert(i, Op::NOP(args[1].parse::<i64>()?)),
            "acc" => instructions.insert(i, Op::ACC(args[1].parse::<i64>()?)),
            "jmp" => instructions.insert(i, Op::JMP(args[1].parse::<i64>()?)),
            _ => continue,
        };
    }

    // Add the last command
    instructions.insert(instructions.keys().len(), Op::Exit());

    Ok(instructions)
}

// run_instructions takes a map of instructions and performs
// each operation starting from 0. This function will return
// a tuple with the final value of the accumulator and a bool
// which is true if the instructions completed successfully, or
// false if the instructions did not finish (stuck in a loop).
pub fn run_instructions(
    instructions: &HashMap<usize, Op>,
) -> Result<(i64, bool), Box<dyn std::error::Error>> {
    let instructions_size = instructions.keys().len().try_into()?;

    // This map is used to keep track of the instructions
    // have executed, if an instructions is ran more
    // than once it shows that it is stuck in a loop.
    let mut executed_instructions: HashMap<usize, bool> = HashMap::new();
    // Init the necessary variables needed to operate
    // the instructions.
    let mut current_index: usize = 0;
    let mut accumulator: i64 = 0;

    // Loop through the instructions.
    while current_index <= instructions_size {
        // Check if a Loop has occurred if so
        // exit the loop and notify the caller
        // that the instructions have looped.
        if executed_instructions.contains_key(&current_index) {
            break;
        }

        // Retrieve the current instruction and execute
        // its operation.
        match instructions.get(&current_index) {
            Some(instruction) => {
                // Note down that the current operation will
                // now be executed
                executed_instructions.insert(current_index, true);
                // Perform the Operation's function
                match instruction {
                    Op::NOP(_) => {
                        // Do Nothing and progress
                        current_index += 1;
                    }
                    Op::ACC(amount) => {
                        // Add the amount specified to the
                        // accumulator then progress
                        accumulator += amount;
                        current_index += 1;
                    }
                    Op::JMP(amount) => {
                        // Progress to the command relative by
                        // the amount specified from the current
                        // command.
                        let x: i64 = i64::try_from(current_index)? + amount;
                        current_index = x.try_into()?;
                    }
                    Op::Exit() => return Ok((accumulator, true)),
                }
            }
            None => {
                return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                    "Invalid Operation",
                )))
            }
        }
    }

    // Return the accumulator value and a false
    // value to represent that the instructions
    // did not complete successfully.
    Ok((accumulator, false))
}
