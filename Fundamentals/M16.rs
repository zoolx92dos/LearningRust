#![allow(unused)]

// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;

fn main() {
    // enum - create custom data types
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    // functions for enumerated data types
    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Days = Days::Monday;
    match today {
        Days::Monday => println!("Everyone hates Monday"),
        Days::Tuesday => println!("Everyone hates Monday"),
        Days::Wednesday => println!("Everyone hates Monday"),
        Days::Thursday => println!("Everyone hates Monday"),
        Days::Friday => println!("Everyone hates Monday"),
        Days::Saturday => println!("Everyone hates Monday"),
        Days::Sunday => println!("Everyone hates Monday"),
    }

    println!("Is today the weekend? {}", today.is_weekend());
}
