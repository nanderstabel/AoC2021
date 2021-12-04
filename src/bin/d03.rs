use aoc::*;
use derive_more::{BitAnd, BitOrAssign, Display, Mul, Not, Shl};
use std::str::FromStr;

#[derive(Display, Copy, Clone, BitAnd, Shl, BitOrAssign, PartialEq, PartialOrd, Not, Mul)]
#[mul(forward)]
pub struct Binary(u32);

impl FromStr for Binary {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Binary(u32::from_str_radix(s, 2).unwrap()) << 0)
	}
}

fn main() {
	let input: Vec<Binary> = read("input/d03");
	let mut gamma = Binary(0);

	for sh in 0..12 {
		if input
			.iter()
			.filter(|b| (**b & Binary(1 << sh)) != Binary(0))
			.count() > input.len() / 2
		{
			gamma |= Binary(1 << sh);
		}
	}
	output!(gamma * (!gamma & Binary(0b1111_11111111u32)), 0);
}
