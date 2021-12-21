use anyhow::{anyhow, Result};
use aoc::*;
use phf::phf_map;
use std::collections::VecDeque;

struct Score {
	corrupted: u32,
	incomplete: u64,
}

static SCORES: phf::Map<char, Score> = phf_map! {
	')' => Score { corrupted: 3, incomplete: 1 },
	']' => Score { corrupted: 57, incomplete: 2 },
	'}' => Score { corrupted: 1197, incomplete: 3 },
	'>' => Score { corrupted: 25137, incomplete: 4 },
};

struct Subsystem {
	corrupted_score: u32,
	incomplete_scores: VecDeque<u64>,
}

impl Subsystem {
	pub fn new() -> Self {
		Subsystem {
			corrupted_score: 0,
			incomplete_scores: VecDeque::from_iter(vec![0]),
		}
	}

	fn get(&mut self, it: &mut dyn Iterator<Item = char>, matching: char) -> Result<Option<char>> {
		match self.find_chunks(it)? {
			Some(p) if p == matching => self.find_chunks(it),
			Some(other) => {
				self.corrupted_score += SCORES.get(&other).unwrap().corrupted;
				Err(anyhow!("Input is corrupted"))
			}
			None => {
				self.incomplete_scores[0] *= 5;
				self.incomplete_scores[0] += SCORES.get(&matching).unwrap().incomplete;
				Ok(None)
			}
		}
	}

	fn find_chunks(&mut self, it: &mut dyn Iterator<Item = char>) -> Result<Option<char>> {
		match it.next() {
			Some('(') => self.get(it, ')'),
			Some('[') => self.get(it, ']'),
			Some('{') => self.get(it, '}'),
			Some('<') => self.get(it, '>'),
			current => Ok(current),
		}
	}

	pub fn evaluate(&mut self, input: &Vec<String>) {
		for line in input {
			if self.find_chunks(&mut line.chars()).is_ok() {
				self.incomplete_scores.push_front(0)
			}
		}
	}

	pub fn get_corrupted_score(&self) -> u32 {
		self.corrupted_score
	}

	pub fn get_incomplete_score(&mut self) -> u64 {
		self.incomplete_scores.make_contiguous().sort();
		self.incomplete_scores[self.incomplete_scores.len() / 2]
	}
}

fn main() {
	let mut system = Subsystem::new();
	system.evaluate(&read("input/d10"));
	output!(system.get_corrupted_score(), system.get_incomplete_score());
}
