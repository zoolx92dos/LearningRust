#![allow(unused)]

// use core::panicking::panic;
// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;
use std::collections::HashMap;
use std::ops::Add;

// Iterators

fn main() {
    let arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }

    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());
}
