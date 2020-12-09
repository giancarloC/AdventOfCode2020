use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

pub fn get_result(){
    match find_solution() {
        Ok(num) => println!("Value of accumulator is: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn find_solution() -> io::Result<i32> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);

    let mut accumulator = 0;
    let mut instructions:Vec<(String, i32)> = Vec::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut curr_ind: usize = 0;

    //stores instructions
    for line in f.lines(){
        let string = line.unwrap();
        let split: Vec<&str> = string.split(" ").collect();

        let instruction = split[0].to_string();
        let num: i32 = split[1].parse().unwrap();

        instructions.push((instruction, num));
    }

    //works through instructions
    let mut repeated = false;
    while !repeated{
        if visited.contains(&curr_ind){
            repeated = true;
            continue;
        }
        visited.insert(curr_ind);

        let (instruction, num) = &instructions[curr_ind];
        if instruction == "nop"{
            curr_ind += 1;
        }
        else if instruction == "acc"{
            curr_ind += 1;
            accumulator += num;
        }
        else if instruction == "jmp"{ //annoying usize manipulation
            if *num > 0{
                for _i in 0..*num{
                    curr_ind += 1;
                }
            }
            else if *num == 0{
                continue;
            }
            else{
                for _i in *num..0{
                    curr_ind -= 1;
                }
            }
        }
    }

    return Ok(accumulator);
}
