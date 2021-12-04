use aoc::*;
use std::{
	ops::{Index, IndexMut},
	str::FromStr,
};

#[derive(Copy, Clone, Debug)]
pub enum Direction {
	Down,
	Up,
	Forward,
}

impl FromStr for Direction {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"down" => Ok(Direction::Down),
			"up" => Ok(Direction::Up),
			"forward" => Ok(Direction::Forward),
			_ => Err(()),
		}
	}
}

impl<T> Index<Direction> for [T] {
	type Output = T;

	fn index(&self, direction: Direction) -> &T {
		Index::index(self, direction as usize)
	}
}

impl<T> IndexMut<Direction> for [T] {
	fn index_mut(&mut self, direction: Direction) -> &mut T {
		IndexMut::index_mut(self, direction as usize)
	}
}

pub type Command = Tuple<Direction, u32>;

fn main() {
	let input: Vec<Command> = read("input/d02");
	let mut directions = [0u32; 3];
	let (mut position, mut depth, mut aim) = (0, 0, 0);

	for command in input {
		directions[command.0] += command.1;
		match command.0 {
			Direction::Forward => {
				position += command.1;
				depth += command.1 * aim;
			}
			Direction::Down => aim += command.1,
			Direction::Up => aim -= command.1,
		}
	}
	output!(
		directions[Direction::Forward] * (directions[Direction::Down] - directions[Direction::Up]),
		position * depth
	);
}
