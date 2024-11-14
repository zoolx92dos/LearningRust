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

// Modules
mod restaurant;
use crate::restaurant::order_food;

fn main() {
    // Crates: Modules that produce a library or executable
    // Modules: Organize and handle privacy
    // Packages: Build, test and share crates
    // Paths: A way of naming an item such as a struct, function

    order_food();
}
