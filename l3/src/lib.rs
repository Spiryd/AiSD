use rand::prelude::*;
use rand_pcg::Pcg64;
use std::cmp::Ordering;

#[test]
fn test_rselect() {
    let mut rng = Pcg64::from_entropy();
    for _ in 0..1000 {
        let mut list = gen_list(256 , Order::Random);
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
        let mut list = gen_list(256, Order::Random);
        let i = rng.gen_range(0..32);
        let val = select(&mut list.clone(), i, 5);
        list.sort();
        assert_eq!(list[i], val);
    }
}

#[test]
fn test_bin_search(){
    let list: Vec<u64> = (0..1000).collect();
        for target in 0..1000 {
            assert!(bin_is_in(target, list.clone()));
        }
    }

#[test]
fn test_qs_select(){
    for _ in 0..1000 {
        let mut list = gen_list(256, Order::Random);
        quicksort_select(&mut list);
        assert!(is_sorted(&list));
    }
}

#[test]
fn test_dp_qs_select(){
    for _ in 0..1000 {
        let mut list = gen_list(256, Order::Random);
        dp_quicksort_select_with_stats(&mut list);
        assert!(is_sorted(&list));
    }
}

#[derive(Debug)]
pub enum Order{
    Random,
    Sorted,
    Reverse
}

pub fn is_sorted(table: &Vec<u64>) -> bool{
    for i in 1..table.len(){
        if table[i] < table[i-1]{
            return false;
        }
    }
    true
}

pub fn gen_list(n: u64, order: Order) -> Vec<u64>{
    let mut rng: Pcg64 = Pcg64::from_entropy();
    let mut vector: Vec<u64> = Vec::new();
    for _ in 0..n {
        vector.push(rng.gen_range(0..2*n));
    }
    match order {
        Order::Random => vector,
        Order::Sorted => {
            vector.sort();
            vector
        },
        Order::Reverse => {
            vector.sort();
            vector.reverse();
            vector
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
    match n.cmp(&k) {
        Ordering::Less => _randomized_select(arr, left, pivot_index - 1, n),
        Ordering::Equal => arr[pivot_index],
        Ordering::Greater => _randomized_select(arr, pivot_index + 1, right, n - k),
    }
}

fn randomized_partition(arr: &mut [u64], left: usize, right: usize) -> usize {
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
    i
}

pub fn randomized_select_with_stats(arr: &mut Vec<u64>, n: usize, print_process: bool) -> (u64, (u64, u64)) {
    let mut stats = (0, 0);
    let val = _randomized_select_with_stats(arr, 0, arr.len() - 1, n + 1, print_process, &mut stats);
    (val, stats)
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
    match n.cmp(&k) {
        Ordering::Less => _randomized_select_with_stats(arr, left, pivot_index - 1, n, print_process, stats),
        Ordering::Equal => arr[pivot_index],
        Ordering::Greater => _randomized_select_with_stats(arr, pivot_index + 1, right, n - k, print_process, stats),
    }
}

fn randomized_partition_with_stats(arr: &mut [u64], left: usize, right: usize, stats: &mut(u64, u64)) -> usize{
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
    i
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
        match n.cmp(&pivot_index) {
            Ordering::Less => right = pivot_index - 1,
            Ordering::Equal => return n,
            Ordering::Greater => left = pivot_index + 1,
        }
    }
}

fn select_partition(arr: &mut [u64], left: usize, right: usize, pivot_index: usize, n: usize) -> usize {
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
    store_index_eq
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
    _select(arr, left, left + ((right - left)/partition_size), mid, partition_size)
}

fn partition5(arr: &mut [u64], left: usize, right: usize) -> usize {
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
    (left + right) / 2
}

pub fn select_with_stats(arr: &mut Vec<u64>, n: usize, partition_size: usize, print_process: bool) -> (u64, (u64, u64)){
    let mut stats = (0, 0);
    let idx = _select_with_stats(arr, 0, arr.len() - 1, n, partition_size, print_process, &mut stats);
    let val = arr[idx];
    (val, stats)
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
        match n.cmp(&pivot_index) {
            Ordering::Less => right = pivot_index - 1,
            Ordering::Equal => return n,
            Ordering::Greater => left = pivot_index + 1,
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
    _select_with_stats(arr, left, left + ((right - left)/partition_size), mid, partition_size, print_process, stats)
}

fn select_partition_with_stats(arr: &mut [u64], left: usize, right: usize, pivot_index: usize, n: usize, stats: &mut (u64, u64)) -> usize {
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
    store_index_eq
}

fn partition5_with_stats(arr: &mut [u64], left: usize, right: usize, stats: &mut (u64, u64)) -> usize {
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
    (left + right) / 2
}

pub fn bin_is_in(k: u64, items: Vec<u64>) -> bool {
    let low: usize = 0;
    let high: usize = items.len() - 1;


    if low <= high {
        let middle = (high + low) / 2;
        if middle > items.len(){
            return false;
        }
        match items[middle].cmp(&k) {
            Ordering::Equal => return true,
            Ordering::Greater => return bin_is_in(k, items[low ..= middle - 1].to_vec()),
            Ordering::Less => return bin_is_in(k, items[middle + 1 ..= high].to_vec())
        }
    }
    false
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
        *stats += 2;
        match items[middle].cmp(&k) {
            Ordering::Equal => return (true, *stats),
            Ordering::Greater => return _bin_is_in_with_stats(k, items[low ..= middle - 1].to_vec(), stats),
            Ordering::Less => return _bin_is_in_with_stats(k, items[middle + 1 ..= high].to_vec(), stats)
        }
    }
    (false, *stats)
}

pub fn quicksort_select(arr: &mut Vec<u64>) {
    _quicksort_select(arr, 0, arr.len() - 1);
}
 
fn _quicksort_select(arr: &mut Vec<u64>, low: usize, high: usize){
    if low < high {
        let p = partition(arr, low, high);
        _quicksort_select(arr, low, p);
        _quicksort_select(arr, p + 1, high);
    }
}

fn partition(arr: &mut Vec<u64>, low: usize, high: usize) -> usize{
    let pivot = select_pivot(arr, low, high, 5);
    let pivot = arr[pivot];
    let mut left_index = (low  as isize) - 1;
    let mut right_index = (high + 1) as isize;
    loop {
        loop {
            left_index += 1;
            if arr[left_index as usize] >= pivot{
                break;
            }
        }
        loop {
            right_index -= 1;
            if arr[right_index as usize] <= pivot{
                break;
            }
        }
        if left_index >= right_index{
            return right_index as usize;
        }
        arr.swap(left_index as usize, right_index as usize);
    }
}

pub fn quicksort_select_with_stats(arr: &mut Vec<u64>) -> u64{
    let mut stats = (0, 0);
    _quicksort_select_with_stats(arr, 0, arr.len() - 1, &mut stats);
    stats.0
}

fn _quicksort_select_with_stats(arr: &mut Vec<u64>, low: usize, high: usize, stats: &mut (u64, u64)){
    if low < high {
        let p = partition_with_stats(arr, low, high, stats);
        _quicksort_select_with_stats(arr, low, p, stats);
        _quicksort_select_with_stats(arr, p + 1, high, stats);
    }
}

fn partition_with_stats(arr: &mut Vec<u64>, low: usize, high: usize, stats: &mut (u64, u64)) -> usize{
    let pivot = select_pivot_with_stats(arr, low, high, 5, false, stats);
    let pivot = arr[pivot];
    let mut left_index = (low  as isize) - 1;
    let mut right_index = (high + 1) as isize;
    loop {
        loop {
            left_index += 1;
            stats.0 += 1;
            if arr[left_index as usize] >= pivot{
                break;
            }
        }
        loop {
            right_index -= 1;
            stats.0 += 1;
            if arr[right_index as usize] <= pivot{
                break;
            }
        }
        if left_index >= right_index{
            return right_index as usize;
        }
        stats.1 += 1;
        arr.swap(left_index as usize, right_index as usize);
    }
}

pub fn dp_quicksort_select_with_stats(arr: &mut Vec<u64>) -> u64{
    let mut stats = (0, 0);
    _dp_quicksort_select_with_stats(arr, 0, arr.len() - 1, &mut stats);
    stats.0
}

pub fn _dp_quicksort_select_with_stats(arr: &mut Vec<u64>, left: usize, right: usize, stats: &mut (u64, u64)){
    if left < right{
        let mid = (left + right)/2;
        let p_idx = select_pivot_with_stats(arr, left, mid, 5, false, stats);
        let q_idx = select_pivot_with_stats(arr, mid + 1, right, 5, false, stats);
        let p: u64;
        let q: u64;
        arr.swap(left, p_idx);
        arr.swap(right, q_idx);
        stats.0 += 1;
        if arr[right] < arr[left] {
            p = arr[right];
            q = arr[left];
        } else {
            q = arr[right];
            p = arr[left];
        }
        let mut i = left + 1;
        let mut k = right - 1;
        let mut j = i;
        let mut d = 0;
        while j <= k {
            stats.0 += 1;
            if d >= 0{
                stats.0 += 1;
                if arr[j] < p {
                    stats.1 += 1;
                    arr.swap(i, j);
                    i += 1;
                    j += 1;
                    d += 1;
                } else if arr[j] < q {
                    stats.0 += 1;
                    j += 1;
                    
                } else {
                    stats.0 += 1;
                    stats.1 += 1;
                    arr.swap(j, k);
                    k -= 1;
                    d -= 1;
                }
            } else if arr[k] > q {
                stats.0 += 1;
                k -= 1;
                d -= 1;
            } else {
                stats.0 += 1;
                if arr[k] < p {
                    stats.1 += 3;
                    let tmp = arr[k];
                    arr[k] = arr[j];
                    arr[j] = arr[i];
                    arr[i] = tmp;
                    i += 1;
                    d += 1;
                } else {
                    stats.1 += 1;
                    arr.swap(j, k)
                }
                j += 1;
            }
        }
        stats.1 += 4;
        arr[left] = arr[i - 1];
        arr[i - 1] = p;
        arr[right] = arr[k + 1];
        arr[k + 1] = q;
        if (i as isize - 2)  >= 0{
            _dp_quicksort_select_with_stats(arr, left, i - 2, stats);
        }
        _dp_quicksort_select_with_stats(arr, i, k, stats);
        _dp_quicksort_select_with_stats(arr, k + 2, right, stats);
    }
}

pub fn dual_pivot_quicksort_with_stats(table: &mut Vec<u64>) -> u64{
    let mut stats: (u64, u64) = (0, 0);
    _dual_pivot_quicksort_with_stats(table, 0, table.len() - 1, &mut stats);
    stats.1
}

fn _dual_pivot_quicksort_with_stats(table: &mut Vec<u64>, left: usize, right: usize, stats: &mut (u64, u64)){
    if left < right  {
        stats.1 += 1;
        if table[right] < table[left] {
            stats.0 += 1;
            table.swap(right, left);
        }
        let p = table[left];
        let q = table[right];
        let mut i = left + 1;
        let mut k = right - 1;
        let mut j = i;
        let mut d = 0;
        while j <= k {
            if d >= 0 {
                stats.1 += 1;
                if table[j] < p {
                    stats.0 += 1;
                    table.swap(i, j);
                    i += 1;
                    j += 1;
                    d += 1;
                } else {
                    stats.1 += 1;
                    if table[j] < q {
                        j += 1;
                    } else {
                        stats.0 += 1;
                        table.swap(j, k);
                        k -= 1;
                        d -= 1;
                    }
                }
            } else {
                stats.1 += 1;
                if table[k] > q {
                    k -= 1;
                    d -= 1;
                } else {
                    stats.1 += 1;
                    if table[k] < p {
                        let tmp = table[k];
                        table[k] = table[j];
                        table[j] = table[i];
                        table[i] = tmp;
                        i += 1;
                        d += 1;
                    } else {
                        stats.0 += 1;
                        table.swap(j, k);
                    }
                    j += 1;
                }
            }
        }
        stats.0 += 1;
        table.swap(left, i - 1);
        stats.0 += 1;
        table.swap(right, k + 1);
        if (i as isize - 2)  >= 0{
            _dual_pivot_quicksort_with_stats(table, left, i - 2, stats);
        }
        _dual_pivot_quicksort_with_stats(table, i, k, stats);
        _dual_pivot_quicksort_with_stats(table, k + 2, right, stats);
    }
}

pub fn quick_sort_with_stats(table: &mut Vec<u64>, print_proc: bool) ->  u64{
    //(num of swaps, num od cmp)
    let mut stats: (u64, u64) = (0, 0);
    if !table.is_empty(){
        _quick_sort_with_stats(table, 0, table.len() - 1, &mut stats, print_proc);
    }
    stats.1
}

fn qs_partition_with_stats(table: &mut [u64], low: usize, high: usize, stats: &mut (u64, u64), _print_proc: bool) -> usize{
    let pivot = table[(((((high - low)/2) as f64).floor()) as usize) + low];
    let mut left_index = (low  as isize) - 1;
    let mut right_index = (high + 1) as isize;
    loop {
        loop {
            left_index += 1;
            stats.1 += 1;
            if table[left_index as usize] >= pivot{
                break;
            }
        }
        loop {
            right_index -= 1;
            stats.1 += 1;
            if table[right_index as usize] <= pivot{
                break;
            }
        }
        if left_index >= right_index{
            return right_index as usize;
        }
        stats.0 += 1;
        table.swap(left_index as usize, right_index as usize);
    }
}

fn _quick_sort_with_stats(table: &mut Vec<u64>, low: usize, high: usize, stats: &mut (u64, u64), print_proc: bool){
    if low < high {
        let p = qs_partition_with_stats(table, low, high, stats, print_proc);
        _quick_sort_with_stats(table, low, p, stats, print_proc);
        _quick_sort_with_stats(table, p + 1, high, stats, print_proc);
    }
}
