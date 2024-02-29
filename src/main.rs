#![forbid(unsafe_code)]
#![feature(lazy_cell)]

mod data;
mod register;

mod parser;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "./data.csv";
    let file_handle = File::open(filename).unwrap();
    let lines = BufReader::new(file_handle).lines();

    Ok(())
}
