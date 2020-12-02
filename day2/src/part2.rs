use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn get_result() -> io::Result<()>{
    let f = File::open("./input/input.txt")?;
    let f = BufReader::new(f);
    let mut valid_pass = 0;

    //loops through file, puts numbers in set
    for line in f.lines(){
        //splits line to get appropriate values
        let line_str = line.unwrap();
        let line_split = line_str.split(": ");
        let v: Vec<&str> = line_split.collect();
        let first_part: Vec<&str> = v[0].split(" ").collect();
        let first_nums: Vec<&str>  = first_part[0].split("-").collect();

        //grabs values needed
        let low = first_nums[0].parse::<usize>().unwrap();
        let high = first_nums[1].parse::<usize>().unwrap();
        let letter = first_part[1];
        let string = v[1];
        let string_bytes = string.as_bytes();
        let first_string = (string_bytes[low-1] as char).to_string();
        let second_string = (string_bytes[high-1] as char).to_string();
        
        //checks if password is valid
        if (letter == first_string) ^ (letter == second_string){
            valid_pass += 1;
        }
    }
    
    println!("Number of valid passwords: {}", valid_pass);

    Ok(())
}