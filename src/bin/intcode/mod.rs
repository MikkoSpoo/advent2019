// Intcode computer from AoC

// https://adventofcode.com/2019/day/2
// https://adventofcode.com/2019/day/5

// Iterators as arguments bit tricky, would be nice here
//use std::iter;
// https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html

use std::io::Read;
use std::fs::File;

use std::sync::mpsc::{channel, Sender, Receiver};

pub fn mem_from_string(s: &str) -> Result<Vec<i128>, Box<dyn std::error::Error>> {
    let mut r :Vec<i128> = Vec::new();
    for s in s.trim().split(',') {
        r.push(s.parse()?)
    }
    Ok(r)
}

pub fn mem_from_file(filename: &str) -> Result<Vec<i128>, Box<dyn std::error::Error>> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    mem_from_string(&s)
}

pub fn run_inpv(ram: &mut Vec<i128>, input: &Vec<i128>) -> Vec<i128> {
    let (inputtx, inputrx) = channel();
    let (outputtx, outputrx) = channel();
    for d in input {
        inputtx.send(*d).unwrap();
    }
    drop(inputtx);
    drop(outputrx); // give closed channels
    run_channels(ram, inputrx, outputtx).unwrap()
}

pub fn run_channels(ram: &mut Vec<i128>, rx: Receiver<i128>, tx: Sender<i128>) -> Result<Vec<i128>, Box<dyn std::error::Error>> {
    //println!("Hello, world! ram={:?}", ram);
    let mut pc: usize = 0;
    let mut v_output: Vec<i128> = vec![];
    let mut inputval = None;
    let mut relative_base: i128 = 0;
    // could try_recv value here to be ready
    // match input.get(inputpos) {
    //    None => None,
    //    Some(v) => Some(*v)
    //};
    loop {
        let (optnewpc, next_relative_base, optoutput, usedinput) = step(pc, relative_base, ram, inputval);
        relative_base = next_relative_base;
        match optoutput {
            None => {},
            Some(output) => { v_output.push(output);
                              let _ = tx.send(output); }
        }
        match optnewpc {
            Some(newpc) => pc = newpc,
            None => break
        }
        if usedinput {
            match inputval {
                None => {
                    //println!("pc {:?} reading input", pc);
                    inputval = Some(rx.recv()?); // blocks
                },
                Some(_) => {
                    // used value
                    // could try_recv value here to be ready
                    inputval = None;
                },
            }
        }
    }
    //println!("Goodbye, world! ram={:?} Last output={:?}", ram, last_output);
    Ok(v_output)
}

pub fn run_inpi(ram: &mut Vec<i128>, input: i128) -> Vec<i128> {
    run_inpv(ram, &(vec![input]))
}

pub fn run(ram: &mut Vec<i128>) -> Vec<i128> {
    run_inpv(ram, &(vec![]))
}

// TODO: use splices / mutable references
// Increases memory size to fit references
// TODO: could only resize on write
fn getarglocs(pc: usize, relative_base: i128,
              numofargs: usize, ram: &mut Vec<i128>)
              -> Vec<usize>
{
    let mut r = vec![0;numofargs];
    let mut o = ram[pc] / 100;
    for i in 0..numofargs {
        let loc = pc + i + 1;
        let mode = o % 10;
        r[i] = if mode == 0 {
            // 0 normal: location
            ram[loc] as usize
        } else if mode == 1 {
            // 1: immediate
            loc
        } else if mode == 2 {
            // 2: relative
            //println!("pc {:?} op {:?} arg {:?} relative_base {:?} arg {:?}",
            //         pc, ram[pc], i, relative_base, ram[loc]);
            (ram[loc] + relative_base) as usize
        } else {
            panic!("pc {:?} op {:?} invalid mode {:?}", pc, o, mode);
        };
        if r[i] >= ram.len() {
            //println!("pc {:?} resizing mem {} -> {}", pc, ram.len(), r[i]+1);
            ram.resize(r[i]+1, 0);
        }
        o = o / 10;
    }
    r
}

#[test]
fn t_intcode_getarglocs() {
    assert_eq!(getarglocs(0, 0, 3, &mut vec![1101,100,-1,4,0]),
               vec![1, 2, 4]);

}

fn step(pc: usize, relative_base: i128,
        ram: &mut Vec<i128>, input: Option<i128>)
        -> (Option<usize>, i128, Option<i128>, bool) {
    let op = ram[pc];
    let c = op % 100;
    let mut next_relative_base = relative_base;
    let mut usedinput = false;
    let mut output: Option<i128> = None;
    let next_pc :Option<usize> =
        if c == 99 {
            None
        } else if c == 1 {
            let a = getarglocs(pc, relative_base, 3, ram);
            ram[a[2]] = ram[a[0]] + ram[a[1]];
            Some(pc + 4)
        } else if c == 2 {
            let a = getarglocs(pc, relative_base, 3, ram);
            ram[a[2]] = ram[a[0]] * ram[a[1]];
            Some(pc + 4)
        } else if c == 3 {
            let a = getarglocs(pc, relative_base, 1, ram);
            match input {
                None => {
                    //println!("Needs input, not given, doing nothing");
                    usedinput=true;
                    Some(pc)
                },
                Some(v) => {
                    //println!("INPUT: {} to loc {}", v, a[0]);
                    ram[a[0]] = v;
                    usedinput = true;
                    Some(pc + 2)
                }
            }
        } else if c == 4 {
            let a = getarglocs(pc, relative_base, 1, ram);
            //println!("OUTPUT: {}", ram[a[0]]);
            output = Some(ram[a[0]]);
            Some(pc + 2)
        } else if c == 5 {
            // Opcode 5 is jump-if-true: if the first parameter is non-zero,
            // it sets the instruction pointer to the value from the second
            // parameter. Otherwise, it does nothing.
            let a = getarglocs(pc, relative_base, 2, ram);
            if ram[a[0]] != 0 {
                Some(ram[a[1]] as usize)
            } else {
                Some(pc + 3)
            }
        } else if c == 6 {
            // Opcode 6 is jump-if-false: if the first parameter is zero, it sets
            // the instruction pointer to the value from the second parameter.
            // Otherwise, it does nothing.
            let a = getarglocs(pc, relative_base, 2, ram);
            if ram[a[0]] == 0 {
                Some(ram[a[1]] as usize)
            } else {
                Some(pc + 3)
            }
        } else if c == 7 {
            let a = getarglocs(pc, relative_base, 3, ram);
            ram[a[2]] = if ram[a[0]] < ram[a[1]] {
                1
            } else {
                0
            };
            Some(pc + 4)
        } else if c == 8 {
            let a = getarglocs(pc, relative_base, 3, ram);
            ram[a[2]] = if ram[a[0]] == ram[a[1]] {
                1
            } else {
                0
            };
            Some(pc + 4)
        } else if c == 9 {
            // Opcode 9 adjusts the relative base by the value of its only parameter
            let a = getarglocs(pc, relative_base, 1, ram);
            next_relative_base = next_relative_base + ram[a[0]];
            //println!("pc {:?} op {:?} relative_base {:?} arg at {:?} adjust by {:?} {:?}",
            //         pc, op, relative_base, a[0], ram[a[0]], next_relative_base);
            Some(pc + 2)
        } else {
            panic!("pc {:?} bad instruction {:?}", pc, op);
        };
    (next_pc, next_relative_base, output, usedinput)
}

// for test
fn rtl(ram: &mut Vec<i128>, input: i128) -> Option<i128> {
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

#[test]
fn t_intcode_aoc9_phase1() {
    // takes no input and produces a copy of itself as output.
    assert_eq!(rtl(&mut vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], 0),
               Some(99));
    // should output the large number in the middle
    assert_eq!(rtl(&mut vec![104,1125899906842624,99], 0),
               Some(1125899906842624));

}
