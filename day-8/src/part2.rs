extern crate utils;
use super::shared;
use std::collections::HashMap;

// solve attempts to solve part 2 of day 8 of AoC 2020.
// Modify the input instruction set by swapping the JMP
// and NOP operations to find a set of instructions which
// will complete successfully.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut instructions: HashMap<usize, shared::Op> = shared::parse_instructions(lines)?;

    // Loop through each instruction to see if it can be
    // swapped, then run the modified instruction set to
    // see if it runs successfully.
    let instructions_size: usize = instructions.keys().len();
    for i in 0..instructions_size {
        // Swap if JMP or NOP
        match instructions.get_mut(&i) {
            Some(instruction) => match instruction {
                shared::Op::JMP(amount) => *instruction = shared::Op::NOP(*amount),
                shared::Op::NOP(amount) => *instruction = shared::Op::JMP(*amount),
                _ => (),
            },
            None => {
                return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                    "Missing Instruction",
                )))
            }
        }
        // Run the instructions
        let result: (i64, bool) = shared::run_instructions(&instructions)?;
        // If true we have found the correct
        if result.1 {
            return Ok(result.0);
        }
        // Swap instruction back
        match instructions.get_mut(&i) {
            Some(instruction) => match instruction {
                shared::Op::JMP(amount) => *instruction = shared::Op::NOP(*amount),
                shared::Op::NOP(amount) => *instruction = shared::Op::JMP(*amount),
                _ => (),
            },
            None => {
                return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                    "Missing Instruction",
                )))
            }
        }
    }
    return Err(Box::new(utils::errors::NoResultError::new(
        "No Valid Instructions",
    )));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 8);
        Ok(())
    }
}
