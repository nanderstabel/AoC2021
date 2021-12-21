use aoc::*;

struct Arrangement(Vec<Vec<u8>>, u32);

impl Arrangement {
	pub fn from(input: Vec<String>) -> Self {
		Arrangement(
			input
				.iter()
				.map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
				.collect(),
			0
		)
	}

	fn flash(&mut self, y: usize, x: usize) {
		if self.0[y][x] == 10 {
			return;
		}
		self.0[y][x] += 1;
		if self.0[y][x] == 10 {
			self.1 += 1;
			if y > 0 {
				if x > 0 {
					self.flash(y - 1, x - 1);
				}
				if x < 10 - 1 {
					self.flash(y - 1, x + 1);
				}
				self.flash(y - 1, x);
			}
			if x > 0 {
				self.flash(y, x - 1);
			}
			if x < 10 - 1 {
				self.flash(y, x + 1);
			}
			if y < 10 - 1 {
				if x > 0 {
					self.flash(y + 1, x - 1);
				}
				if x < 10 - 1 {
					self.flash(y + 1, x + 1);
				}
				self.flash(y + 1, x);
			}
		}
	}

	pub fn take_steps(&mut self, steps: u8) {
		for _ in 0..steps {
			for y in 0..10 {
				for x in 0..10 {
					self.flash(y, x);
				}
			}
			for y in 0..10 {
				for x in 0..10 {
					if self.0[y][x] == 10 {
						self.0[y][x] = 0;
					}
				}
			}
		}
		for line in &self.0 {
			println!("{:?}", line);
		}
	}
}

fn main() {
	let mut arrangement = Arrangement::from(read("input/d11"));

	arrangement.take_steps(100);

	output!(arrangement.1, 42);
}
