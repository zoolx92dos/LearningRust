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

// Closures

fn main() {
    // let var_name = |parameters| -> return_type {BODY}
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote : {}", can_vote(8));
}
