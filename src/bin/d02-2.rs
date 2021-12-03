use aoc::{Direction::*, *};

fn main() {
    let input: Vec<Command> = read("input/d02");
    let (mut position, mut depth, mut aim) = (0, 0, 0);

    for command in input {
        match command.0 {
            Forward => {
                position += command.1;
                depth += command.1 * aim;
            }
            Down => aim += command.1,
            Up => aim -= command.1,
        }
    }
    println!("{:?}", position * depth);
}
