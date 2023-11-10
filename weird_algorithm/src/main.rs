/* Consider an algorithm that takes as input a positive integer n.
If n is even, the algorithm divides it by two, and if n is odd, the algorithm multiplies it by three and adds one.
The algorithm repeats this, until n is one. */

use std::io;

fn main() {
    // define variables for string io
    let mut input_var = String::new();

    // read input to string, and specify error code
    io::stdin()
        .read_line(&mut input_var)
        .expect("failed to readline");

    let mut x: i64 = input_var.trim().parse().expect("Input not an integer");

    println!("{}", x);

    while x > 1 {
        if x % 2 == 0 {
            x = x / 2
        } else {
            x = x * 3 + 1
        };
        println!("{}", x)
    }
}
