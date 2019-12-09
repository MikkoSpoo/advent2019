// Intcode computer from AoC

// https://adventofcode.com/2019/day/2
// https://adventofcode.com/2019/day/5

pub fn run(ram: &mut Vec<i32>, input: i32) -> Option<i32>{
    //println!("Hello, world! ram={:?}", ram);
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
    //println!("Goodbye, world! ram={:?} Last output={:?}", ram, last_output);
    return last_output;
}

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
fn t_intcode_getarglocs() {

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

#[test]
fn t_intcode_aoc5_phase2() {
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
