use aoc::*;
use derive_more::{BitAnd, BitOrAssign, Display, Mul, Not, Shl, ShlAssign, Shr};
use std::str::FromStr;

#[derive(
	Copy, Clone, PartialEq, PartialOrd, Display, BitAnd, BitOrAssign, Not, Mul, Shl, Shr, ShlAssign,
)]
#[mul(forward)]
struct Binary(u32);

impl FromStr for Binary {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Binary(u32::from_str_radix(s, 2).unwrap()))
	}
}

enum Metric {
	O2,
	CO2,
}

fn get_rating(metric: Metric, (one, zero): (Vec<&Binary>, Vec<&Binary>), sb: Binary) -> Binary {
	macro_rules! get_rating {
		($criterion:tt) => {
			get_rating(
				metric,
				(if one.len() $criterion zero.len() {one} else {zero})
					.into_iter().partition(|b| **b & sb != Binary(0)),
				sb >> 1,
			)
		};
	}
	match (one.len(), zero.len()) {
		(1, 0) => *one[0],
		(0, 1) => *zero[0],
		_ => match metric {
			Metric::O2 => get_rating!(>=),
			Metric::CO2 => get_rating!(<),
		},
	}
}

fn main() {
	let input: Vec<Binary> = read("input/d03");
	let mut sb = Binary(1);
	let mut gamma = Binary(0);

	while sb < Binary(1 << 11) {
		if input.iter().filter(|b| (**b & sb) != Binary(0)).count() > input.len() / 2 {
			gamma |= sb;
		}
		sb <<= 1;
	}
	let input = &input.iter().map(|b| b).collect::<Vec<&Binary>>();
	let select = |b: &&&Binary| ***b & sb != Binary(0);

	output!(
		gamma * (!gamma & Binary(0b1111_11111111u32)),
		get_rating(Metric::O2, input.iter().partition(select), sb >> 1,)
			* get_rating(Metric::CO2, input.iter().partition(select), sb >> 1,)
	);
}
