use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn get_result(){
    match find_sum_count() {
        Ok(num) => println!("Sum of the counts are: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn find_sum_count() -> io::Result<i32> {
    let f = File::open("./input/input.txt")?;
    let f = BufReader::new(f);

    let mut count = 0;
    let mut questions: HashSet<char> = HashSet::new();

    for line in f.lines(){
        let string = line.unwrap();

        if string == ""{
            count += questions.len() as i32;
            questions.clear();
        }
        
        else{
            for c in string.chars(){
                questions.insert(c);
            }
        }
    }

    //adds last one
    count += questions.len() as i32;
  
    return Ok(count);
}