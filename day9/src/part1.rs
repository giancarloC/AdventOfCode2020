use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn get_result(){
    match find_solution() {
        Ok(num) => println!("Invalid number is: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn find_solution() -> io::Result<i32> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);

    let preamble = 25;
    let mut numbers: HashSet<i32> = HashSet::new();
    let mut number_vec: Vec<i32> = Vec::new();
    let mut answer = 0;
    let mut lines = f.lines();

    //looks through preamble
    for _i in 0..preamble{
        let line = lines.next();
        let string = line.unwrap();
        match string {
            Ok(string) => {
                let num: i32 = string.parse().unwrap();
                numbers.insert(num);
                number_vec.push(num);
            },
            Err(_e) => println!("oops")
        }
    }
    
    let mut good_num = false;
    for line in lines{
        let string = line.unwrap();
        let num: i32 = string.parse().unwrap();

        //checks if it's sum
        for check in numbers.iter(){
            if numbers.contains(&(num-check)) && num != check*2{
                good_num = true;
            }
        }

        //if bad number, that is the answer
        if !good_num{
            answer = num;
            break;
        }

        let last_num = number_vec.remove(0);
        numbers.remove(&last_num);

        numbers.insert(num);
        number_vec.push(num);

        good_num = false;
    }
     

    return Ok(answer);
}
