use std::{
    fs::File,
    io::{self},
    path::Path,
};

use io::{BufRead, BufReader};

pub fn read_lines<P>(path: P) -> Option<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(path).ok()?;
    let lines = BufReader::new(file).lines().filter_map(Result::ok);
    Some(lines)
}
