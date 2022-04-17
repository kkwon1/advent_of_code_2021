// Day 1 Part 1
/*
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<u32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("could not parse line"))
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

fn main() {
    let lines: Vec<u32> = lines_from_file("./day1.txt");
    let mut count = 0;
    for (i, _) in lines.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if lines[i - 1] < lines[i] {
            count += 1;
        }
    }
    println!("{}", count);
}
*/

use crate::helpers::file_reader;

pub fn get_counts(filename: &str) -> u32 {
    println!("{}", filename);
    let lines: Vec<u32> = file_reader::lines_from_file(filename);
    let mut count = 0;
    let mut prev_sum = lines[0] + lines[1] + lines[2];
    let mut next_sum = 0;
    for (i, _) in lines.iter().enumerate() {
        if i == 0 || i >= 1998 {
            continue;
        }
        next_sum = lines[i] + lines[i + 1] + lines[i + 2];

        if prev_sum < next_sum {
            count += 1;
        }

        prev_sum = next_sum;
    }

    count
}
