#![allow(unused)]



use std::{io, vec};

use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    let mut i1 = String::new();
    let mut i2 = String::new();

    io::stdin().read_line(&mut i1).expect("failed to read amount");
    io::stdin().read_line(&mut i2).expect("failed to read numbers");

    let mut first_number: usize = i1.trim().parse().expect("didnt work, son");

    if first_number % 2 == 1{
        first_number += 1
    }
    let mut numbers: Vec<u32> = i2 
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("didn't work!"))
        .collect();

    numbers.sort();
    numbers.reverse();

    let mut _count = 0;
    for _index in 0..(first_number/2){
        _count += numbers[_index]

    }
    println!("{:?}", _count)

}
