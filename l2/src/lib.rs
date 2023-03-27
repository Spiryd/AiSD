use std::collections::VecDeque;
use rand::prelude::*;
use rand_pcg::Pcg64;

#[derive(Debug)]
pub enum Order{
    Random,
    Sorted,
    Reverse
}

pub fn gen_list(n: u64, order: Order) -> Vec<u64>{
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

pub fn print_to_std(list: Vec<u64>){
    for key in list {
        print!(" {key}")
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

pub fn insertion_sort_with_stats(table: &mut Vec<u64>, full_stats: bool) -> (u32, u32){
    let mut swap_count: u32 = 0; 
    let mut check_count: u32 = 0;
    if full_stats {
        for i in 1..table.len() {
            let mut j = i;
            check_count+=1;
            while j > 0 && table[j] < table[j - 1] {
                check_count+=1;
                table.swap(j, j - 1);
                swap_count+=1;
                j -= 1;
                println!("{:?}", table);
            }
        }
    }else {
        for i in 1..table.len() {
            let mut j = i;
            while j > 0 && table[j] < table[j - 1] {
                table.swap(j, j - 1);
                j -= 1;
            }
        }
    }
    return (swap_count, check_count);

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
    if !table.is_empty(){
        _quick_sort(table, 0, table.len() - 1);
    }
}

fn _quick_sort(table: &mut Vec<u64>, low: usize, high: usize){
    if low < high {
        let p = partition(table, low, high);
        _quick_sort(table, low, p);
        _quick_sort(table, p + 1, high);
    }
}

fn partition(table: &mut Vec<u64>, low: usize, high: usize) -> usize{
    let pivot = table[(((((high - low)/2) as f64).floor()) as usize) + low];
    let mut left_index = (low  as isize) - 1;
    let mut right_index = (high + 1) as isize;
    loop {
        loop {
            left_index += 1;
            if table[left_index as usize] >= pivot{
                break;
            }
        }
        loop {
            right_index -= 1;
            if table[right_index as usize] <= pivot{
                break;
            }
        }
        if left_index >= right_index{
            return right_index as usize;
        }
        table.swap(left_index as usize, right_index as usize);
    }
}

pub fn quick_sort_with_stats(table: &mut Vec<u64>, full_stats: bool) -> (u32, u32){
    let mut stats: (u32, u32) = (0, 0);
    if !table.is_empty(){
        _quick_sort(table, 0, table.len() - 1);
    }
    return (stats.0, stats.1);
}

fn partition_with_stats(table: &mut Vec<u64>, low: usize, high: usize) -> usize{
    let pivot = table[(((((high - low)/2) as f64).floor()) as usize) + low];
    let mut left_index = (low  as isize) - 1;
    let mut right_index = (high + 1) as isize;
    loop {
        loop {
            left_index += 1;
            if table[left_index as usize] >= pivot{
                break;
            }
        }
        loop {
            right_index -= 1;
            if table[right_index as usize] <= pivot{
                break;
            }
        }
        if left_index >= right_index{
            return right_index as usize;
        }
        table.swap(left_index as usize, right_index as usize);
    }
}

fn _quick_sort_with_stats(table: &mut Vec<u64>, low: usize, high: usize){
    if low < high {
        let p = partition(table, low, high);
        _quick_sort(table, low, p);
        _quick_sort(table, p + 1, high);
    }
}
