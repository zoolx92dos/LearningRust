#![allow(unused)]

// use core::panicking::panic;
// use std::intrinsics::prefetch_read_instruction;
use std::io;
use rand::{Rng, Error};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use core::result;
use std::collections::HashMap;
use std::ops::Add;

use std::thread;
use std::time::Duration;

// Thread, Concurrency & Smart Pointers

fn main() {
    pub struct Bank {
        balance: f32,
    }

    fn withdraw(the_bank: &mut Bank, amt: f32) {
        the_bank.balance -= amt;
    }

    let mut bank = Bank { balance: 100.0 };
    withdraw(&mut bank, 5.00);
    println!("Balance : {}", bank.balance);

    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 5.00);
    }

    thread::spawn(|| {
        customer(&mut bank)
    })
    .join()
    .unwrap();
}
