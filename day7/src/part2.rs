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

    let mut bags: HashMap<String, Vec<(String,i32)>> = HashMap::new();

    for line in f.lines(){
        /*
            line is of form:
            mirrored beige bags contain 1 drab brown bag, 3 dotted crimson bags.
        */
        let string = line.unwrap();
        let split_string: Vec<&str> = string.split(",").collect();
        let mut inner_bags: Vec<(String,i32)> = Vec::new();

        //grabs og bag
        let split_1: Vec<&str> = split_string[0].split(" bags contain ").collect();
        let bag = split_1[0].to_string(); //ex: mirrored beige

        //sees if bag cannot have inner bags
        if string.contains("no other bags"){
            bags.insert(bag, inner_bags);
            continue;
        }

        //grabs first inner bag
        let first_inner_string = &split_1[1][2..];
        let split_1_2: Vec<&str> = first_inner_string.split(" bag").collect();
        let first_inner_bag = split_1_2[0].to_string();
        let first_count = split_1[1][0..1].parse::<i32>().unwrap();
        inner_bags.push((first_inner_bag, first_count));

        //loops through rest of inner bags
        for i in 1..split_string.len(){
            let a_string = &split_string[i][3..];
            let split_i_2: Vec<&str> = a_string.split(" bag").collect();
            let inner_bag = split_i_2[0].to_string();
            let inner_bag_count = split_string[i][1..2].parse::<i32>().unwrap();

            inner_bags.push((inner_bag, inner_bag_count));
        }

        bags.insert(bag, inner_bags);
    }

    let count = count_bags(&bags, "shiny gold".to_string());

    //accounts for "gold bag" itself, we must not include that in the count
    return Ok(count - 1);
}

fn count_bags(bags: &HashMap<String, Vec<(String,i32)>>, bag: String) -> i32{
    let inner_bags: Vec<(String,i32)> = bags.get(&bag).unwrap().to_vec();
    if inner_bags.is_empty(){
        return 1;
    }

    let mut count = 0;
    for (inner_bag, inner_count) in inner_bags.iter(){
        count += count_bags(&bags, inner_bag.to_string()) * inner_count;
    }

    //add 1 to include itself
    return count + 1;
}