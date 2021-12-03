use std::{
    fmt,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
pub struct Tuple<T, U>(T, U);

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

pub type Command = Tuple<String, u8>;

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Command({}, {})", self.0, self.1)
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
