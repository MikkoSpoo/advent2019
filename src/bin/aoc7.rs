// https://adventofcode.com/2019/day/7

mod intcode;

fn run_amplifier(rom: & Vec<i32>,
                 phase_setting :i32,
                 input_signal: i32)
                 -> Option<i32> {
    let mut ram = rom.clone();
    let v_output = intcode::run_inpv(&mut ram, & vec![phase_setting, input_signal]);
    Some(*(v_output.last()?)) // last output, or None
}

fn run_amplifiers(rom: & Vec<i32>, phase_settings: & Vec<i32>) -> Option<i32> {
    let mut signal = 0;
    for ps in phase_settings {
        signal = run_amplifier(rom, *ps, signal)?
    }
    Some(signal)
}

// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn heaps_alg(k : usize, v: &mut Vec<i32>, v_out :&mut Vec<Vec<i32>>) {
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

fn find_highest_trust_perm(rom: & Vec<i32>) -> Option<i32> {
    let mut perms :Vec<Vec<i32>> = Vec::new();
    let mut highest_thrust :Option<i32> = None;
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

fn main() {
    println!("Phase 1: {:?}",
             find_highest_trust_perm(&(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0])));
}

#[test]
fn t_aoc7() {
    assert_eq!(run_amplifiers(&(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]),
                              &(vec![4,3,2,1,0])),
               Some(43210));
    assert_eq!(find_highest_trust_perm(&(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0])),
               Some(43210));
}
