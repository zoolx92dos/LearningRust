#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    // eliminating duplicates
    let st3 = String::from("sfsfdbdbfsfvdgbvdcszvfvdvf");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {}", st6.len());

    st5.clear();

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    for char in st8.bytes() {
        println!("{}", char);
    }
}
