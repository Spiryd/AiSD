use rand::prelude::*;
use rand_pcg::Pcg64;

#[derive(Debug)]
pub enum Order{
    Random,
    Sorted,
    Reverse
}

pub fn gen_list(n: u64, order: Order) -> Vec<u64>{
    let mut rng: Pcg64 = Pcg64::from_entropy();
    let mut vector: Vec<u64> = Vec::new();
    for _ in 0..n {
        vector.push(rng.gen_range(0..2*n));
    }
    match order {
        Order::Random => return vector,
        Order::Sorted => {
            vector.sort();
            return vector;
        },
        Order::Reverse => {
            vector.sort();
            vector.reverse();
            return vector;
        },
    }
}

pub fn print_to_std(list: Vec<u64>){
    for key in list {
        print!(" {key}")
    }
}

pub fn randomized_select(arr: &mut Vec<u64>, i: usize) -> u64 {
    _randomized_select(arr, 0, arr.len() - 1, i)
}

fn _randomized_select(arr: &mut Vec<u64>, left: usize, right: usize, i: usize) -> u64 {
    if left == right {
        return arr[left];
    }
    let pivot_index = randomized_partition(arr, left, right);
    let k = pivot_index - left + 1;
    if i == k {
        return arr[pivot_index];
    } else if i < k {
        return _randomized_select(arr, left, pivot_index - 1, i);
    } else {
        return _randomized_select(arr, pivot_index + 1, right, i - k);
    }
}

fn randomized_partition(arr: &mut Vec<u64>, left: usize, right: usize) -> usize {
    let mut rng = Pcg64::from_entropy();
    let pivot_index = rng.gen_range(left..=right);
    arr.swap(pivot_index, right);
    let mut i = left;
    for j in left..right {
        if arr[j] <= arr[right] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, right);
    return i;
}
