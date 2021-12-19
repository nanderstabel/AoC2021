#![feature(drain_filter)]
use aoc::*;

type Card<'a> = &'a mut [Vec<Option<u16>>];

fn split_input(input: &Vec<String>) -> (Vec<u8>, Vec<Vec<Option<u16>>>) {
	(
		input[0]
			.split(",")
			.map(|s| s.parse::<u8>().unwrap())
			.collect(),
		input[1..]
			.iter()
			.filter(|v| !v.is_empty())
			.map(|s| {
				s.split_whitespace()
					.map(|s| Some(s.parse::<u16>().unwrap()))
					.collect()
			})
			.collect(),
	)
}

fn is_complete(array: &[Option<u16>]) -> bool {
	array.iter().filter(|n| **n == None).count() == 5
}

fn complete(num: u8, card: Card) -> Option<u16> {
	for y in 0..5 {
		for x in 0..5 {
			if card[y][x] == Some(num as u16) {
				card[y][x] = None;
				if is_complete(&[card[0][x], card[1][x], card[2][x], card[3][x], card[4][x]])
					|| is_complete(&card[y])
				{
					return Some(card.iter().flatten().flat_map(|x| x).sum::<u16>() * num as u16);
				}
			}
		}
	}
	None
}

fn main() {
	let input: Vec<String> = read("input/d04");
	let (draw, mut cards) = split_input(&input);
	let mut cards: Vec<Card> = cards.chunks_mut(5).collect();

	let (mut first, mut last) = (0, 0);
	for num in draw {
		cards.drain_filter(|card| {
			if let Some(result) = complete(num, card) {
				if first == 0 {
					first = result;
				}
				last = result;
				true
			} else {
				false
			}
		});
	}
	output!(first, last);
}
