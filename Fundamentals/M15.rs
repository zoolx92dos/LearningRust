#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    let int1_u8: u8 = 5;
    let int2_u8: u8 = 4;

    let int3_u32: u32 = (int1_u8 as u32) + (int2_u8 as u32);
}
