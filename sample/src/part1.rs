use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

/*
use std::collections::HashSet;
*/

pub fn get_result(){
    match find_solution() {
        Ok(num) => println!("Part 1 answer: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn find_solution() -> io::Result<i32> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);
    
    for line in f.lines(){
        let string = line.unwrap();
    }
     
    return Ok(0);
}
