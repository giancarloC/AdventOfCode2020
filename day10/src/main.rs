//#![allow(unused)]
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main(){
    match answer() {
        Ok(()) => println!("Success"),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn answer() -> io::Result<()> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);

    let mut jolts: Vec<i32> = Vec::new();
    let mut set: HashMap<i32, i64> = HashMap::new();
    
    for line in f.lines(){
        let string = line.unwrap();
        let num: i32 = string.parse().unwrap();

        jolts.push(num);
        set.insert(num, 0);
    }
    jolts.push(0);
    set.insert(0,1);
    jolts.sort();
    let end = jolts.last().unwrap() + 3;
    jolts.push(end);
    set.insert(end, 0);

    let mut diff1 = 0; //lists how many jolts are 1 apart
    let mut diff3 = 0; //same but 3 apart
    let mut last = 0;

    for i in &jolts{
        if i - last == 3{
            diff3 += 1;
        }
        else if i - last == 1{
            diff1 += 1;
        }

        last = *i;
    }

    let answer = diff1 * diff3;
    println!("Part 1 Answer: {}", answer);

    /*beginning of part 2*/

    for val in &jolts{
        let curr_count = *set.get(val).unwrap();

        if set.contains_key(&(val+1)){
            set.insert(val+1, *set.get(&(val+1)).unwrap() + curr_count);
        }
        if set.contains_key(&(val+2)){
            set.insert(val+2, *set.get(&(val+2)).unwrap() + curr_count);
        }
        if set.contains_key(&(val+3)){
            set.insert(val+3, *set.get(&(val+3)).unwrap() + curr_count);
        }

    }

    let answer_2 = set.get(&end).unwrap();
    println!("Part 2 Answer: {}", answer_2);

    return Ok(());
}
