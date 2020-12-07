use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub fn get_result(){
    match find_solution() {
        Ok(num) => println!("Sum of the counts are: {}", num),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn find_solution() -> io::Result<i32> {
    let f = File::open("./src/input.txt")?;
    let f = BufReader::new(f);

    let mut bags: HashMap<String, Vec<String>> = HashMap::new();

    for line in f.lines(){
        /*
            line is of form:
            mirrored beige bags contain 1 drab brown bag, 3 dotted crimson bags.
        */
        let string = line.unwrap();
        let split_string: Vec<&str> = string.split(",").collect();
        let mut inner_bags: Vec<String> = Vec::new();

        //grabs og bag
        let split_1: Vec<&str> = split_string[0].split(" bags ").collect();
        let bag = split_1[0].to_string(); //ex: mirrored beige

        //sees if bag cannot have inner bags
        if string.contains("no other bags"){
            bags.insert(bag, inner_bags);
            continue;
        }

        //grabs first inner bag
        let first_inner_string = &split_1[1][10..];
        let split_1_2: Vec<&str> = first_inner_string.split(" bag").collect();
        let first_inner_bag = split_1_2[0].to_string();
        inner_bags.push(first_inner_bag);

        //loops through rest of inner bags
        for i in 1..split_string.len(){
            let a_string = &split_string[i][3..];
            let split_i_2: Vec<&str> = a_string.split(" bag").collect();
            let inner_bag = split_i_2[0].to_string();

            inner_bags.push(inner_bag);
        }

        bags.insert(bag, inner_bags);
    }

    //loops to see which bags have at least one gold bag (can be more efficient w/ dynamic programming)
    let mut count = 0;
    for (key, _bag) in bags.iter(){
        if is_good_bag(&bags, key.to_string()){
            count += 1;
        }
    }

    //accounts for "gold bag" itself, we must not include that in the count
    return Ok(count-1);
}

fn is_good_bag(bags: &HashMap<String, Vec<String>>, bag: String) -> bool{
    if bag == "shiny gold"{
        return true;
    }

    //grabs inner_bag from map
    let inner_bags: Vec<String> = bags.get(&bag).unwrap().to_vec();
    if inner_bags.is_empty(){
        return false;
    }

    for inner_bag in inner_bags.iter(){
        if is_good_bag(&bags, inner_bag.to_string()){
            return true
        }
    }

    return false;
}