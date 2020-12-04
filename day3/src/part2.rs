use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

pub fn get_result(){
    //reads file and processes vector
    match read_file() {
        Ok(a_vec) => find_multiple_trees(a_vec),
        Err(_e) => println!("Incorrect Input"),
    }
}

fn find_multiple_trees(forest: Vec<Vec<char>>){
    let tree1 = find_trees(&forest, 1, 1) as i64;
    let tree2 = find_trees(&forest, 1, 3) as i64;
    let tree3 = find_trees(&forest, 1, 5) as i64;
    let tree4 = find_trees(&forest, 1, 7) as i64;
    let tree5 = find_trees(&forest, 2, 1) as i64;

    println!("tree2 {}", tree3);

    let answer = tree1*tree2*tree3*tree4*tree5;

    println!("Amount of trees: {}", answer);
}

fn find_trees(forest: &[Vec<char>], to_down: usize, to_right: usize) -> i32{
    let mut count = 0; //amount of trees found
    let mut v_index = 0; //vertical index
    let mut h_index = 0; //horizontal index

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

    return count;
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
