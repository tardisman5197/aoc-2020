extern crate utils;

// Entry is all the information that
// can be retrieved from an input line.
struct Entry {
    indexes: Vec<usize>,
    character: char,
    password: String
}

// parse_line takes a line from Day 2's input and attempts
// to parse the line into an Entry struct.
fn parse_line(line: String) -> Result<Entry, Box<dyn std::error::Error>> {
    // Split the line by white space,
    // this should result in:
    //  lower_limit-upper_limit
    //  character:
    //  password
    let fields: Vec<String> = line.split_whitespace().map(String::from).collect();
    if fields.len() != 3 {
        return Err(Box::new(utils::errors::SomethingIsWrongError::new("Invalid Entry")))
    };

    // Parse the lower and upper limits
    let limits: Vec<usize> = fields[0].split("-").flat_map(|limit| limit.parse::<usize>()).collect();
    if limits.len() != 2 {
        return Err(Box::new(utils::errors::SomethingIsWrongError::new("Invalid Entry, indexes are wrong")))
    };

    // Get the character.
    let character: char = match fields[1].chars().nth(0) {
        Some(characters) => characters,
        None => return Err(Box::new(utils::errors::SomethingIsWrongError::new("Invalid Entry, could not get the character"))),
    };

    // Create the Entry to return
    Ok(Entry{
        indexes: limits,
        character: character,
        password: fields[2].clone(),
    })
}

// solve attempts to solve Day 2 Part 2 of AoC.
// Parse each line of the input, then check that
// at only one of the indexes provided the password
// matches the specified character. The result of
// this function is the number of valid passwords.
pub fn solve(lines: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut valid_password_count: i32 = 0;

    for line in lines.iter() {
        let entry: Entry = parse_line(line.to_string())?;

        let mut matched: usize = 0;

        for index in entry.indexes.iter() {
            // Get char at the current index,
            // continue to the next index if the index
            // is in valid.
            // The problem also states that the indexes
            // start at 1, not 0, so -1 to the index.
            let current_char: char = match entry.password.chars().nth(*index-1) {
                Some(current_char) => current_char,
                None => continue,
            };
            
            if current_char == entry.character {
                matched = matched + 1;
            };            
        };

        // Only one index can match not both
        if matched == 1 {
            valid_password_count= valid_password_count+1;
        }
    };

    Ok(valid_password_count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<String> = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];
        
        assert_eq!(solve(&input)?, 1);
        Ok(())
    }
}