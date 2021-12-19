use aoc::*;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
pub struct Line(pub (u16, u16), pub (u16, u16));

impl FromStr for Line {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let vec: Vec<&str> = s.split(" -> ").collect();
		let (start, end) = vec
			.iter()
			.map(|p| {
				let a: Vec<&str> = p.split(",").collect();
				a.iter()
					.map(|c| c.parse().unwrap())
					.collect_tuple()
					.unwrap()
			})
			.collect_tuple()
			.unwrap();
		Ok(Line(start, end))
	}
}

fn fill(floor: &mut [[u8; 1000]; 1000], line: &Line) {
	match line {
		line if line.0 .1 < line.1 .1 && line.0 .0 == line.1 .0 => {
			for i in line.0 .1..=line.1 .1 {
				floor[i as usize][line.0 .0 as usize] += 1;
			}
		}
		line if line.0 .1 > line.1 .1 && line.0 .0 == line.1 .0 => {
			for i in line.1 .1..=line.0 .1 {
				floor[i as usize][line.0 .0 as usize] += 1;
			}
		}
		line if line.0 .1 == line.1 .1 && line.0 .0 < line.1 .0 => {
			for i in line.0 .0..=line.1 .0 {
				floor[line.0 .1 as usize][i as usize] += 1;
			}
		}
		line if line.0 .1 == line.1 .1 && line.0 .0 > line.1 .0 => {
			for i in line.1 .0..=line.0 .0 {
				floor[line.0 .1 as usize][i as usize] += 1;
			}
		}
		_ => (),
	}
}

fn fill_diagonally(floor: &mut [[u8; 1000]; 1000], line: Line) {
	match line {
		line if line.0 .1 < line.1 .1 && line.0 .0 < line.1 .0 => {
			let mut i = line.0 .0 as i16;
			for j in line.0 .1..=line.1 .1 {
				floor[j as usize][i as usize] += 1;
				i += 1;
			}
		}
		line if line.0 .1 < line.1 .1 && line.0 .0 > line.1 .0 => {
			let mut i = line.0 .0 as i16;
			for j in line.0 .1..=line.1 .1 {
				floor[j as usize][i as usize] += 1;
				i -= 1;
			}
		}
		line if line.0 .1 > line.1 .1 && line.0 .0 < line.1 .0 => {
			let mut i = line.1 .0 as i16;
			for j in line.1 .1..=line.0 .1 {
				floor[j as usize][i as usize] += 1;
				i -= 1;
			}
		}
		line if line.0 .1 > line.1 .1 && line.0 .0 > line.1 .0 => {
			let mut i = line.1 .0 as i16;
			for j in line.1 .1..=line.0 .1 {
				floor[j as usize][i as usize] += 1;
				i += 1;
			}
		}
		_ => (),
	}
}

fn main() {
	let input: Vec<Line> = read("input/d05");
	let mut floor = [[0u8; 1000]; 1000];
	for line in &input {
		fill(&mut floor, &line);
	}
	let part1 = floor.iter().flatten().filter(|&&p| p > 1).count();
	for line in input {
		fill_diagonally(&mut floor, line);
	}
	output!(part1, floor.iter().flatten().filter(|&&p| p > 1).count());
}
