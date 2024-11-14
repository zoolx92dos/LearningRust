#![allow(unused)]

use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    let is_true = true;
    let my_grade = 'A';

    // precision of data types
    let num_1: f32 = 1.111111111111;
    println!("f32: {}", num_1 + 0.111111111111);

    let num_2: f64 = 1.111111111111;
    println!("f64: {}", num_2 + 0.111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
}
