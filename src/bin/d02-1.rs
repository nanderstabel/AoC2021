use aoc::{Direction::*, *};

fn main() {
    let input: Vec<Command> = read("input/d02");
    let mut directions = [0u32; 3];
    for command in input {
        directions[command.0] += command.1;
    }
    println!(
        "{:?}",
        directions[Forward] * (directions[Down] - directions[Up])
    );
}
