// https://adventofcode.com/2019/day/4

fn digitloop(between: (u32, u32)) -> u32 {
	let mut r : u32 = 0;
	for d1 in 0..10 {
		for d2 in d1..10 {
			for d3 in d2..10 {
				for d4 in d3..10 {
					for d5 in d4..10 {
						for d6 in d5..10 {
							let num :u32 = d6+10*(d5+10*(d4+10*(d3+10*(d2+10*d1))));
							//println!("{}{}{}{}{}{} = {}", d1,d2,d3,d4,d5,d6,num);
							if (num >= between.0 &&
								num <= between.1 &&
								(d1==d2 || d2 == d3 || d3==d4 || d4==d5 || d5==d6)) {
								println!("{}!", num);
								r += 1;
							}
						}
					}
				}
			}
		}
	}
	r
}

fn main() {
	println!("AoC4 phase1: {}", digitloop((171309,643603)));
}

