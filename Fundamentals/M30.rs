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
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    
    println!("samp1 = {}", samp1);
    
    samp1 = 10;
    println!("samp1 = {}", samp1);
}
