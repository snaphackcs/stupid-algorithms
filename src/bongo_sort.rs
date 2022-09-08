//!

#[allow(unused)]
use rand::{self, Rng};

macro_rules! rand_range {
    ($max:expr) => {{
        let mut rng = rand::thread_rng();
        rng.gen::<usize>() % $max
    }};
}

#[allow(unused)]
pub fn bongo_sort(arr: &mut Vec<u32>) {
    println!("calling: bongo_sort({:?})", arr);
    while !check(&arr) {
        for _ in 0..arr.len()*2 {
            let idx1 = rand_range!(arr.len());
            let idx2 = rand_range!(arr.len());
            arr.swap(idx1, idx2);
        }
        println!("arr: {:?}", arr);
    }
}

#[allow(unused)]
fn check<T: PartialOrd>(arr: &Vec<T>) -> bool {
    for i in 1..arr.len() {
        if arr[i] < arr[i - 1] {
            return false;
        }
    }
    true
}