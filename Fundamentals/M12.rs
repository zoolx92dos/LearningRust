#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    let my_tuple: (u8, String, f64) = (47, "Alex".to_string(), 50000.00);

    println!("Name : {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}
