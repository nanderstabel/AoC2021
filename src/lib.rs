use std::{
    fmt::Debug,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
pub struct Tuple<T, U>(T, U);

impl<U: FromStr, T: FromStr> FromStr for Tuple<T, U>
where
    T::Err: Debug,
    U::Err: Debug,
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

pub fn read<T: FromStr>(filename: impl AsRef<Path>) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap().parse::<T>().unwrap())
        .collect()
}
