use aoc::*;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Horizontal(Vec<u8>);

impl FromStr for Horizontal {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Horizontal(
			s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(),
		))
	}
}

type Basin = HashSet<(usize, usize)>;

pub struct HeightMap {
	width: usize,
	length: usize,
	map: Vec<Horizontal>,
	risk_level_sum: u16,
	large_volume: usize,
	basins: Vec<usize>,
}

impl HeightMap {
	pub fn from(map: Vec<Horizontal>) -> Self {
		HeightMap {
			width: map[0].0.len(),
			length: map.len(),
			map,
			risk_level_sum: 0,
			large_volume: 0,
			basins: Vec::new(),
		}
	}

	fn fill_basin(&mut self, basin: &mut Basin, (y, x): (usize, usize)) {
		if self.map[y].0[x] == 9 {
			return;
		}
		if basin.insert((y, x)) {
			if y > 0 {
				if self.map[y - 1].0[x] > self.map[y].0[x] {
					self.fill_basin(basin, (y - 1, x));
				}
			}
			if y < self.length - 1 {
				if self.map[y + 1].0[x] > self.map[y].0[x] {
					self.fill_basin(basin, (y + 1, x));
				}
			}
			if x > 0 {
				if self.map[y].0[x - 1] > self.map[y].0[x] {
					self.fill_basin(basin, (y, x - 1));
				}
			}
			if x < self.width - 1 {
				if self.map[y].0[x + 1] > self.map[y].0[x] {
					self.fill_basin(basin, (y, x + 1));
				}
			}
		}
	}

	pub fn find_basins(&mut self) {
		for y in 0..self.length {
			for x in 0..self.width {
				if y > 0 {
					if self.map[y - 1].0[x] <= self.map[y].0[x] {
						continue;
					}
				}
				if y < self.length - 1 {
					if self.map[y + 1].0[x] <= self.map[y].0[x] {
						continue;
					}
				}
				if x > 0 {
					if self.map[y].0[x - 1] <= self.map[y].0[x] {
						continue;
					}
				}
				if x < self.width - 1 {
					if self.map[y].0[x + 1] <= self.map[y].0[x] {
						continue;
					}
				}
				let mut basin = HashSet::new();
				self.fill_basin(&mut basin, (y, x));
				self.basins.push(basin.len());
				self.risk_level_sum += self.map[y].0[x] as u16 + 1;
			}
		}
	}

	pub fn get_large_volume(&mut self) {
		self.basins.sort_by(|a, b| b.partial_cmp(a).unwrap());
		self.large_volume = self.basins[0] * self.basins[1] * self.basins[2];
	}
}

fn main() {
	let mut map = HeightMap::from(read("input/d09"));

	map.find_basins();
	map.get_large_volume();
	output!(map.risk_level_sum, map.large_volume);
}
