#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);

    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Task");
    } else if (age == 21) || (age == 50) {
        println!("Important Task 2");
    } else if age >= 65 {
        println!("Important Task 3");
    } else {
        println!("Not an Important Task");
    }
}
