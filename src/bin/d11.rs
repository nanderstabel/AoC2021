use aoc::*;

struct Arrangement {
	map: Vec<Vec<u8>>,
	steps: u32,
	flashes: u32,
}

impl Arrangement {
	pub fn from(input: Vec<String>) -> Self {
		Arrangement {
			map: input
				.iter()
				.map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
				.collect(),
			steps: 0,
			flashes: 0,
		}
	}

	fn flash(&mut self, y: isize, x: isize) {
		if y < 0 || y == 10 || x < 0 || x == 10 || self.map[y as usize][x as usize] == 10 {
			return;
		}
		self.map[y as usize][x as usize] += 1;
		if self.map[y as usize][x as usize] == 10 {
			self.flashes += 1;
			self.flash(y - 1, x - 1);
			self.flash(y - 1, x);
			self.flash(y - 1, x + 1);
			self.flash(y, x - 1);
			self.flash(y, x + 1);
			self.flash(y + 1, x - 1);
			self.flash(y + 1, x);
			self.flash(y + 1, x + 1);
		}
	}
}

impl Iterator for Arrangement {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		for y in 0..10 {
			for x in 0..10 {
				self.flash(y, x);
			}
		}
		for y in 0..10 {
			for x in 0..10 {
				if self.map[y][x] == 10 {
					self.map[y][x] = 0;
				}
			}
		}
		self.steps += 1;
		if self.map.iter().flatten().filter(|&&x| x == 0).count() == 100 {
			return None;
		}
		Some(self.flashes)
	}
}

fn main() {
	let mut arrangement = Arrangement::from(read("input/d11"));
	let mut step100 = 0;
	while let Some(flashes) = arrangement.next() {
		if arrangement.steps == 100 {
			step100 = flashes;
		}
	}
	output!(step100, arrangement.steps);
}
