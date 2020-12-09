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

    let mut instructions:Vec<(String, i32)> = Vec::new();

    //stores instructions
    for line in f.lines(){
        let string = line.unwrap();
        let split: Vec<&str> = string.split(" ").collect();

        let instruction = split[0].to_string();
        let num: i32 = split[1].parse().unwrap();

        instructions.push((instruction, num));
    }

    let length = instructions.len();
    for i in 0..length{
        let (get_inst, get_num) = &instructions[i];
        let num: i32 = *get_num;
        let instruction = get_inst.clone();
        
        //changes an instruction and tests
        if instruction == "nop"{
            std::mem::replace(&mut instructions[i], ("jmp".to_string(), num));
        }
        else if instruction == "jmp"{
            std::mem::replace(&mut instructions[i], ("nop".to_string(), num));
        }
        else{
            continue;
        }

        //tests if it works
        let (does_not_repeat, accumulator) = run_game(&instructions);
        if does_not_repeat{
            return Ok(accumulator);
        }
        else{ //replaces back
            std::mem::replace(&mut instructions[i], (instruction, num));
        }
    }

    return Ok(0);
}

fn run_game(instructions: &Vec<(String, i32)>) -> (bool, i32){
    let mut visited: HashSet<usize> = HashSet::new();
    let mut curr_ind: usize = 0;
    let mut accumulator: i32 = 0;

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
            else if *num < 0{
                for _i in *num..0{
                    curr_ind -= 1;
                }
            }
        }

        //checks if we have reached the end
        if curr_ind >= instructions.len(){
            return (true, accumulator);
        }
    }

    //has repeated and thus, we return false
    return (false, accumulator);
}
