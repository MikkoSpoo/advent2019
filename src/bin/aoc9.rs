// https://adventofcode.com/2019/day/9

mod intcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Phase 1: {:?}",
             intcode::run_inpi(&mut(intcode::mem_from_file("input_data/aoc9.txt")?), 1));
    println!("Phase 2: {:?}",
             intcode::run_inpi(&mut(intcode::mem_from_file("input_data/aoc9.txt")?), 2));
    Ok(())
}

