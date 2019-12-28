// https://adventofcode.com/2019/day/7

use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Sender, Receiver};

mod intcode;

fn run_amplifier(rom: & Vec<i128>,
                 phase_setting :i128,
                 input_signal: i128)
                 -> Option<i128> {
    let mut ram = rom.clone();
    let v_output = intcode::run_inpv(&mut ram, & vec![phase_setting, input_signal.into()]);
    Some(*(v_output.last()?)) // last output, or None
}

fn run_amplifiers(rom: & Vec<i128>, phase_settings: & Vec<i128>) -> Option<i128> {
    let mut signal: i128 = 0;
    for ps in phase_settings {
        signal = run_amplifier(rom, *ps, signal)?
    }
    Some(signal)
}

// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn heaps_alg(k : usize, v: &mut Vec<i128>, v_out :&mut Vec<Vec<i128>>) {
    if k == 1 {
        v_out.push(v.clone());
    } else {
        // Generate permutations with kth unaltered
        // Initially k == length(A)
        heaps_alg(k - 1, v, v_out);

        // Generate permutations for kth swapped with each k-1 initial
        for i in 0..(k-1) {
            // Swap choice dependent on parity of k (even or odd)
            if k % 2 == 0 {
                let t = v[i];
                v[i] = v[k-1];
                v[k-1] = t;
            } else {
                let t = v[0];
                v[0] = v[k-1];
                v[k-1] = t;
            }
            heaps_alg(k - 1, v, v_out);
        }
    }
}

fn find_highest_trust_perm(rom: & Vec<i128>) -> Option<i128> {
    let mut perms :Vec<Vec<i128>> = Vec::new();
    let mut highest_thrust :Option<i128> = None;
    heaps_alg(5, &mut (vec![0, 1, 2, 3, 4]), &mut perms);
    for v in perms {
        println!("v {:?}", v);
        match run_amplifiers(rom, &v) {
            None => (),
            Some(v) => match highest_thrust {
                None => highest_thrust = Some(v),
                Some(high_v) => if v > high_v { highest_thrust = Some(v) }
            }
        }
    }
    return highest_thrust;
}

fn find_highest_trust_perm2(rom: & Vec<i128>) -> Option<i128> {
    let mut perms :Vec<Vec<i128>> = Vec::new();
    let mut highest_thrust :Option<i128> = None;
    heaps_alg(5, &mut (vec![5, 6, 7, 8, 9]), &mut perms);
    for v in perms {
        println!("v {:?}", v);
        match run_feedback_loop(rom, &v) {
            None => (),
            Some(v) => match highest_thrust {
                None => highest_thrust = Some(v),
                Some(high_v) => if v > high_v { highest_thrust = Some(v) }
            }
        }
    }
    return highest_thrust;
}


fn run_amp2(rom: & Vec<i128>, rx: Receiver<i128>, tx: Sender<i128>) ->  JoinHandle<Vec<i128>> {
    let mut ram = rom.clone();
    thread::spawn( move || {    
        match intcode::run_channels(&mut ram, rx, tx) {
            Ok(v) => v,
            Err(e) => {
                println!("Error {:?}", e);
                vec![]
            }
        }

    })
}

fn run_feedback_loop(rom: & Vec<i128>, phase_settings: & Vec<i128>) -> Option<i128> {
    let (tx_a, rx_b) = channel::<i128>();
    let (tx_b, rx_c) = channel::<i128>();
    let (tx_c, rx_d) = channel::<i128>();
    let (tx_d, rx_e) = channel::<i128>();
    let (tx_e, rx_a) = channel::<i128>();
    tx_e.send(phase_settings[0]).unwrap(); // to a
    tx_a.send(phase_settings[1]).unwrap();
    tx_b.send(phase_settings[2]).unwrap();
    tx_c.send(phase_settings[3]).unwrap();
    tx_d.send(phase_settings[4]).unwrap();
    tx_e.send(0).unwrap();
    let h_a = run_amp2(rom, rx_a, tx_a);
    let h_b = run_amp2(rom, rx_b, tx_b);
    let h_c = run_amp2(rom, rx_c, tx_c);
    let h_d = run_amp2(rom, rx_d, tx_d);
    let h_e = run_amp2(rom, rx_e, tx_e);
    let resultva = h_a.join().unwrap();
    let resultvb = h_b.join().unwrap();
    let resultvc = h_c.join().unwrap();
    let resultvd = h_d.join().unwrap();
    let resultve = h_e.join().unwrap();
    println!("{:?} {:?} {:?} {:?} {:?}", resultva, resultvb, resultvc, resultvd, resultve);
    Some(*(resultve.last()?))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Phase 1: {:?}",
             find_highest_trust_perm(&(intcode::mem_from_file("input_data/aoc7.txt")?)));
    println!("Phase 2: {:?}",
             find_highest_trust_perm2(&(intcode::mem_from_file("input_data/aoc7.txt")?)));
    Ok(())
}

#[test]
fn t_aoc7_1() {
    assert_eq!(run_amplifiers(&(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]),
                              &(vec![4,3,2,1,0])),
               Some(43210));
    assert_eq!(find_highest_trust_perm(&(intcode::mem_from_file("test_data/aoc_7_1_example1.txt").unwrap())),
               Some(43210));
}

#[test]
fn t_aoc7_2() {
    assert_eq!(run_feedback_loop(&(intcode::mem_from_file("test_data/aoc_7_2_example1.txt").unwrap()), &(vec![9,8,7,6,5])),
               Some(139629729));
}
