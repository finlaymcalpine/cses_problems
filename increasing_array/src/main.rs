/* 
You are given an array of n integers. 
You want to modify the array so that it is increasing, 
i.e., every element is at least as large as the previous element.
On each move, you may increase the value of any element by one. 
What is the minimum number of moves required?
*/

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    let _n: usize = input.trim().parse().expect("Could not parse integer");

    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Could not read input");
    let int_vec: Vec<i64> = input_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut moves = 0;
    let mut last_val = int_vec[0];
    for i in int_vec.iter() {
        if i < &last_val {
            moves += last_val - i;
            last_val = i + (last_val - i)
        }
        else {
            last_val = *i
        }
    }
    
    println!("{}", moves);
}
