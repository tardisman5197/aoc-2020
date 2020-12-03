// Declare the modules in the package
mod part1;
mod part2;

extern crate utils;

// main runs each part of the day's challenges
fn main() -> Result<(),  Box<dyn std::error::Error>> {
    // Read in the input
    let lines: Vec<String> = utils::read_file_lines("./inputs/day2.txt".to_string())?;

    // Run and print the result
    println!("Day 1:");
    println!("\tPart 1: {}", part1::solve(&lines)?);
    println!("\tPart 2: {}", part2::solve(&lines)?);

    Ok(())
}
