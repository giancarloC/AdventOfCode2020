use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn get_result(){
    match find_max_seat() {
        Ok(num) => println!("Highest seat ID is: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn find_max_seat() -> io::Result<i32> {
    let f = File::open("./input/input.txt")?;
    let f = BufReader::new(f);

    let mut highest_seat = 0;

    //finds max seat
    for line in f.lines(){
        let string = line.unwrap();
        let id = grab_seat_id(&string); //parses string

        if id > highest_seat{
            highest_seat = id;
        }
    }
  
    return Ok(highest_seat);
}

fn grab_seat_id(string: &str) -> i32 {
    let first_part = &string[..7];
    let second_part = &string[7..];

    //finds row
    let mut low = 0;
    let mut high = 128;
    for c in first_part.chars(){
        if c == 'F'{
            high -= (high-low)/2;
        }
        else if c == 'B'{
            low += (high-low)/2;
        }
    }
    let row = low;

    //finds column
    low = 0;
    high = 8;
    for c in second_part.chars(){
        if c == 'L'{
            high -= (high-low)/2;
        }
        else if c == 'R'{
            low += (high-low)/2;
        }
    }
    let column = low;

    //finds seat ID
    let seat_id = row*8 + column;
    return seat_id;
}