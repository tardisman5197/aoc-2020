extern crate utils;
use std::collections::HashMap;

// get_common_answers takes an iterator over the values stored
// in a hashmap containing the frequencies of every answer as
// well as the size of the group relating to the answers. These
// are then used to find the number of answers which everyone in
// the group answered in the affirmative.
fn get_number_of_common_answers(
    frequencies: std::collections::hash_map::Values<'_, char, i64>,
    group_size: &i64,
) -> usize {
    let mut number_of_answers: usize = 0;

    for frequency in frequencies {
        if frequency == group_size {
            number_of_answers += 1
        }
    }
    number_of_answers
}

// solve attempts to solve part 2 of day 6 of AoC 2020.
// Calculate the sum of the common answers for each group.
pub fn solve(lines: &Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut result: usize = 0;
    let mut declaration_forms_answers: HashMap<char, i64> = HashMap::new();
    let mut group_size: i64 = 0;

    for line in lines.iter() {
        // Check if there is a new set of
        // declaration forms.
        if line == "" {
            result += get_number_of_common_answers(declaration_forms_answers.values(), &group_size);

            // Reset all of the group lated variables
            declaration_forms_answers = HashMap::new();
            group_size = 0;

            continue;
        }

        // Log every occurrence of an answer for that
        // group
        for answer in line.chars() {
            if declaration_forms_answers.contains_key(&answer) {
                declaration_forms_answers
                    .insert(answer, declaration_forms_answers.get(&answer).unwrap() + 1);
            } else {
                declaration_forms_answers.insert(answer, 1);
            }
        }

        group_size += 1;
    }

    // Add the last set of forms
    result += get_number_of_common_answers(declaration_forms_answers.values(), &group_size);

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

        assert_eq!(solve(&input)?, 6);
        Ok(())
    }
}
