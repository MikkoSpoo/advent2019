// https://adventofcode.com/2019/day/8

use std::io::Read;
use std::fs::File;


fn layer_counts(l: &Vec<Vec<u8>>) -> Vec<i32> {
	let mut v: Vec<i32> = vec![0;10];
	for row in l {
		for d in row {
			v[*d as usize] += 1;
		}
	}
	v
}

fn find_minimum_component(counts: &Vec<Vec<i32>>, comp: usize) -> usize {
	let mut minval: Option<i32> = None;
	let mut r = 0;
	for (i, c) in counts.iter().enumerate() {
		match minval {
			None => { minval = Some(c[comp]); r=i; },
			Some(v) => if c[comp] < v { minval = Some(c[comp]); r=i;},
		}
	}
	return r
}

fn read_from_str(s: &str, width: usize, height: usize) -> Vec<Vec<Vec<u8>>> {
	let lsize = width * height;
	let lnum = s.len() / lsize;
	let mut layers: Vec<Vec<Vec<u8>>> = Vec::with_capacity(lnum);
	let mut loc: usize = 0;
	for _ in 0..lnum {
		let mut l: Vec<Vec<u8>> = Vec::with_capacity(height);
		for _ in 0..height {
			l.push(s[loc..(loc+width)].chars().map(|c| (c as u8) - 48).collect());
			loc += width;
		}
		layers.push(l);
	}
	layers
}

fn phase1() -> Result<i32, Box<dyn std::error::Error>> {
	let mut s = String::new();
	File::open("input_data/aoc8.txt")?.read_to_string(&mut s)?;
	let t = read_from_str(&s, 25, 6);
	let counts: Vec<Vec<i32>> = t.iter().map(layer_counts).collect();
	let min_0_i = find_minimum_component(&counts, 0);
	let min_0_c = &counts[min_0_i];
	Ok(min_0_c[1] * min_0_c[2])
}

fn layers_to_stringpic(t: &Vec<Vec<Vec<u8>>>) -> String {
	let mut s: String = String::new();
	for y in 0..t[0].len() {
		for x in 0..t[0][0].len() {
			let mut b: bool = false;
			for l in t {
				let d = l[y][x];
				if d == 0 {
					break;
				} else if d == 1 {
					b = true;
					break;
				}
			}
			s.push(if b {'X'} else {'.'});
		}
		s.push('\n');
	}
	s
}

fn phase2() -> Result<String, Box<dyn std::error::Error>> {
	let mut s = String::new();
	File::open("input_data/aoc8.txt")?.read_to_string(&mut s)?;
	let t = read_from_str(&s, 25, 6);
	Ok(layers_to_stringpic(&t))
}

fn main() {
	//println!("Phase1: {:?}", phase1());
	println!("Phase2: \n{}", phase2().unwrap());
}

#[test]
fn t_aoc8_phase1() {
	assert_eq!(read_from_str("123456789012", 3, 2),
	           [[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [0, 1, 2]]]);
}

#[test]
fn t_aoc8_phase2() {
	assert_eq!(layers_to_stringpic(&read_from_str("0222112222120000", 2, 2)),
	           ".X\nX.\n");
}
