// Declare the modules in the package
mod part1;
mod part2;

extern crate utils;

use std::time::Instant;

// main runs each part of the day's challenges
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read in the input
    let lines: Vec<String> = utils::read_file_lines("./inputs/day10.txt")?;

    // Run the solutions
    let start = Instant::now();
    let part1_solution = part1::solve(&lines)?;
    let part1_duration = start.elapsed();

    let start = Instant::now();
    let part2_solution = part2::solve(&lines)?;
    let part2_duration = start.elapsed();

    println!("Day 10:");
    println!("\tPart 1: {} ({:?})", part1_solution, part1_duration);
    println!("\tPart 2: {} ({:?})", part2_solution, part2_duration);

    Ok(())
}
