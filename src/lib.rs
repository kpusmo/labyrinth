use std::convert::TryFrom;
use std::error::Error;
use std::fs;

use crate::labyrinth::{Labyrinth, START_POINT, State};

mod labyrinth;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn solve_labyrinth(input_path: &str) -> Result<u32> {
    let mut labyrinth = Labyrinth::from_file(input_path)?;
    labyrinth.find_way_out_from(&START_POINT, State::empty())
        .ok_or(Box::try_from("No way out found")?)
}

pub fn solve_binary(input_path: &str) -> Result<Vec<i32>> {
    // filter_map will filter the first line out
    let result = fs::read_to_string(input_path)?
        .lines()
        .filter_map(|line| i32::from_str_radix(line, 2).ok())
        .collect();
    Ok(result)
}