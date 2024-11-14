#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

use std::ops::Add;

// Stack: Stores values in a LIFO format
// Data on the stack must have a defined fixed size

/*
Heap: When putting data on the heap you request a certain amount of space.
The OS finds space available and returns an address for that space called a pointer.
Reference to the space and address -> pointer
*/

// Automatic deallocation of resources
// Ownership

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message : {}", name);
}

fn main() {
    let str1 = String::from("World");
    let str2 = str1.clone();
    println!("Hello {}", str1);

    print_str(str1);

    let str3 = print_return_str(str2);
    println!("str3 = {}", str3);
}
