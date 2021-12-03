use std::{
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    ops::{Index, IndexMut},
    path::Path,
    str::FromStr,
};

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Down,
    Up,
    Forward,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(direction: &str) -> Result<Self, Self::Err> {
        match direction {
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            "forward" => Ok(Direction::Forward),
            _ => Err(()),
        }
    }
}

impl Index<Direction> for [u32; 3] {
    type Output = u32;

    fn index(&self, direction: Direction) -> &u32 {
        &self[direction as usize]
    }
}

impl IndexMut<Direction> for [u32; 3] {
    fn index_mut(&mut self, direction: Direction) -> &mut u32 {
        &mut self[direction as usize]
    }
}

#[derive(Debug)]
pub struct Tuple<T, U>(pub T, pub U);

impl<U: FromStr, T: FromStr> FromStr for Tuple<T, U>
where
    T::Err: fmt::Debug,
    U::Err: fmt::Debug,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<&str> = s.split_whitespace().collect();
        Ok(Tuple(
            vec[0].parse::<T>().unwrap(),
            vec[1].parse::<U>().unwrap(),
        ))
    }
}

pub type Command = Tuple<Direction, u32>;

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Command({:?}, {})", self.0, self.1)
    }
}

pub fn read<T: FromStr>(filename: impl AsRef<Path>) -> Vec<T>
where
    <T as FromStr>::Err: fmt::Debug,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap().parse::<T>().unwrap())
        .collect()
}
