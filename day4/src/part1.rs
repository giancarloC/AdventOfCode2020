use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn get_result(){
    match count_passports() {
        Ok(num) => print_answer(num),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn print_answer(answer: i32){
    println!("There are {} passports", answer);
}

//counts number of passports
fn count_passports() -> io::Result<i32> {
    let f = File::open("./input/input.txt")?;
    let f = BufReader::new(f);

    let mut valid_count = 0;

    //flags for items in passport (we don't care about cid)
    let (mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl, mut pid) = (false, false, false, false, false, false, false);

    for line in f.lines(){
        let string = String::from(line.unwrap());

        //checks if line is empty
        if string.is_empty(){
            //adds to count if prev passport is valid
            if byr && iyr && eyr && hgt && hcl && ecl && pid{
                valid_count += 1;
            }

            byr = false;
            iyr = false;
            eyr = false;
            hgt = false;
            hcl = false;
            ecl = false;
            pid = false;
        }

        //processes passport
        else{
            if string.contains("byr:"){
                byr = true;
            }
            if string.contains("iyr:"){
                iyr = true;
            }
            if string.contains("eyr:"){
                eyr = true;
            }
            if string.contains("hgt:"){
                hgt = true;
            }
            if string.contains("hcl:"){
                hcl = true;
            }
            if string.contains("ecl:"){
                ecl = true;
            }
            if string.contains("pid:"){
                pid = true;
            }
        }
    }

    //checks last passport
    if byr && iyr && eyr && hgt && hcl && ecl && pid{
        valid_count += 1;
    }
  
    return Ok(valid_count);
}
