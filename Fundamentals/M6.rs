#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    let mut my_age = 47;
    let can_vote = if my_age >= 19 {
        true
    } else {
        false
    };

    println!("Can vote : {}", can_vote);
}
