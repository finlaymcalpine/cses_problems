/*
You are given a DNA sequence: a string consisting of characters A, C, G, and T.
Your task is to find the longest repetition in the sequence.
This is a maximum-length substring containing only one type of character.
*/

use std::io;

fn main() {
    // take input string from stdin. don't need to convert to int later here, as string is desired
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    // wondering if there a cleaner way than below line to trim whitespace, without shadowing
    let input = input.trim();

    // create unsigned int variable to hold max repetition lenght observed. will be always > 0
    let mut max_length: u32 = 1;

    // read length of input string (note that len() method returns byte length, which works here
    // because we are only dealing with A, C, G, T in the input).
    // let _n: usize = input.len(); // Don't need this, because we just enumerate the iterable.

    // does this duplicate the input in an inefficient way? should we drop the input string here?
    let char_vec: Vec<char> = input.chars().collect();
    let mut runner: u32 = 1;

    for (i, c) in char_vec.iter().enumerate() {
        if i == 0 {
            // pass
        } else {
            if *c == char_vec[i - 1] { // need to deref c to get char, not &char. IS THIS A RISK???
                runner += 1;
                if runner > max_length {
                    max_length = runner
                }
            }
            else {
                if runner > max_length {
                    max_length = runner // since runner is > max_length, this is the new longest length
                }
                runner = 1 // reset runner to 1 for use by next letter
            }
        }
    }

    println!("{}", max_length);
}
