extern crate utils;

// solve attempts to solve part 1 of day 3 of AoC 2020.
// For each line check how far right we are based
// on the problem specification. Then use modulo maths
// to check what the character is at that right value.
// If the character found is a '#' (A Tree), then count
// it and the result of the function is the number of
// trees on the given path.
pub fn solve(lines: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut tree_count = 0;

    for (i, line) in lines.iter().enumerate() {
        // Calculate current position in line:
        //  (line_number*delta_right)%line_length
        let current_position = match line.chars().nth((i*3)%line.len()) {
            Some(current_position) => current_position,
            None => return Err(Box::new(utils::errors::SomethingIsWrongError::new("Could not get position"))),
        };

        // Check if it is a tree, if so count it
        if current_position == '#' {
            tree_count = tree_count + 1;
        };
    };

    Ok(tree_count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<String> = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];
        
        assert_eq!(solve(&input)?, 7);
        Ok(())
    }
}