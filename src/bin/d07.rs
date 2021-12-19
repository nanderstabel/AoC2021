use aoc::*;

fn calculate_fuel<F: Fn(u32) -> u32>(crabs: &Vec<u32>, cost: F) -> u32 {
	let mut fuel = u32::MAX;
	for alignment in 0..u32::MAX {
		let new = crabs
			.iter()
			.map(|&position| {
				if position > alignment {
					cost(position - alignment)
				} else {
					cost(alignment - position)
				}
			})
			.sum();
		if fuel < new {
			break;
		}
		fuel = new;
	}
	fuel
}

fn main() {
	let input: Vec<String> = read("input/d07");
	let crabs: Vec<u32> = input[0].split(",").map(|f| f.parse().unwrap()).collect();

	output!(
		calculate_fuel(&crabs, |n| n),
		calculate_fuel(&crabs, |n| n * (n + 1) / 2)
	);
}
