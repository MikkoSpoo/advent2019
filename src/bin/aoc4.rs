// https://adventofcode.com/2019/day/4

fn adjcheck1(d1:u32,d2:u32,d3:u32,d4:u32,d5:u32,d6:u32) -> bool {
	(d1==d2 || d2 == d3 || d3==d4 || d4==d5 || d5==d6)
}

fn adjcheck2(d1:u32,d2:u32,d3:u32,d4:u32,d5:u32,d6:u32) -> bool {
	((d1 == d2 && d2 != d3) ||
	 (d1 != d2 && d2 == d3 && d3 != d4) ||
	 (d2 != d3 && d3 == d4 && d4 != d5) ||
	 (d3 != d4 && d4 == d5 && d5 != d6) ||
	 (d4 != d5 && d5 == d6))
}

fn digitloop(between: (u32, u32)) -> (u32, u32) {
	let mut r1 : u32 = 0;
	let mut r2 : u32 = 0;
	for d1 in 0..10 {
		for d2 in d1..10 {
			for d3 in d2..10 {
				for d4 in d3..10 {
					for d5 in d4..10 {
						for d6 in d5..10 {
							let num :u32 = d6+10*(d5+10*(d4+10*(d3+10*(d2+10*d1))));
							//println!("{}{}{}{}{}{} = {}", d1,d2,d3,d4,d5,d6,num);
							if num >= between.0 && num <= between.1 {
								if adjcheck1(d1,d2,d3,d4,d5,d6) {r1 += 1};
								if adjcheck2(d1,d2,d3,d4,d5,d6) {r2 += 1};
							}
						}
					}
				}
			}
		}
	}
	(r1, r2)
}

fn main() {
	println!("{:?}", digitloop((171309,643603)));
}

