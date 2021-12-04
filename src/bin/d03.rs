use aoc::*;
use derive_more::{BitAnd, BitOrAssign, Shl, ShlAssign};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, BitAnd, Shl, ShlAssign, BitOrAssign, PartialEq, PartialOrd)]
pub struct Binary(u16);

impl FromStr for Binary {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Binary(u16::from_str_radix(s, 2).unwrap()) << 0)
	}
}

fn main() {
	let input: Vec<Binary> = read("input/d03");
	let size = input.len();
	let mut gamma: Binary = Binary(0);

	for sh in 0..12 {
		gamma |= Binary(1);
		gamma <<= 1;
		input
			.iter()
			.filter(|b| (**b & Binary(1 << sh)) > Binary(0))
			.count();
		println!("{:?}", gamma);
	}
	output!(1, 2);
}
