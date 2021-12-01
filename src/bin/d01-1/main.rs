use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn get(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn main() {
    let lines = get("src/bin/d01-1/input");
    for line in lines {
        println!("{:?}", line);
    } 
}