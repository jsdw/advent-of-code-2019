use crate::error::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

const TRILLION: u128 = 1_000_000_000_000;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let reactions = parse_reactions(input);
    let recipes = make_recipes(&reactions);

    println!("Star 1: {}", ore_needed_for_fuel(&recipes, 1));
    println!("Star 2: {}", search_for_trillion(&recipes));

    Ok(())
}

/// Binary chop different fuel targets to find out how much fuel
/// we can make with a trillion ore:
fn search_for_trillion(recipes: &Recipes) -> u128 {
    let mut best_higher = TRILLION;
    let mut best_lower = std::u128::MIN;
    loop {
        let current = (best_higher - best_lower) / 2 + best_lower;
        let lower_ore = ore_needed_for_fuel(recipes, current);
        let higher_ore = ore_needed_for_fuel(recipes, current+1);
        if lower_ore > TRILLION {
            best_higher = current;
        } else if higher_ore < TRILLION {
            best_lower = current;
        } else {
            return current
        }
    }
}

/// Store reactions in the form of recipes, linking output to quantity
/// made and the inputs required for it (borrows from reactions):
fn make_recipes(reactions: &[Reaction]) -> Recipes {
    reactions
        .into_iter()
        .map(|r| (&*r.output.name, (r.output.amount, &*r.inputs)))
        .collect()
}

/// Find the greatest distance away from FUEL that each chemical is, so
/// that we know what order to work through to figure out how much of each
/// thing earlier on that we need.
fn increment_distance_from<'a> (
    chemical: &str,
    distance: usize,
    recipes: &'a Recipes,
    distance_from_fuel: &mut HashMap<&'a str,usize>
) {
    let (_, inputs) = recipes[chemical];
    for input in inputs {
        if input.name == "ORE" { continue }
        let entry = distance_from_fuel.entry(&input.name).or_insert(0);
        if *entry < distance + 1 {
            *entry = distance + 1;
            increment_distance_from(&input.name, distance + 1, recipes, distance_from_fuel);
        }
    }
}

/// How much ore do we need to make the amount of fuel asked for.
fn ore_needed_for_fuel(recipes: &Recipes, fuel_needed: u128) -> u128 {
    // Step 1: work out the distance from FUEL that each chemical is.
    let mut distance_from_fuel: HashMap<&str,usize> = HashMap::new();
    distance_from_fuel.insert("FUEL", 0);
    increment_distance_from("FUEL", 0, &recipes, &mut distance_from_fuel);

    // Step 2: based on the above, order from closest to furthest from FUEL:
    let mut ordering: Vec<(&str,usize)> = distance_from_fuel.into_iter().collect();
    ordering.sort_by_key(|&(_,n)| n);

    // Step 3: Go over this order and tally up what's needed. Because of our order
    // we know that by the time we get to an item, we have the full quantity we'll
    // need of it from later reactions.
    let mut ore_needed = 0;
    let mut needed: HashMap<&str,u128> = HashMap::new();
    needed.insert("FUEL", fuel_needed);
    for (item, _) in ordering {
        let (output_quantity, inputs) = recipes[item];
        let required_amount = needed[item];
        let required_multiple = if required_amount % output_quantity == 0 {
            required_amount / output_quantity
        } else {
            required_amount / output_quantity + 1
        };
        for input in inputs {
            if input.name == "ORE" {
                ore_needed += input.amount * required_multiple;
            } else {
                *needed.entry(&*input.name).or_insert(0) += input.amount * required_multiple;
            }
        }
    }
    ore_needed
}

fn parse_reactions(input: &str) -> Vec<Reaction> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([0-9]+) ([A-Z]+)").unwrap());
    let mut reactions: Vec<Reaction> = vec![];
    for line in input.trim().lines() {
        let mut chemicals: Vec<Chemical> = vec![];
        for cap in RE.captures_iter(line) {
            chemicals.push(Chemical {
                amount: cap[1].parse().unwrap(),
                name: cap[2].to_owned()
            });
        }
        let last = chemicals.pop().unwrap();
        reactions.push(Reaction {
            inputs: chemicals,
            output: last
        });
    }
    reactions
}

type Recipes<'a> = HashMap<&'a str,(u128, &'a [Chemical])>;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Chemical {
    amount: u128,
    name: String
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical
}

#[cfg(test)]
mod test {

    use super::*;

    static SAMPLE1: &str = "
        1 ORE => 2 A
        1 A => 1 B
        1 A, 1 B => 1 FUEL
    ";
    static SAMPLE2: &str = "
        9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL
    ";
    static SAMPLE3: &str = "
        157 ORE => 5 NZVS
        165 ORE => 6 DCFZ
        44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
        12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
        179 ORE => 7 PSHF
        177 ORE => 5 HKGWZ
        7 DCFZ, 7 PSHF => 2 XJWVT
        165 ORE => 2 GPVTF
        3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
    ";
    static SAMPLE4: &str = "
        2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
        17 NVRVD, 3 JNWZP => 8 VPVL
        53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
        22 VJHF, 37 MNCFX => 5 FWMGM
        139 ORE => 4 NVRVD
        144 ORE => 7 JNWZP
        5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
        5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
        145 ORE => 6 MNCFX
        1 NVRVD => 8 CXFTF
        1 VJHF, 6 MNCFX => 4 RFSQX
        176 ORE => 6 VJHF
    ";
    static SAMPLE5: &str = "
        171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX
    ";

    #[test]
    fn test_ore_needed() {
        let tests = vec![
            (SAMPLE1, 1),
            (SAMPLE2, 165),
            (SAMPLE3, 13312),
            (SAMPLE4, 180697),
            (SAMPLE5, 2210736)
        ];
        for (s, expected) in tests {
            let actual = ore_needed_for_fuel(&make_recipes(&parse_reactions(s)), 1);
            assert_eq!(actual, expected, "Needs {} ore but algo says {}", expected, actual);
        }
    }

    #[test]
    fn test_fuel_obtained_for_trillion_ore() {
        let tests = vec![
            (SAMPLE3, 82892753),
            (SAMPLE4, 5586022),
            (SAMPLE5, 460664)
        ];
        for (s, ore_needed) in tests {
            let lower = ore_needed_for_fuel(&make_recipes(&parse_reactions(s)), ore_needed);
            let upper = ore_needed_for_fuel(&make_recipes(&parse_reactions(s)), ore_needed+1);
            let success = lower <= TRILLION && upper > TRILLION;
            assert!(success, "Expected {} - {} to surround 1 trillion", lower, upper);
        }
    }

}