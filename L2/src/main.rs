use std::collections::VecDeque;

use rand::prelude::*;
use rand_pcg::Pcg64;

mod sort;

#[derive(Debug)]
enum Order{
    Random,
    Sorted,
    Reverse
}

fn gen_list(n: u64, order: Order) -> Vec<u64>{
    let mut rng:Pcg64 = Pcg64::from_entropy();
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

 pub fn insertion_sort(table: &mut Vec<u64>){
    for i in 1..table.len() {
        let mut j = i;
        while j > 0 && table[j] < table[j - 1] {
            table.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn merge_sort(table: &mut Vec<u64>){
    let mut to_be_sorted = table.clone();
    table.swap_with_slice(&mut _merge_sort(&mut to_be_sorted)[..]);
}

fn _merge_sort(table: &mut Vec<u64>) -> Vec<u64>{
    if table.len() > 1 {
        let mut left: Vec<u64> = Vec::new();
        let mut right: Vec<u64> = Vec::new();
        for (i, key) in table.iter().enumerate() {
            if i < table.len()/2 {
                left.push(key.clone());
            } else {
                right.push(key.clone());
            }
        }
        left = _merge_sort(&mut left);
        right = _merge_sort(&mut right);
        return merge(left, right);
    } else {
        return table.clone();
    }
}

fn merge(left: Vec<u64>, right: Vec<u64>) -> Vec<u64>{
    let mut merged: VecDeque<u64> = VecDeque::new();
    let mut left = VecDeque::from(left);
    let mut right = VecDeque::from(right);
    while !left.is_empty() && !right.is_empty() {
        if left.front().unwrap() <= right.front().unwrap() {
            merged.push_back(left.pop_front().unwrap());
        } else {
            merged.push_back(right.pop_front().unwrap());
        }
    }
    while !left.is_empty() {
        merged.push_back(left.pop_front().unwrap());
    }
    while !right.is_empty() {
        merged.push_back(right.pop_front().unwrap());
    }
    let merged: Vec<u64> = Vec::from(merged);
    return merged;
}

pub fn quick_sort(table: &mut Vec<u64>){
    let len = table.len();
    _quick_sort(table, 1, len - 1);
}

fn _quick_sort(table: &mut Vec<u64>, low: usize, high: usize){
    if low < high {
        let p = partition(table, low, high);
        _quick_sort(table, low, p - 1);
        _quick_sort(table, p + 1, high);
    }
}

fn partition(table: &mut Vec<u64>, low: usize, high: usize) -> usize{
    let pivot = match table.get(high) {
        Some(v) => {v.clone()}
        _ => {panic!("Array index {:?} out of bounds", high)}
    };
    let mut i = low;
    for j in low..high-1 {
        match table.to_vec().get(j) {
            Some(v) => {
                if v <= &pivot {
                    &table.swap(i, j);
                    i += 1;
                }
            }
            _ => {panic!("Array index {:?} for j out of bounds", j)}
        }
    }
    &table.swap(i, high);
    i
}

fn main() {
    let mut list = gen_list(25, Order::Random);
    println!("{:?}", list);
    insertion_sort(&mut list);
    println!("{:?}", list);
    list = gen_list(25, Order::Random);
    println!("{:?}", list);
    merge_sort(&mut list);
    println!("{:?}", list);
    list = gen_list(25, Order::Random);
    println!("{:?}", list);
    quick_sort(&mut list);
    println!("{:?}", list);
}
