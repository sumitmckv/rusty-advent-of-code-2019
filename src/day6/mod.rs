use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
struct Orbit {
    orbiter: Vec<String>,
    orbitee: String,
}

impl Orbit {
    fn new() -> Orbit {
        Orbit {
            orbiter: Vec::new(),
            orbitee: String::new(),
        }
    }

    fn add_orbiter(&mut self, orbiter: String) {
        self.orbiter.push(orbiter);
    }

    fn add_orbitee(&mut self, orbitee: String) {
        self.orbitee = orbitee;
    }
}

/*
part1:
COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L
part2:
COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L K)YOU I)SAN
*/
pub fn day6() {
    let orbit_map = build_orbit_map();
    let checksum = get_checksum(&String::from("COM"), &orbit_map, 0, 0);
    println!("Result of day6a is {}", checksum);
    let path_you = get_path(
        &String::from("YOU"),
        &String::from("COM"),
        &orbit_map,
        HashSet::new(),
    );
    let path_santa = get_path(
        &String::from("SAN"),
        &String::from("COM"),
        &orbit_map,
        HashSet::new(),
    );
    let intersections: HashSet<String> = path_you
        .intersection(&path_santa)
        .map(|s| String::from(s))
        .collect();
    let path_total: HashSet<String> = path_you
        .union(&path_santa)
        .map(|s| String::from(s))
        .collect();
    println!(
        "Result of day6b is {:?}",
        path_total.len() - intersections.len() - 2
    );
}

fn parse_input() -> std::io::Result<Vec<(String, String)>> {
    let input_file = PathBuf::from("./src/day6/input.txt").canonicalize()?;
    let file = File::open(input_file)?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut orbits: Vec<(String, String)> = Vec::new();
    for line in lines {
        let relation = line.expect("Failed to read line");
        let relation_vec: Vec<String> =
            relation.split(")").map(|s| s.to_string()).take(2).collect();
        let parent = String::from(&relation_vec[0]);
        let child = String::from(&relation_vec[1]);
        orbits.push((parent, child));
    }
    Ok(orbits)
}

fn build_orbit_map() -> HashMap<String, Orbit> {
    let orbits = parse_input().expect("Failed to parse input");
    let mut orbit_map = HashMap::new();
    for orbit in &orbits {
        let (parent, child) = orbit;
        if !orbit_map.contains_key(parent) {
            orbit_map.insert(String::from(parent), Orbit::new());
        }

        if !orbit_map.contains_key(child) {
            orbit_map.insert(String::from(child), Orbit::new());
        }

        orbit_map
            .get_mut(parent)
            .unwrap()
            .add_orbiter(String::from(child));
        orbit_map
            .get_mut(child)
            .unwrap()
            .add_orbitee(String::from(parent));
    }
    orbit_map
}

fn get_checksum(
    parent: &String,
    orbit_map: &HashMap<String, Orbit>,
    root: usize,
    mut orbits: usize,
) -> usize {
    if let Some(orbiter) = orbit_map.get(parent) {
        orbits += root;
        for child in &orbiter.orbiter {
            orbits = get_checksum(child, orbit_map, root + 1, orbits);
        }
    }
    orbits
}

fn get_path(
    source: &String,
    dest: &String,
    orbit_map: &HashMap<String, Orbit>,
    mut orbits: HashSet<String>,
) -> HashSet<String> {
    if let Some(orbiter) = orbit_map.get(source) {
        orbits.insert(String::from(source));
        let orbitee = &orbiter.orbitee;
        if dest != orbitee {
            orbits = get_path(orbitee, dest, orbit_map, orbits);
        }
    }
    orbits
}
