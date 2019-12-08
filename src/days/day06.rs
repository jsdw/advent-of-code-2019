use crate::error::Error;
use std::collections::HashMap;

/// Part 1 and 2 combined
pub fn both_parts(input: &str) -> Result<(),Error> {
    let orbits = parse_input(input)?;

    // What is orbiting what?
    let mut object_orbits: HashMap<String,String> = HashMap::new();
    for (a,b) in orbits {
        object_orbits.insert(b, a);
    }

    // Count up the orbits
    println!("Star 1: {}", get_orbit_counts(&object_orbits));

    // Get he distance to santa.
    let distance_to_santa = get_distance_to_santa(&object_orbits);
    println!("Star 2: {}", distance_to_santa);

    Ok(())
}

/// How many orbits do you have to hop to get to santa? This
/// assumes that everything ultiamtely orbits COM.
fn get_distance_to_santa(object_orbits: &HashMap<String,String>) -> usize {

    // Step 1: How far away are objects that Santa is orbiting?
    let mut distances_from_santa = HashMap::new();
    {
        let mut current_dist: usize = 0;
        for new_o in iterate_orbits("SAN", object_orbits) {
            distances_from_santa.insert(new_o, current_dist);
            current_dist += 1;
        }
    }

    // Step 2: How far away are objects that You is orbiting?
    let mut distances_from_you = HashMap::new();
    {
        let mut current_dist: usize = 0;
        for new_o in iterate_orbits("YOU", object_orbits) {
            distances_from_you.insert(new_o, current_dist);
            current_dist += 1;
        }
    }

    // Step 3: Find the object common to both of the above and add
    // the scores together to get the total hops.
    let mut min_hops = std::usize::MAX;
    for (object, distance_a) in distances_from_you {
        if let Some(distance_b) = distances_from_santa.get(object) {
            let total = distance_a + distance_b;
            min_hops = min_hops.min(total);
        }
    }

    min_hops
}

/// Naively count all orbits from every object to get a total
fn get_orbit_counts(object_orbits: &HashMap<String,String>) -> usize {
    let mut count = 0;
    for (object, orbiting) in object_orbits {
        count += iterate_orbits(object, object_orbits).count()
    }
    count
}

/// Given a starting object and a graph, iterate over the things orbited by the starting object
fn iterate_orbits<'a>(start: &'a str, object_orbits: &'a HashMap<String,String>) -> impl Iterator<Item=&'a str> + 'a {
    let mut current_o = start;
    std::iter::from_fn(move || {
        if let Some(new_o) = object_orbits.get(current_o) {
            current_o = &**new_o;
            Some(current_o)
        } else {
            None
        }
    })
}

fn parse_input(input: &str) -> Result<Vec<(String,String)>,Error> {
    let mut results = vec![];
    for (idx,line) in input.trim().lines().enumerate() {
        let pos = line.find(')').ok_or_else(|| err!("Cannot parse line {}", idx+1))?;
        results.push((line[0..pos].trim().to_owned(), line[pos+1..].trim().to_owned()));
    }
    Ok(results)
}