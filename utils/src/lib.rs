pub mod errors;

use std::fs;

// read_file_lines takes in a path to a file and returns
// a Vector of Strings with each element being a line
// in the file.
pub fn read_file_lines(filename: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // read_to_string can return an error if
    // the file could not be read.
    let contents = fs::read_to_string(filename)?;
    // Create a Vector from the contents of the
    // file.
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    
    Ok(lines)
}
