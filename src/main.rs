
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
use std::collections::HashMap;
use solutions::GC::{GC};

fn main() {
    // some file handling
    let file_path = env::args().nth(1)
        .expect("Please provide a file path");
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let gc_dict: HashMap<String, f32> = GC(&contents);
    for (key, value) in gc_dict.iter() {
        println!("{} \n{}", key, value);
    }
}
