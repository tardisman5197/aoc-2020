extern crate utils;
use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    color: String,
    contains: HashMap<String, (i64, *const Bag)>,
    inside: HashMap<String, *const Bag>,
    holds_shiny: bool,
}

impl Bag {
    fn new(color: String) -> Bag {
        return Bag {
            color: color,
            contains: HashMap::new(),
            inside: HashMap::new(),
            holds_shiny: false,
        };
    }

    fn add_bag(&mut self, bag: &mut Bag, amount: i64) {
        if bag.holds_shiny {
            self.holds_shiny = true
        }
        self.contains.insert(bag.color.to_owned(), (amount, bag));
    }

    fn add_inside(&mut self, bag: &mut Bag) {
        self.inside.insert(bag.color.to_owned(), bag);
    }
}

fn create_bag_tree(
    lines: &Vec<String>,
) -> Result<HashMap<String, &mut Bag>, Box<dyn std::error::Error>> {
    let bags: HashMap<String, &mut Bag> = HashMap::new();

    for line in lines.iter() {
        let line_split: Vec<String> = line.split("contain").map(String::from).collect();
        if line_split.len() != 2 {
            return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                "Invalid Input Line",
            )));
        }
        // Create or fetch the bag mentioned on this line
        let color: String = line_split[0].replace("bags", "").trim().to_string();
        let bag: &mut Bag = match bags.get_mut(&color) {
            Some(bag) => bag,
            None => &mut Bag::new(color),
        };

        // Create or add the bags inside
        let bags_inside: Vec<String> = line_split[1].split(",").map(String::from).collect();
        for bag_internal in bags_inside.iter() {
            let bag_internal_split: Vec<String> =
                bag_internal.split_whitespace().map(String::from).collect();
            if bag_internal_split.len() < 3 {
                return Err(Box::new(utils::errors::SomethingIsWrongError::new(
                    "Invalid Input Line",
                )));
            }
            let bag_internal_color: String =
                format!("{} {}", bag_internal_split[1], bag_internal_split[2]);
            let bag_internal_amount: i64 = bag_internal_split[0].parse::<i64>()?;
            match bags.get_mut(&bag_internal_color) {
                Some(bag_internal) => {
                    bag_internal.add_inside(bag);
                    bag.add_bag(bag_internal, bag_internal_amount)
                }
                None => {
                    let bag_internal = &mut Bag::new(bag_internal_color);
                    bag_internal.add_inside(bag);
                    bag.add_bag(bag_internal, bag_internal_amount);
                    bags.insert(bag_internal_color, bag_internal);
                }
            }
        }
    }

    Ok(bags)
}

// solve attempts to solve part 1 of day 7 of AoC 2020.
pub fn solve(lines: &Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let bags: HashMap<String, &mut Bag> = create_bag_tree(lines)?;
    println!("{:?}", bags);

    Err(Box::new(utils::errors::SomethingIsWrongError::new(
        "Not Implemented",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn std::error::Error>> {
        let file: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let input: Vec<String> = file.lines().map(String::from).collect();

        assert_eq!(solve(&input)?, 4);
        Ok(())
    }
}
