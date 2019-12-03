fn step(pc: usize, ram: &mut Vec<u32>) -> Option<usize> {
    let op = ram[pc];
    if op == 99 {
        return None;
    } else {
        let i1loc: usize = ram[pc+1] as usize;
        let i2loc: usize = ram[pc+2] as usize;
        let tloc: usize = ram[pc+3] as usize;
        let i1 = ram[i1loc];
        let i2 = ram[i2loc];
        let r = if op == 1 {
            i1 + i2
        } else if op == 2 {
            i1 * i2
        } else {
            assert!(false);
            return None;
        };
        //println!("pc={} op={}(@{}={}, @{}={}) = {} -> @{}",
        //          pc, op, i1loc, i1, i2loc, i2, r, tloc);
        ram[tloc] = r;
    }

    Some(pc + 4)
}

fn run(ram: &mut Vec<u32>) {
    println!("Hello, world! ram={:?}", ram);
    let mut pc: usize = 0;
    loop {
        match step(pc, ram) {
            Some(newpc) => pc = newpc,
            None => break
        }
    }
    println!("Goodbye, world! ram={:?}", ram);    
}

fn run_lunarprog(noun: u32, verb: u32) -> u32 {
    let mut v = vec![1,noun,verb,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,13,19,23,2,23,9,27,1,6,27,
                     31,2,10,31,35,1,6,35,39,2,9,39,43,1,5,43,47,2,47,13,51,2,51,10,55,1,
                     55,5,59,1,59,9,63,1,63,9,67,2,6,67,71,1,5,71,75,1,75,6,79,1,6,79,83,
                     1,83,9,87,2,87,10,91,2,91,10,95,1,95,5,99,1,99,13,103,2,103,9,107,1,
                     6,107,111,1,111,5,115,1,115,2,119,1,5,119,0,99,2,0,14,0];
    run(&mut v);
    //println!("NounVerb: {} Result: {:?}", 100*noun+verb, v[0]);
    v[0]
}

fn find_lunarprog_nounverb(expected_output: u32) -> Option<u32> {
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
    // Tests (part1):
    //run(&mut vec![1,0,0,0,99]);
    //run(&mut vec![2,3,0,3,99]);
    //run(&mut vec![2,4,4,5,99,0]);
    //run(&mut vec![1,1,1,4,99,5,6,0,99]);
    // original without noun & verb
    //run(&mut vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,13,19,23,2,23,9,27,1,6,27,
    //              31,2,10,31,35,1,6,35,39,2,9,39,43,1,5,43,47,2,47,13,51,2,51,10,55,1,
    //              55,5,59,1,59,9,63,1,63,9,67,2,6,67,71,1,5,71,75,1,75,6,79,1,6,79,83,
    //              1,83,9,87,2,87,10,91,2,91,10,95,1,95,5,99,1,99,13,103,2,103,9,107,1,
    //              6,107,111,1,111,5,115,1,115,2,119,1,5,119,0,99,2,0,14,0]);
    // phase1: nount 12, verb 2
    //run(&mut vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,13,19,23,2,23,9,27,1,6,27,
    //              31,2,10,31,35,1,6,35,39,2,9,39,43,1,5,43,47,2,47,13,51,2,51,10,55,1,
    //              55,5,59,1,59,9,63,1,63,9,67,2,6,67,71,1,5,71,75,1,75,6,79,1,6,79,83,
    //              1,83,9,87,2,87,10,91,2,91,10,95,1,95,5,99,1,99,13,103,2,103,9,107,1,
    //              6,107,111,1,111,5,115,1,115,2,119,1,5,119,0,99,2,0,14,0]);
    // phase1 alternative:
    run_lunarprog(12, 2);

    // phase2
    let nounverb = find_lunarprog_nounverb(19690720);
    println!("{:?}", nounverb);
}
