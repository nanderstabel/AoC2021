use aoc::*;
use std::str::FromStr;

type Signals = Vec<String>;
type Output = Vec<String>;
pub struct Entry(Signals, Output);

impl FromStr for Entry {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let vec: Vec<&str> = s.split("|").collect();
		let collect_strings = |s: &str| -> Vec<String> {
			s.split_whitespace()
				.into_iter()
				.map(|s| String::from(s))
				.collect()
		};
		Ok(Entry(collect_strings(vec[0]), collect_strings(vec[1])))
	}
}

fn main() {
	let input: Vec<Entry> = read("input/d08");

	output!(
		input
			.iter()
			.map(|e| {
				e.1.iter()
					.filter(|s| [2, 3, 4, 7].contains(&s.len()))
					.count()
			})
			.sum::<usize>(),
		42
	);
}
