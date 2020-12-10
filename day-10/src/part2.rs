extern crate utils;
use std::collections::HashMap;

// solve attempts to solve part 2 of day 10 of AoC 2020.
// Calculate the number of valid chains of adapters which
// can convert from 0 to the highest jolt available from
// the list of adapters.
pub fn solve(lines: &Vec<String>) -> Result<i64, Box<dyn std::error::Error>> {
    let mut adapters: Vec<i64> = lines.iter().flat_map(|x| x.parse::<i64>()).collect();
    let adapters_size: usize = adapters.len();
    adapters.sort();

    // Calculate the number of possible chains from 0
    // to the current adapter.
    let mut chains: HashMap<i64, i64> = HashMap::new();
    chains.insert(0, 1);
    for (i, adapter) in adapters.iter().enumerate() {
        // The number of chains from 0  is the sum of the number of
        // chains from any adapter 3 jolts before.
        let mut current_chains: i64 = 0;
        for j in 1..4 {
            match chains.get(&(adapter - j)) {
                Some(x) => current_chains += x,
                None => (),
            };
        }

        // If this is the last adapter return the
        // number of chains as this is the total
        // number of valid chains from 0.
        if i == adapters_size - 1 {
            return Ok(current_chains);
        }

        chains.insert(*adapter, current_chains);
    }

    Err(Box::new(utils::errors::NoResultError::new(
        "Could not calculate permutations",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 19208);
        Ok(())
    }
}
