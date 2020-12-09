use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn get_result(){
    match find_invalid_number() {
        Ok(num) => {
            match find_weakness(num as i64) {
                Ok(weakness) => println!("Encryption weakness is: {}", weakness),
                Err(_e2) => println!("big oof2")
            }
        },
        Err(_e) => println!("big oof")
    }
}

fn find_weakness(invalid: i64) -> io::Result<i64> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);

    let mut numbers: Vec<i64> = Vec::new();
    for line in f.lines(){
        let string = line.unwrap();
        let to_insert: i64 = string.parse().unwrap();
        numbers.push(to_insert);
    }

    let mut start: usize = 0;
    let mut end: usize = 1;
    let limit = numbers.len();
    let mut sum = numbers[0] + numbers[1];

    while end < limit{
        if sum < invalid{
            end += 1;
            sum += numbers[end];
        }
        else if sum > invalid{
            sum -= numbers[start];
            start += 1;
        }
        else{
            break;
        }
    }

    let mut small = i64::MAX;
    let mut big = i64::MIN;

    for i in start..end+1{
        let num = numbers[i];
        if num > big{
            big = num;
        }
        if num < small{
            small = num;
        }
    }

    let weakness = big + small;

    return Ok(weakness);
}

fn find_invalid_number() -> io::Result<i32> {
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
