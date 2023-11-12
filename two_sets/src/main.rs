/*
Your task is to divide the numbers 1,2,\ldots,n into two sets of equal sum.
Print YES if division is possible, print NO otherwise.
After, if YES, then print number of elements in the first set and then the set,
and likewise for the second set.
*/

use std::collections::HashSet;
use std::io;
// would use itertools, but won't pass test cases
//use itertools::Itertools;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not parse input");
    let n: usize = input.trim().parse().expect("Input not an integer");

    // trying out non N*(N+1) / 2 way to find sum of all numbers 1 to N
    // summing integers from 1 through N. if divisible by 2, then the list can be composed into two sets
    if (1..=n).sum::<usize>() % 2 == 0 {
        println!("YES");

        let sigma_n: usize = (n * (n + 1)) / 2;
        let sigma_n_2: usize = &sigma_n / 2;

        // we will start at n and collect elements backwards until their sum would be greater than sigma_n/2.
        // then, we find the element needed to make up to sigma_n/2, and collect all other elements into the other set.
        let mut first_set: HashSet<usize> = HashSet::new();
        let mut second_set: HashSet<usize> = HashSet::new();

        let mut sum: usize = 0;
        let mut omitted: usize = 0;
        let mut last: usize = 0;

        for i in (1..=n).rev() {
            sum += i;
            if sum <= sigma_n_2 {
                first_set.insert(i);
            } else {
                sum -= i;
                // hacky fix for case of n=3, where inserting 0 into first_set
                if (sigma_n_2 - sum) != 0 {
                first_set.insert(sigma_n_2 - sum);
                }
                omitted = sigma_n_2 - sum;
                last = i;
                break;
            }
        }

        for i in (1..=last).rev() {
            if i != omitted {
                second_set.insert(i);
            }
        }

        println!("{}", first_set.len());
        println!("{}", first_set.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        //println!("{}", first_set.iter().join(" "));
        println!("{}", second_set.len());
        println!("{}", second_set.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        //println!("{}", second_set.iter().join(" "));
    } else {
        println!("NO");
    }
}
