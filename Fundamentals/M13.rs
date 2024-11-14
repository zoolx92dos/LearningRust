#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
}
