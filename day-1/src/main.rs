// Declare the modules in the package
mod part1;

extern crate utils;

// main runs each part of the day's challenges
fn main() -> Result<(),  Box<dyn std::error::Error>> {
    // Read in the input
    let lines: Vec<String> = utils::read_file_lines("./inputs/day1.txt".to_string())?;

    // Run and print the result
    println!("Day 1:\n\tPart 1: {}\n",part1::solve(lines)?);

    Ok(())
}


