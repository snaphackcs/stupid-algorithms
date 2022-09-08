//! this file tests the final result of function stupid_sort
//! the `main` function calls function `stupid_sort` and prints the result.

use std::env::args;
#[allow(unused)]
use time::get_time;

use stupid_sort::stupid_sort;
// use bongo_sort::bongo_sort;

// mod bongo_sort;

fn main() {
    let mut output = false;
    let mut arr: Vec<u32> = args()
        .skip(1)
        .filter_map(|s| if s == "-o" { output = true;None } else { s.parse().ok() })
        .collect();

    let t1 = get_time();
    stupid_sort(&mut arr, output);
    let t2 = get_time();

    println!("{:?}, time: {}", arr, t2 - t1);
}
