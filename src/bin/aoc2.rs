// https://adventofcode.com/2019/day/2

mod intcode;

fn run_lunarprog(noun: i128, verb: i128) -> i128 {
    let mut v: Vec<i128> =
      vec![1,noun,verb,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,13,19,23,2,23,9,27,1,6,27,
                     31,2,10,31,35,1,6,35,39,2,9,39,43,1,5,43,47,2,47,13,51,2,51,10,55,1,
                     55,5,59,1,59,9,63,1,63,9,67,2,6,67,71,1,5,71,75,1,75,6,79,1,6,79,83,
                     1,83,9,87,2,87,10,91,2,91,10,95,1,95,5,99,1,99,13,103,2,103,9,107,1,
                     6,107,111,1,111,5,115,1,115,2,119,1,5,119,0,99,2,0,14,0];
    intcode::run(&mut v);
    //println!("NounVerb: {} Result: {:?}", 100*noun+verb, v[0]);
    v[0]
}

fn find_lunarprog_nounverb(expected_output: i128) -> Option<i128> {
    for noun in 1..101 {
        for verb in 1..102 {
            let r = run_lunarprog(noun, verb);
            if r == expected_output {
                return Some(noun*100 + verb);
            }
        }
    }
    return None
}

fn main() {
    println!("Phase1 result: {:?}", run_lunarprog(12, 2));
    println!("Phase2 nounverb: {:?}", find_lunarprog_nounverb(19690720));
}
