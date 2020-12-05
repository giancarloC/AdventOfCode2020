use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn get_result(){
    match find_min_max_seats() {
        Ok(num) => println!("My seat ID is: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }
}

//counts number of passports
fn find_min_max_seats() -> io::Result<i32> {
    let f = File::open("./input/input.txt")?;
    let f = BufReader::new(f);

    let mut seat_ids: HashSet<i32> = HashSet::new();
    let mut highest_seat = 0;
    let mut lowest_seat = i32::MAX;

    //finds max and min seat ids, places all in set
    for line in f.lines(){
        let string = line.unwrap();
        let id = grab_seat_id(&string); //parses string

        seat_ids.insert(id);

        if id > highest_seat{
            highest_seat = id;
        }
        if id < lowest_seat{
            lowest_seat = id;
        }
    }

    //finds missing id
    let mut my_id = 0;
    for id in lowest_seat..highest_seat{
        if !seat_ids.contains(&id){
            my_id = id;
        }
    }
  
    return Ok(my_id);
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