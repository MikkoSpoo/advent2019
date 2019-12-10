// Intcode computer from AoC

// https://adventofcode.com/2019/day/2
// https://adventofcode.com/2019/day/5

// Iterators as arguments bit tricky, would be nice here
//use std::iter;
// https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html

use std::io;
use std::io::Read;
use std::fs::File;

pub fn mem_from_string(s: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut r :Vec<i32> = Vec::new();
    for s in s.trim().split(',') {
        r.push(s.parse()?)
    }
    Ok(r)
}

pub fn mem_from_file(filename: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    mem_from_string(&s)
}

pub fn run_inpv(ram: &mut Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    //println!("Hello, world! ram={:?}", ram);
    let mut pc: usize = 0;
    let mut v_output: Vec<i32> = vec![];
    let mut inputpos = 0;
    let mut inputval = match input.get(inputpos) {
        None => None,
        Some(v) => Some(*v)
    };
    loop {
        let (optnewpc, optoutput, usedinput) = step(pc, ram, inputval);
        match optoutput {
            None => {},
            Some(output) => { v_output.push(output) }
        }
        match optnewpc {
            Some(newpc) => pc = newpc,
            None => break
        }
        if usedinput {
            inputpos += 1;
            inputval = match input.get(inputpos) {
                None => None,
                Some(v) => Some(*v)
            }
        }
    }
    //println!("Goodbye, world! ram={:?} Last output={:?}", ram, last_output);
    v_output
}

pub fn run_inpi(ram: &mut Vec<i32>, input: i32) -> Vec<i32> {
    run_inpv(ram, &(vec![input]))
}

pub fn run(ram: &mut Vec<i32>) -> Vec<i32> {
    run_inpv(ram, &(vec![]))
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

fn step(pc: usize, ram: &mut Vec<i32>, input: Option<i32>)
        -> (Option<usize>, Option<i32>, bool) {
    let op = ram[pc];
    let c = op % 100;
    let mut usedinput = false;
    let mut output: Option<i32> = None;
    let next_pc :Option<usize> =
        if c == 99 {
            None
        } else if c == 1 {
            let a = getarglocs(pc, 3, ram);
            ram[a[2]] = ram[a[0]] + ram[a[1]];
            Some(pc + 4)
        } else if c == 2 {
            let a = getarglocs(pc, 3, ram);
            ram[a[2]] = ram[a[0]] * ram[a[1]];
            Some(pc + 4)
        } else if c == 3 {
            let a = getarglocs(pc, 1, ram);
            match input {
                None => {
                    panic!("pc {:?} no input", pc);
                },
                Some(v) => {
                    println!("INPUT: {} to loc {}", v, a[0]);
                    ram[a[0]] = v;
                    usedinput = true;
                    Some(pc + 2)
                }
            }
        } else if c == 4 {
            let a = getarglocs(pc, 1, ram);
            println!("OUTPUT: {}", ram[a[0]]);
            output = Some(ram[a[0]]);
            Some(pc + 2)
        } else if c == 5 {
            // Opcode 5 is jump-if-true: if the first parameter is non-zero,
            // it sets the instruction pointer to the value from the second
            // parameter. Otherwise, it does nothing.
            let a = getarglocs(pc, 2, ram);
            if ram[a[0]] != 0 {
                Some(ram[a[1]] as usize)
            } else {
                Some(pc + 3)
            }
        } else if c == 6 {
            // Opcode 6 is jump-if-false: if the first parameter is zero, it sets
            // the instruction pointer to the value from the second parameter.
            // Otherwise, it does nothing.
            let a = getarglocs(pc, 2, ram);
            if ram[a[0]] == 0 {
                Some(ram[a[1]] as usize)
            } else {
                Some(pc + 3)
            }
        } else if c == 7 {
            let a = getarglocs(pc, 3, ram);
            ram[a[2]] = if ram[a[0]] < ram[a[1]] {
                1
            } else {
                0
            };
            Some(pc + 4)
        } else if c == 8 {
            let a = getarglocs(pc, 3, ram);
            ram[a[2]] = if ram[a[0]] == ram[a[1]] {
                1
            } else {
                0
            };
            Some(pc + 4)
        } else {
            panic!("pc {:?} bad instruction {:?}", pc, op);
        };
    (next_pc, output, usedinput)
}

// for test
fn rtl(ram: &mut Vec<i32>, input: i32) -> Option<i32> {
    Some(*(run_inpi(ram, input).last()?))
}

#[test]
fn t_intcode_aoc5_phase2() {
    assert_eq!(rtl(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], 21),
               Some(0));
    assert_eq!(rtl(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], 8),
               Some(1));

    // Using position mode, consider whether the input is
    // less than 8; output 1 (if it is) or 0 (if it is not).
    assert_eq!(rtl(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 7), Some(1));
    assert_eq!(rtl(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 8), Some(0));
    assert_eq!(rtl(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], 9), Some(0));

    // Using immediate mode, consider whether the input is
    // less than 8; output 1 (if it is) or 0 (if it is not).
    assert_eq!(rtl(&mut vec![3,3,1107,-1,8,3,4,3,99], 7), Some(1));
    assert_eq!(rtl(&mut vec![3,3,1107,-1,8,3,4,3,99], 8), Some(0));
    assert_eq!(rtl(&mut vec![3,3,1107,-1,8,3,4,3,99], 9), Some(0));

    // Using immediate mode, consider whether the input is equal to 8;
    // output 1 (if it is) or 0 (if it is not).
    assert_eq!(rtl(&mut vec![3,3,1108,-1,8,3,4,3,99], 7), Some(0));
    assert_eq!(rtl(&mut vec![3,3,1108,-1,8,3,4,3,99], 8), Some(1));
    assert_eq!(rtl(&mut vec![3,3,1108,-1,8,3,4,3,99], 9), Some(0));

    // Here are some jump tests that take an input, then output 0 if
    // the input was zero or 1 if the input was non-zero:
    assert_eq!(rtl(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 0), Some(0));
    assert_eq!(rtl(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], 888), Some(1));
    assert_eq!(rtl(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 0), Some(0));
    assert_eq!(rtl(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], 888), Some(1));



    assert_eq!(rtl(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 5),
               Some(999));
    assert_eq!(rtl(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 8),
               Some(1000));
    assert_eq!(rtl(&mut vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], 9),
               Some(1001));
}
