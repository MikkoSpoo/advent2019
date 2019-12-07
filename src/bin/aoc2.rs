// https://adventofcode.com/2019/day/2
// https://adventofcode.com/2019/day/5

fn getarglocs(pc: usize, numofargs: usize, ram: &mut Vec<i32>)
              -> Vec<usize>
{
    let mut r = vec![0;numofargs];
    let mut o = ram[pc] / 100;
    for i in 0..numofargs {
        let loc = pc + i + 1;
        r[i] = if o % 10 != 0 {
            // immediate
            loc
        } else {
            // normal: location
            ram[loc] as usize
        };
        o = o / 10;
    }
    r
}

#[test]
fn t_getarglocs() {

    assert_eq!(getarglocs(0, 3, &mut vec![1101,100,-1,4,0]),
               vec![1, 2, 4]);

}
fn step(pc: usize, ram: &mut Vec<i32>, input: i32)
        -> (Option<usize>, Option<i32>) {
    let op = ram[pc];
    let c = op % 100;
    if c == 99 {
        return (None, None);
    } else if c == 1 {
        let a = getarglocs(pc, 3, ram);
        ram[a[2]] = ram[a[0]] + ram[a[1]];
        return (Some(pc + 4), None);
    } else if c == 2 {
        let a = getarglocs(pc, 3, ram);
        ram[a[2]] = ram[a[0]] * ram[a[1]];
        return (Some(pc + 4), None);
    } else if c == 3 {
        let a = getarglocs(pc, 1, ram);
        println!("INPUT: {} to loc {}", input, a[0]);
        ram[a[0]] = input;
        return (Some(pc + 2), None);
    } else if c == 4 {
        let a = getarglocs(pc, 1, ram);
        println!("OUTPUT: {}", ram[a[0]]);
        return (Some(pc + 2), Some(ram[a[0]]));
    } else if c == 5 {
        // Opcode 5 is jump-if-true: if the first parameter is non-zero,
        // it sets the instruction pointer to the value from the second
        // parameter. Otherwise, it does nothing.
        let a = getarglocs(pc, 2, ram);
        if ram[a[0]] != 0 {
            return (Some(ram[a[1]] as usize), None)
        } else {
            return (Some(pc + 3), None);
        }
    } else if c == 6 {
        // Opcode 6 is jump-if-false: if the first parameter is zero, it sets
        // the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.
        let a = getarglocs(pc, 2, ram);
        if ram[a[0]] == 0 {
            return (Some(ram[a[1]] as usize), None)
        } else {
            return (Some(pc + 3), None);
        }
    } else if c == 7 {
        let a = getarglocs(pc, 3, ram);
        ram[a[2]] = if ram[a[0]] < ram[a[1]] {
            1
        } else {
            0
        };
        return (Some(pc + 4), None);
    } else if c == 8 {
        let a = getarglocs(pc, 3, ram);
        ram[a[2]] = if ram[a[0]] == ram[a[1]] {
            1
        } else {
            0
        };
        return (Some(pc + 4), None);
    } else {
        assert!(false);
        return (None, None);
    };
}

fn run(ram: &mut Vec<i32>, input: i32) -> Option<i32>{
    println!("Hello, world! ram={:?}", ram);
    let mut pc: usize = 0;
    let mut last_output: Option<i32> = None;
    loop {
        let (optnewpc, optoutput) = step(pc, ram, input);
        match optoutput {
            None => {},
            Some(_) => {last_output = optoutput }
        }
        match optnewpc {
            Some(newpc) => pc = newpc,
            None => break
        }
    }
    println!("Goodbye, world! ram={:?} Last output={:?}", ram, last_output);
    return last_output;
}

fn run_lunarprog(noun: i32, verb: i32) -> i32 {
    let mut v = vec![1,noun,verb,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,13,19,23,2,23,9,27,1,6,27,
                     31,2,10,31,35,1,6,35,39,2,9,39,43,1,5,43,47,2,47,13,51,2,51,10,55,1,
                     55,5,59,1,59,9,63,1,63,9,67,2,6,67,71,1,5,71,75,1,75,6,79,1,6,79,83,
                     1,83,9,87,2,87,10,91,2,91,10,95,1,95,5,99,1,99,13,103,2,103,9,107,1,
                     6,107,111,1,111,5,115,1,115,2,119,1,5,119,0,99,2,0,14,0];
    run(&mut v, 0);
    //println!("NounVerb: {} Result: {:?}", 100*noun+verb, v[0]);
    v[0]
}

fn find_lunarprog_nounverb(expected_output: i32) -> Option<i32> {
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

fn aoc5(input:i32) -> Option<i32> {
    let mut v : Vec<i32> = vec![
        3,225,1,225,6,6,1100,1,238,225,104,0,1102,16,13,225,1001,88,68,224,101,-114,224,224,4,224,1002,223,8,223,1001,224,2,224,1,223,224,223,1101,8,76,224,101,-84,224,224,4,224,102,8,223,223,101,1,224,224,1,224,223,223,1101,63,58,225,1102,14,56,224,101,-784,224,224,4,224,102,8,223,223,101,4,224,224,1,223,224,223,1101,29,46,225,102,60,187,224,101,-2340,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1102,60,53,225,1101,50,52,225,2,14,218,224,101,-975,224,224,4,224,102,8,223,223,1001,224,3,224,1,223,224,223,1002,213,79,224,101,-2291,224,224,4,224,102,8,223,223,1001,224,2,224,1,223,224,223,1,114,117,224,101,-103,224,224,4,224,1002,223,8,223,101,4,224,224,1,224,223,223,1101,39,47,225,101,71,61,224,101,-134,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1102,29,13,225,1102,88,75,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,1107,677,677,224,102,2,223,223,1006,224,329,1001,223,1,223,108,677,677,224,1002,223,2,223,1005,224,344,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,359,1001,223,1,223,1107,226,677,224,102,2,223,223,1006,224,374,1001,223,1,223,8,677,226,224,102,2,223,223,1006,224,389,101,1,223,223,8,226,226,224,102,2,223,223,1006,224,404,101,1,223,223,7,677,677,224,1002,223,2,223,1006,224,419,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,434,101,1,223,223,1108,677,226,224,1002,223,2,223,1006,224,449,1001,223,1,223,108,677,226,224,1002,223,2,223,1006,224,464,101,1,223,223,1108,226,677,224,1002,223,2,223,1006,224,479,101,1,223,223,1007,677,677,224,1002,223,2,223,1006,224,494,1001,223,1,223,107,226,226,224,102,2,223,223,1005,224,509,1001,223,1,223,1008,677,226,224,102,2,223,223,1005,224,524,1001,223,1,223,1007,226,226,224,102,2,223,223,1006,224,539,101,1,223,223,1108,677,677,224,102,2,223,223,1005,224,554,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,569,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,584,1001,223,1,223,7,226,677,224,102,2,223,223,1005,224,599,101,1,223,223,108,226,226,224,1002,223,2,223,1005,224,614,101,1,223,223,107,226,677,224,1002,223,2,223,1005,224,629,1001,223,1,223,107,677,677,224,1002,223,2,223,1006,224,644,101,1,223,223,1007,677,226,224,1002,223,2,223,1006,224,659,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,674,1001,223,1,223,4,223,99,226
    ];
    run(&mut v, input)
}

fn main() {
    // Tests (part1):
    //run(&mut vec![1,0,0,0,99]);
    //run(&mut vec![2,3,0,3,99]);
    //run(&mut vec![2,4,4,5,99,0]);
    //run(&mut vec![1,1,1,4,99,5,6,0,99], 0);
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
    //run_lunarprog(12, 2);

    // phase2
    let nounverb = find_lunarprog_nounverb(19690720);
    println!("{:?}", nounverb);

    // AoC5 phase1 4601506 correct
    //aoc5(1);
    // AoC5 phase2 2762809 is wrong
    aoc5(5);
}


#[test]
fn t_aoc5_phase2() {
    assert_eq!(run(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], 21),
               Some(0));
    assert_eq!(run(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], 8),
               Some(1));

    // Using position mode, consider whether the input is
    // less than 8; output 1 (if it is) or 0 (if it is not).
    assert_eq!(run(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 7), Some(1));
    assert_eq!(run(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 8), Some(0));
    assert_eq!(run(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 9), Some(0));

    // Using immediate mode, consider whether the input is
    // less than 8; output 1 (if it is) or 0 (if it is not).
    assert_eq!(run(&mut vec![3,3,1107,-1,8,3,4,3,99], 7), Some(1));
    assert_eq!(run(&mut vec![3,3,1107,-1,8,3,4,3,99], 8), Some(0));
    assert_eq!(run(&mut vec![3,3,1107,-1,8,3,4,3,99], 9), Some(0));

    // Using immediate mode, consider whether the input is equal to 8;
    // output 1 (if it is) or 0 (if it is not).
    assert_eq!(run(&mut vec![3,3,1108,-1,8,3,4,3,99], 7), Some(0));
    assert_eq!(run(&mut vec![3,3,1108,-1,8,3,4,3,99], 8), Some(1));
    assert_eq!(run(&mut vec![3,3,1108,-1,8,3,4,3,99], 9), Some(0));

    // Here are some jump tests that take an input, then output 0 if
    // the input was zero or 1 if the input was non-zero:
    assert_eq!(run(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0), Some(0));
    assert_eq!(run(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 888), Some(1));
    assert_eq!(run(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0), Some(0));
    assert_eq!(run(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 888), Some(1));



    assert_eq!(run(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 5),
               Some(999));
    assert_eq!(run(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8),
               Some(1000));
    assert_eq!(run(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 9),
               Some(1001));
}

#[test]
fn t_aoc2_correct() {
    assert_eq!(find_lunarprog_nounverb(19690720), Some(9342));
}

#[test]
fn t_aoc5_correct() {
    assert_eq!(aoc5(1), Some(4601506));
    assert_eq!(aoc5(5), Some(5525561));
}
