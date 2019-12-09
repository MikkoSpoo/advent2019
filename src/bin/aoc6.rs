// https://adventofcode.com/2019/day/6

use std::collections::HashMap;
use std::collections::HashSet;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

struct Obj {
    orbits: Option<String>,
    on_orbit: HashSet<String>,
}

impl Obj {
    fn new() -> Obj {
        Obj {
            orbits: None,
            on_orbit: HashSet::<String>::new()
        }
    }

    fn set_on_orbit_of(&mut self, name :&str) {
        match &self.orbits {
            Some(o) => assert_eq!(o, name),
            None => self.orbits = Some(name.to_string()),
        }
    }

    fn add_orbiting(&mut self, name: &str) {
        self.on_orbit.insert(name.to_string()); // returns bool weather already exists
    }
}

struct Orbitmap {
    hm :HashMap::<String, Obj>,
}

impl Orbitmap {
    fn new() -> Orbitmap {
        Orbitmap { hm: HashMap::<String, Obj>::new()}
    }

    fn new_from_file(filename :&str) -> Result<Orbitmap, io::Error> {
        let mut boomer = Orbitmap::new();
        boomer.read_from_file(filename)?;
        Ok(boomer)
    }

    fn read_from_file(&mut self, filename :&str) -> std::io::Result<()> {
        let f = File::open(filename)?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let l = line.unwrap();
            if !l.is_empty() {
              let mut spl = l.split(')');
              let n1 = spl.next().unwrap(); // TODO check
              let n2 = spl.next().unwrap(); // TODO check
              self.add_orbit(n1, n2);
          }
        }
        Ok(())
    }

    fn add_orbit(&mut self, name1 :&str, name2:&str) {
        {
            let o1 = self.hm.entry(name1.to_string()).or_insert_with(Obj::new);
            o1.add_orbiting(name2);
        }
        {
            let o2 = self.hm.entry(name2.to_string()).or_insert_with(Obj::new);
            o2.set_on_orbit_of(name1);
        }
    }

    fn get_parent_of(&self, name :&str) -> Option<String> {
        match self.hm.get(name) {
            None => None,
            Some(obj) => match &obj.orbits {
                None => None,
                Some(s) => Some(String::from(s)),
            }
        }
    }

    fn count_depth(&self, name :&str) -> u32 {
        match &self.get_parent_of(name) {
            None => 0,
            Some(parent) => 1 + self.count_depth(&parent)
        }
    }

    fn sum_depths(&self) -> u32 {
        let mut sum = 0;
        for (name, _) in self.hm.iter() {
            sum += self.count_depth(name);
        }
        sum
    }
}

#[test]
fn t_6_1_example() {
    assert_eq!(Orbitmap::new_from_file("test_data/aoc_6_1_example.txt").unwrap().sum_depths(), 42);
}

fn main() -> std::io::Result<()> {
    println!("{:?}", Orbitmap::new_from_file("input_data/aoc6.txt")?.sum_depths());
    Ok(())
}
