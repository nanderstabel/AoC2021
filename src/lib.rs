use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn read(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.unwrap().parse().unwrap()).collect()
}
