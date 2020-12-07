use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

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
    let mut group_count = 0;
    let mut questions: HashMap<char, i32> = HashMap::new();

    for line in f.lines(){
        let string = line.unwrap();

        if string == ""{
            //loops through map
            for (_key, val) in questions.iter(){
                if val == &group_count{
                    count += 1;
                }
            }

            questions.clear();
            group_count = 0;
        }
        
        else{
            group_count += 1;
            
            for c in string.chars(){
                if questions.contains_key(&c){
                    questions.insert(c, questions.get(&c).unwrap() + 1);
                }
                else{
                    questions.insert(c, 1);
                }
            }    
        }
    }

    //loops through last one
    for (_key, val) in questions.iter(){
        if val == &group_count{
            count += 1;
        }
    }
  
    return Ok(count);
}