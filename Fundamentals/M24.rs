#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

use std::collections::HashMap;
use std::ops::Add;

// Using Structs & Traits

fn main() {
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }

    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
}
