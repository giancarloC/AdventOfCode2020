use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn get_result(){
    //reads file and processes vector
    match read_file() {
        Ok(a_vec) => find_trees(a_vec),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn find_trees(forest: Vec<Vec<char>>){
    let mut count = 0; //amount of trees found
    let mut v_index = 0; //vertical index
    let mut h_index = 0; //horizontal index

    let to_down = 1; //spaces moved down
    let to_right = 3; //spaces moved to the right
    let tree = '#';
    let max_v = forest.len();
    let max_h = forest[0].len();

    v_index += to_down;
    while v_index < max_v{
        h_index += to_right;
        if h_index >= max_h{
            h_index -= max_h;
        }
        
        if forest[v_index][h_index] == tree{
            count += 1;
        }

        v_index += to_down;
    }

    println!("Amount of trees: {}", count);
}

//returns vector from given string representing files
fn read_file() -> io::Result<Vec<Vec<char>>> {
    let f = File::open("./input/input.txt")?;
    let mut f = BufReader::new(f);

    let mut lines_of_chars: Vec<Vec<char>> = Vec::new();
    for line in f.lines(){
        let string = line.unwrap();
        lines_of_chars.push(string.chars().collect());
    }
  
    return Ok(lines_of_chars);
}
