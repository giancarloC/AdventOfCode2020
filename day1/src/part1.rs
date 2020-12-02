use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn get_result() -> io::Result<()>{
    let mut expenses: HashSet<i32> = HashSet::new();
    let f = File::open("./input/input.txt")?;
    let f = BufReader::new(f);

    //loops through file, puts numbers in set
    for line in f.lines(){
        let str_num = line.unwrap();
        let num = str_num.parse::<i32>().unwrap();
        
        //checks if complement has been found
        let complement = 2020-num;
        if expenses.contains(&complement){
            let answer = complement*num;
            println!("Part 1 answer is: {}", answer);
            break;
        }

        expenses.insert(num);
    }

    Ok(())
}