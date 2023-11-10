/*
You are given all numbers between 1,2,...,n except one. They will not necessarily be in order.
Your task is to find the missing number.
*/

use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: i64 = input.trim().parse().expect("Input not an integer");

    let sigma = (n * (n + 1)) / 2;

    let mut s = 0;

    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read input");
    let numbers: Vec<i64> = input_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for number in numbers.iter() {
        s += number
    }

    let missing_int = sigma - s;

    println!("{}", missing_int);
}
