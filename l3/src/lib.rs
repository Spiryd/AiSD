use rand::prelude::*;
use rand_pcg::Pcg64;
use std::cmp::Ordering;

#[test]
fn test_rselect() {
    let mut rng = Pcg64::from_entropy();
    for _ in 0..1000 {
        let mut list = gen_list(32, Order::Random);
        let i = rng.gen_range(0..32);
        let val = randomized_select(&mut list.clone(), i);
        list.sort();
        assert_eq!(list[i], val);
    }
}

#[test]
fn test_select() {
    let mut rng = Pcg64::from_entropy();
    for _ in 0..1000 {
        let mut list = gen_list(32, Order::Random);
        let i = rng.gen_range(0..32);
        let val = select(&mut list.clone(), i, 5);
        list.sort();
        assert_eq!(list[i], val);
    }
}

#[test]
fn test_bin_search(){
    let list: Vec<u64> = (0..1000).collect();
    for _ in 0..100{
        for target in [10, 250, 500, 750, 990]{
            assert!(bin_is_in(target, list.clone()));
        }
    }
}

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

pub fn randomized_select(arr: &mut Vec<u64>, n: usize) -> u64 {
    //to start from 0 we add 1
    _randomized_select(arr, 0, arr.len() - 1, n + 1)
}

fn _randomized_select(arr: &mut Vec<u64>, left: usize, right: usize, n: usize) -> u64 {
    if left == right {
        return arr[left];
    }
    let pivot_index = randomized_partition(arr, left, right);
    let k = pivot_index - left + 1;
    if n == k {
        return arr[pivot_index];
    } else if n < k {
        return _randomized_select(arr, left, pivot_index - 1, n);
    } else {
        return _randomized_select(arr, pivot_index + 1, right, n - k);
    }
}

fn randomized_partition(arr: &mut Vec<u64>, left: usize, right: usize) -> usize {
    let mut rng = Pcg64::from_entropy();
    let mut pivot_index: usize = left;
    if left < right {
        pivot_index = rng.gen_range(left..=right);
    }
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

pub fn randomized_select_with_stats(arr: &mut Vec<u64>, n: usize, print_process: bool) -> (u64, (u64, u64)) {
    let mut stats = (0, 0);
    let val = _randomized_select_with_stats(arr, 0, arr.len() - 1, n + 1, print_process, &mut stats);
    return (val, stats);
}

fn _randomized_select_with_stats(arr: &mut Vec<u64>, left: usize, right: usize, n: usize, print_process: bool, stats: &mut(u64, u64)) -> u64 {
    if print_process{
        println!("{:?}", arr);
    }
    if left == right {
        return arr[left];
    }
    let pivot_index = randomized_partition_with_stats(arr, left, right, stats);
    let k = pivot_index - left + 1;
    if n == k {
        return arr[pivot_index];
    } else if n < k {
        return _randomized_select_with_stats(arr, left, pivot_index - 1, n, print_process, stats);
    } else {
        return _randomized_select_with_stats(arr, pivot_index + 1, right, n - k, print_process, stats);
    }
}

fn randomized_partition_with_stats(arr: &mut Vec<u64>, left: usize, right: usize, stats: &mut(u64, u64)) -> usize{
    let mut rng = Pcg64::from_entropy();
    let mut pivot_index: usize = left;
    if left < right {
        pivot_index = rng.gen_range(left..=right);
    }
    stats.1 += 1;
    arr.swap(pivot_index, right);
    let mut i = left;
    for j in left..right {
        stats.0 += 1;
        if arr[j] <= arr[right] {
            stats.1 += 1;
            arr.swap(i, j);
            i += 1;
        }
    }
    stats.1 += 1;
    arr.swap(i, right);
    return i;
}

pub fn select(arr: &mut Vec<u64>, n: usize, partition_size: usize) -> u64{
    let idx = _select(arr, 0, arr.len() - 1, n, partition_size);
    arr[idx]
}

fn _select(arr: &mut Vec<u64>, mut left: usize, mut right: usize, n: usize, partition_size: usize) -> usize {
    loop {
        if left == right{
            return left;
        }
        let mut pivot_index = select_pivot(arr, left, right, partition_size);
        pivot_index = select_partition(arr, left, right, pivot_index, n);
        if n == pivot_index {
            return n;
        } else if n < pivot_index {
            right = pivot_index - 1
        } else {
            left = pivot_index + 1
        }
    }
}

fn select_partition(arr: &mut Vec<u64>, left: usize, right: usize, pivot_index: usize, n: usize) -> usize {
    let pivot_value =  arr[pivot_index];
    arr.swap(pivot_index, right);
    let mut store_index = left;
    for i in left..right {
        if arr[i] < pivot_value {
            arr.swap(store_index, i);
            store_index += 1;
        }
    }
    let mut store_index_eq = store_index;
    for i in store_index..right {
        if arr[i] == pivot_value {
            arr.swap(store_index_eq, i);
            store_index_eq += 1;
        }
    }
    arr.swap(right, store_index_eq);
    if n < store_index {
       return store_index; 
    }
    if n <= store_index_eq {
        return n;
    }
    return store_index_eq;
}

fn select_pivot(arr: &mut Vec<u64>, left: usize, right: usize, partition_size: usize) -> usize {
    if right - left < partition_size{
        return partition5(arr, left, right);
    }
    for i in (left..=right).step_by(partition_size){
        let mut sub_right = i + partition_size - 1;
        if sub_right > right {
            sub_right = right
        }
        let med5 = partition5(arr, i, sub_right);
        arr.swap(med5, left + ((i - left)/partition_size));
    }
    let mid = ((right - left)/(partition_size*2)) + left + 1;
    return _select(arr, left, left + ((right - left)/partition_size), mid, partition_size);
}

fn partition5(arr: &mut Vec<u64>, left: usize, right: usize) -> usize {
    let mut i = left + 1;
    let mut j: usize;
    while i <= right {
        while i <= right {
            j = i;
            while j > left && arr[j - 1] > arr[j] {
                arr.swap(j-1, j);
                j -= 1;
            }
            i += 1;
        }
    }
    return (left + right) / 2;
}

pub fn select_with_stats(arr: &mut Vec<u64>, n: usize, partition_size: usize, print_process: bool) -> (u64, (u64, u64)){
    let mut stats = (0, 0);
    let idx = _select_with_stats(arr, 0, arr.len() - 1, n, partition_size, print_process, &mut stats);
    let val = arr[idx];
    return (val, stats);
}

fn _select_with_stats(arr:  &mut Vec<u64>, mut left: usize, mut right: usize, n: usize, partition_size: usize, print_process: bool, stats: &mut (u64, u64)) -> usize {
    if print_process {
        println!("{:?}", arr);
    }
    loop {
        if left == right{
            return left;
        }
        let mut pivot_index = select_pivot_with_stats(arr, left, right, partition_size, print_process, stats);
        pivot_index = select_partition_with_stats(arr, left, right, pivot_index, n, stats);
        if n == pivot_index {
            return n;
        } else if n < pivot_index {
            right = pivot_index - 1
        } else {
            left = pivot_index + 1
        }
    }
}

fn select_pivot_with_stats(arr: &mut Vec<u64>, left: usize, right: usize, partition_size: usize,  print_process: bool, stats: &mut (u64, u64)) -> usize{
    if right - left < 5{
        return partition5_with_stats(arr, left, right, stats);
    }
    for i in (left..=right).step_by(partition_size){
        let mut sub_right = i + partition_size - 1;
        if sub_right > right {
            sub_right = right
        }
        let med5 = partition5_with_stats(arr, i, sub_right, stats);
        stats.1 += 1;
        arr.swap(med5, left + ((i - left)/5));
    }
    let mid = ((right - left)/(partition_size*2)) + left + 1;
    return _select_with_stats(arr, left, left + ((right - left)/partition_size), mid, partition_size, print_process, stats);
}

fn select_partition_with_stats(arr: &mut Vec<u64>, left: usize, right: usize, pivot_index: usize, n: usize, stats: &mut (u64, u64)) -> usize {
    let pivot_value =  arr[pivot_index];
    stats.1 += 1;
    arr.swap(pivot_index, right);
    let mut store_index = left;
    for i in left..right {
        stats.0 += 1;
        if arr[i] < pivot_value {
            stats.1 += 1;
            arr.swap(store_index, i);
            store_index += 1;
        }
    }
    let mut store_index_eq = store_index;
    for i in store_index..right {
        stats.0 += 1;
        if arr[i] == pivot_value {
            stats.1 += 1;
            arr.swap(store_index_eq, i);
            store_index_eq += 1;
        }
    }
    stats.1 += 1;
    arr.swap(right, store_index_eq);
    if n < store_index {
       return store_index; 
    }
    if n <= store_index_eq {
        return n;
    }
    return store_index_eq;
}

fn partition5_with_stats(arr: &mut Vec<u64>, left: usize, right: usize, stats: &mut (u64, u64)) -> usize {
    let mut i = left + 1;
    let mut j: usize;
    while i <= right {
        while i <= right {
            j = i;
            stats.0 += 1;
            while j > left && arr[j - 1] > arr[j] {
                stats.1 += 1;
                arr.swap(j-1, j);
                j -= 1;
                stats.0 += 1;
            }
            i += 1;
        }
    }
    return (left + right) / 2;
}

pub fn bin_is_in(k: u64, items: Vec<u64>) -> bool {
    let low: usize = 0;
    let high: usize = items.len() - 1;
 
    if low < high {
        let middle = (high + low) / 2;
        println!("{middle}");
        match items[middle].cmp(&k) {
            Ordering::Equal => return true,
            Ordering::Greater => return bin_is_in(k, items[low ..= middle - 1].to_vec()),
            Ordering::Less => return bin_is_in(k, items[middle + 1 ..= high].to_vec())
        }
    }
    return false;
}

pub fn bin_is_in_with_stats(k: u64, items: Vec<u64>) -> (bool, u64) {
    let mut stats = 0;
    _bin_is_in_with_stats(k, items, &mut stats)
}

fn _bin_is_in_with_stats(k: u64, items: Vec<u64>, stats: &mut u64) -> (bool, u64) {
    let low: usize = 0;
    let high: usize = items.len() - 1;
 
    if low < high {
        let middle = (high + low) / 2;
        match items[middle].cmp(&k) {
            Ordering::Equal => return (true, *stats),
            Ordering::Greater => return _bin_is_in_with_stats(k, items[low ..= middle - 1].to_vec(), stats),
            Ordering::Less => return _bin_is_in_with_stats(k, items[middle + 1 ..= high].to_vec(), stats)
        }
    }
    return (false, *stats);
}
