extern crate utils;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
enum Op {
    NOP(i64),
    ACC(i64),
    JMP(i64),
}

fn parse_instructions(
    lines: &Vec<String>,
) -> Result<HashMap<usize, Op>, Box<dyn std::error::Error>> {
    let mut instructions: HashMap<usize, Op> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if line == "" {
            continue;
        }

        let args: Vec<String> = line.split_whitespace().map(String::from).collect();
        if args.len() != 2 {
            return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                "Invalid Operation",
            )));
        }
        match args[0].as_str() {
            "nop" => instructions.insert(i, Op::NOP(args[1].parse::<i64>()?)),
            "acc" => instructions.insert(i, Op::ACC(args[1].parse::<i64>()?)),
            "jmp" => instructions.insert(i, Op::JMP(args[1].parse::<i64>()?)),
            _ => continue,
        };
    }

    Ok(instructions)
}

// solve attempts to solve part 1 of day 8 of AoC 2020.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    let instructions: HashMap<usize, Op> = parse_instructions(lines)?;

    let mut executed_instructions: HashMap<usize, bool> = HashMap::new();
    let mut current_index: usize = 0;
    let mut accumulator: i64 = 0;

    let instructions_size = instructions.keys().len().try_into()?;
    while current_index <= instructions_size {
        // Check if a Loop has occurred
        if executed_instructions.contains_key(&current_index) {
            break;
        }

        match instructions.get(&current_index) {
            Some(instruction) => {
                executed_instructions.insert(current_index, true);
                match instruction {
                    Op::NOP(_) => {
                        current_index += 1;
                    }
                    Op::ACC(amount) => {
                        accumulator += amount;
                        current_index += 1;
                    }
                    Op::JMP(amount) => {
                        let x: i64 = i64::try_from(current_index)? + amount;
                        current_index = x.try_into()?;
                    }
                }
            }
            None => {
                return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                    "Invalid Operation",
                )))
            }
        }
    }

    Ok(accumulator)
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

        assert_eq!(solve(&input)?, 5);
        Ok(())
    }
}
