extern crate utils;
use std::collections::HashSet;

// solve attempts to solve part 1 of day 6 of AoC 2020.
// Calculate the sum of the unique values for each set
// of declaration forms.
pub fn solve(lines: &Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut result: usize = 0;
    let mut declaration_forms_answers: HashSet<char> = HashSet::new();

    for line in lines.iter() {
        // Check if there is a new set of
        // declaration forms.
        if line == "" {
            result += declaration_forms_answers.len();
            declaration_forms_answers = HashSet::new();
            continue;
        }

        // Add all of the unique answers to the
        // declaration forms answers.
        for answer in line.chars() {
            if !declaration_forms_answers.contains(&answer) {
                declaration_forms_answers.insert(answer);
            }
        }
    }

    // Add the last set of forms
    result += declaration_forms_answers.len();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 11);
        Ok(())
    }
}
