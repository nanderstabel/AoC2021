use aoc::*;
use derive_more::{BitAnd, BitOrAssign, Display, Mul, Not, Shl, Shr};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, PartialOrd, Display, BitAnd, BitOrAssign, Not, Mul, Shl, Shr)]
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

fn get_rating(metric: Metric, input: Vec<&Binary>, sh: Binary) -> Binary {
	if input.len() == 1 {
		return *input[0];
	}
	let (one, zero): (_, Vec<&Binary>) = input.into_iter().partition(|b| **b & sh != Binary(0));

	macro_rules! get_rating {
		($criteria:tt) => {
			get_rating(
				metric,
				match one.len() $criteria zero.len() {
					true => one,
					false => zero,
				},
				sh >> 1,
			)
		};
	}
	match metric {
		Metric::O2 => get_rating!(>=),
		Metric::CO2 => get_rating!(<),
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
	output!(
		gamma * (!gamma & Binary(0b1111_11111111u32)),
		get_rating(
			Metric::O2,
			input.iter().map(|b| b).collect(),
			Binary(1 << 11),
		) * get_rating(
			Metric::CO2,
			input.iter().map(|b| b).collect(),
			Binary(1 << 11),
		)
	);
}
