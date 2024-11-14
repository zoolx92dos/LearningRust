#![allow(unused)]

use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14159265;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");

    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
