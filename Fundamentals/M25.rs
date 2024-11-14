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
    const PI: f32 = 3.141592;

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    }

    struct Circle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            Rectangle { length, width }
        }
        fn area(&self) -> f32 {
            self.length * self.width
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            Circle { length, width }
        }
        fn area(&self) -> f32 {
            (self.length / 2.0).powf(2.0) * PI
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area: {}", rec.area());
    println!("Circle Area: {}", circ.area());
}
