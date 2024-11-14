// Getting the sum of list

#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += val;
    }
    sum
}

fn main() {
    let (val_1, val_2) = get_2(3);
    println!("Nums : {} {}", val_1, val_2);

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));
}
