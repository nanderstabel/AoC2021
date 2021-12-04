use aoc::*;

trait IncCount {
	fn increment_count(&self) -> i32;
}

impl IncCount for Vec<i32> {
	fn increment_count(&self) -> i32 {
		self.windows(2).filter(|p| p[1] > p[0]).count() as i32
	}
}

fn main() {
	let input: Vec<i32> = read("input/d01");

	output!(
		input.increment_count(),
		input
			.windows(3)
			.map(|t| t.iter().sum())
			.collect::<Vec<i32>>()
			.increment_count()
	);
}
