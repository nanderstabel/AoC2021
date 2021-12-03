use aoc::*;

fn main() {
    let input: Vec<i32> = read("input/d01");
    println!("{}", input.windows(2).filter(|p| p[1] > p[0]).count());
}
