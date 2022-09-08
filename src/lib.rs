//!

#![allow(unused)]

use std::usize;
use rand::{self, seq::index, Rng};

macro_rules! rand_range {
    ($min:expr, $max:expr) => {{
        let mut rng = rand::thread_rng();
        // println!("max: {}, min: {}", $max, $min);
        rng.gen::<usize>() % ($max - $min + 1) + $min
    }};
    ($max:expr) => {{
        let mut rng = rand::thread_rng();
        rng.gen::<usize>() % $max
    }};
}

#[derive(Debug)]
struct SubArr<T> {
    arr: Vec<T>,
    idx: Vec<usize>,
    len: usize,
}

pub fn stupid_sort(arr: &mut Vec<u32>, output: bool) {
    if output {println!("calling: stupid_sort({:?})", arr)};
    if arr.len() == 2 {
        if !check(arr) {
            arr.swap(0, 1);
        }
    } else if arr.len() > 2 {
        while !check(&arr) {
            let n = rand_range!(2, arr.len() - 1);
            let mut subarrs = split_into_subarrs(n, arr.len());
            let idxes = rand_index(arr.len());

            let mut count = 0;
            for subarr in &mut subarrs {
                for i in 0..subarr.len {
                    subarr.idx.push(idxes[count + i]);
                    subarr.arr.push(arr[idxes[count + i]]);
                }
                count += subarr.len;
            }
            // println!("{:?}", subarrs);

            for subarr in &mut subarrs {
                stupid_sort(&mut subarr.arr, output);
                for i in 0..subarr.len {
                    arr[subarr.idx[i]] = subarr.arr[i];
                }
            }
        }
    }
    // println!("arr: {:?}\n", arr);
}

fn split_into_subarrs(n: usize, length: usize) -> Vec<SubArr<u32>> {
    let mut subarrs: Vec<SubArr<u32>> = Vec::new();
    let mut count = 0;

    for i in 0..n - 1 {
        let len = rand_range!(1, length - (n - i) - count + 1);
        count += len;
        subarrs.push(SubArr::new(len));
    }
    subarrs.push(SubArr::new(length - count));
    subarrs
}

fn check<T: PartialOrd>(arr: &Vec<T>) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}

fn rand_index(max: usize) -> Vec<usize> {
    let mut arr: Vec<usize> = (0..max).collect();
    for _ in 0..2 * max {
        let idx1 = rand_range!(max);
        let idx2 = rand_range!(max);
        arr.swap(idx1, idx2);
    }
    arr
}

impl<T> SubArr<T> {
    fn new(len: usize) -> SubArr<T> {
        SubArr {
            arr: Vec::new(),
            idx: Vec::new(),
            len: len,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check;

    #[test]
    fn check_test() {
        assert_eq!(false, check(&[1, -1, 3, 4, 5, 6, 7, 8].to_vec()))
    }
}
