use aoc::*;

fn split_input(input: &Vec<String>) -> (Vec<u8>, Vec<Vec<i8>>) {
	(
		input[0]
			.split(",")
			.map(|s| s.parse::<u8>().unwrap())
			.collect(),
		input[1..]
			.iter()
			.filter(|v| v.len() != 0)
			.map(|s| {
				s.split_whitespace()
					.map(|s| s.parse::<i8>().unwrap())
					.collect()
			})
			.collect(),
	)
}

fn main() {
	let input: Vec<String> = read("input/d04");
	let (draw, mut cards) = split_input(&input);
	let mut cards: Vec<_> = cards.chunks_mut(5).collect();

	cards[0][0][0] = -1;
	println!("{:?}", draw);
	println!("{:?}", cards);
}
