/*
Your task is to calculate the number of possible bit strings of length n.
*/

use std::io;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Could not read input");

    let n: i64 = inp.trim().parse().expect("Input not an integer");
    
    let m: i64 = 1000000007;
    let mut result: i64 = 1;
    let mut i = 1;
    while i <= n {
        i += 1;
        result = (2 * result) % m
    }
    println!("{}", result);
}