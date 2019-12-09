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

fn main() {
    println!("{:?}",
             run_amplifiers(&(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]),
                            &(vec![4,3,2,1,0])));
}

#[test]
fn t_aoc7() {
    assert_eq!(run_amplifiers(&(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]),
                              &(vec![4,3,2,1,0])),
               Some(43210));
}
