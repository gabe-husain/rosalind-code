
mod solutions;
// Some basic imports
use std::env;
use std::fs;


//Problem 1
//use solutions::DNA::{count_nucleotide, count_nucleotide_onepass};
//Problem 2
//use solutions::RNA::{transcribe};
//Problem 3
//use solutions::REVC::{complement};
//Problem 4
//use std::collections::HashMap;
//use solutions::GC::{GC};
//Problem 5
use solutions::SUBS::{SUBS};
// Problem 6


fn main() {
    // some file handling
    let file_path = env::args().nth(1)
        .expect("Please provide a file path");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let positions: Vec<usize> = SUBS(&contents);
    println!("{}", positions.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
