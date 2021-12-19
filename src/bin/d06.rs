use aoc::*;
use std::collections::VecDeque;

struct Shoaling(VecDeque<u64>);

impl Shoaling {
	pub fn from(input: Vec<usize>) -> Self {
		let mut fish = VecDeque::from_iter([0u64; 9]);
		for i in input {
			fish[i] += 1;
		}
		Shoaling(fish)
	}

	pub fn reproduce(&mut self, days: u16) -> u64 {
		for _day in 1..=days {
			self.0.rotate_left(1);
			self.0[6] += self.0[8];
		}
		self.0.iter().sum::<u64>()
	}
}

fn main() {
	let input: Vec<String> = read("input/d06");
	let mut shoaling = Shoaling::from(input[0].split(",").map(|f| f.parse().unwrap()).collect());
	output!(shoaling.reproduce(80), shoaling.reproduce(256 - 80));
}
