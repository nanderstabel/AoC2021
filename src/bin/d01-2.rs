use aoc::*;

fn main() {
    let input = read("input/d01");
    println!(
        "{:?}",
        input
            .windows(3)
            .map(|t| t.iter().sum())
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|p| p[1] > p[0])
            .count()
    );
}
