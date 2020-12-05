use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
extern crate regex;
use regex::Regex;

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

    //regex used to find in text
    let byr_reg = Regex::new(r"byr:(19[2-9][0-9]|200[0-2])").unwrap();
    let iyr_reg = Regex::new(r"iyr:20(1[0-9]|20)").unwrap();
    let eyr_reg = Regex::new(r"eyr:20(2[0-9]|30)").unwrap();
    let hgt_reg = Regex::new(r"hgt:(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)").unwrap();
    let hcl_reg = Regex::new(r"hcl:#[0-9a-f]{6}").unwrap();
    let ecl_reg = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let pid_reg = Regex::new(r"pid:[0-9]{9}").unwrap();
    let pid_wrong = Regex::new(r"pid:[0-9]{10}").unwrap();

    for line in f.lines(){
        let string = line.unwrap();

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
            if byr_reg.is_match(&string){
                byr = true;
            }
            if iyr_reg.is_match(&string){
                iyr = true;
            }
            if eyr_reg.is_match(&string){
                eyr = true;
            }
            if hgt_reg.is_match(&string){
                hgt = true;
            }
            if hcl_reg.is_match(&string){
                hcl = true;
            }
            if ecl_reg.is_match(&string){
                ecl = true;
            }
            if pid_reg.is_match(&string) && !pid_wrong.is_match(&string){
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
