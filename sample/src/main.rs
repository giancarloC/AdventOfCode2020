#![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

/*
use std::collections::HashSet;
*/

fn main(){
    match part1() {
        Ok(num) => println!("Part 1 answer: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }

    match part2() {
        Ok(num) => println!("Part 2 answer: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn part1() -> io::Result<i32> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);
    
    for line in f.lines(){
        let string = line.unwrap();
    }
     
    return Ok(0);
}

fn part2() -> io::Result<i32> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);
    
    for line in f.lines(){
        let string = line.unwrap();
    }
     
    return Ok(0);
}
