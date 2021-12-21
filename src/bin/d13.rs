use aoc::*;
use array_tool::vec::*;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
struct Dot {
	x: usize,
	y: usize,
}

impl FromStr for Dot {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let vec: Vec<&str> = s.split(",").collect();
		let dot: Vec<usize> = vec.iter().filter_map(|s| s.parse().ok()).collect();
		match dot.len() {
			2 => Ok(Dot {
				x: dot[0],
				y: dot[1],
			}),
			_ => Err(()),
		}
	}
}

struct Instruction {
	dim: char,
	index: usize,
}

impl FromStr for Instruction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let vec: Vec<&str> = s.split_whitespace().collect();
		if vec.len() == 3 {
			let res: Vec<&str> = vec[2].split("=").collect();
			return Ok(Instruction {
				dim: res[0].chars().nth(0).unwrap(),
				index: res[1].parse().unwrap(),
			});
		}
		Err(())
	}
}

fn main() {
	let input: Vec<String> = read("input/d13");
	let mut dots: Vec<Dot> = input.iter().filter_map(|l| l.parse().ok()).collect();
	let instructions: Vec<Instruction> = input.iter().filter_map(|l| l.parse().ok()).collect();
	let mut first_fold = 0;
	for instruction in instructions {
		if instruction.dim == 'x' {
			for dot in dots.iter_mut() {
				if dot.x > instruction.index {
					dot.x = instruction.index - (dot.x - instruction.index);
				}
			}
		} else {
			for dot in dots.iter_mut() {
				if dot.y > instruction.index {
					dot.y = instruction.index - (dot.y - instruction.index);
				}
			}
		}
		dots = dots.unique();
		if dots.len() > first_fold {
			first_fold = dots.len();
		}
	}
	let mut paper = [[' '; 45]; 6];
	for dot in dots {
		paper[dot.y][dot.x] = '#';
	}
	for line in paper {
		println!("{:?}", line);
	}
	output!(first_fold, "RKHFZGUB");
}
