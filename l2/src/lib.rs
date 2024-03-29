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

pub fn print_as_two_digit(table: &Vec<u64>){
    let mut to_print = String::new();
    to_print += "[";
    for key in table{
        to_print += format!("{:0>2}", key.to_string().as_str()).as_str();
        to_print += ", ";
    }
    to_print.truncate(to_print.len() - 2);
    to_print += "]";
    println!("{}", to_print);
}

pub fn is_sorted(table: &Vec<u64>) -> bool{
    for i in 1..table.len(){
        if table[i] < table[i-1]{
            return false;
        }
    }
    return true;
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

pub fn insertion_sort_with_stats(table: &mut Vec<u64>, print_proc: bool) -> (u32, u32){
    let mut swap_count: u32 = 0; 
    let mut check_count: u32 = 0;
    if print_proc {
        for i in 1..table.len() {
            let mut j = i;
            check_count+=1;
            while j > 0 && table[j] < table[j - 1] {
                check_count+=1;
                table.swap(j, j - 1);
                swap_count+=1;
                j -= 1;  
            }
            print_as_two_digit(&table);
        }
    }else {
        for i in 1..table.len() {
            let mut j = i;
            check_count+=1;
            while j > 0 && table[j] < table[j - 1] {
                check_count+=1;
                table.swap(j, j - 1);
                swap_count+=1;
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

pub fn quick_sort_with_stats(table: &mut Vec<u64>, print_proc: bool) -> (u32, u32){
    //(num of swaps, num od cmp)
    let mut stats: (u32, u32) = (0, 0);
    if !table.is_empty(){
        _quick_sort_with_stats(table, 0, table.len() - 1, &mut stats, print_proc);
    }
    return (stats.0, stats.1);
}

fn partition_with_stats(table: &mut Vec<u64>, low: usize, high: usize, stats: &mut (u32, u32), print_proc: bool) -> usize{
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
            if print_proc {
                print_as_two_digit(&table);
            }
            return right_index as usize;
        }
        stats.0 += 1;
        table.swap(left_index as usize, right_index as usize);
    }
}

fn _quick_sort_with_stats(table: &mut Vec<u64>, low: usize, high: usize, stats: &mut (u32, u32), print_proc: bool){
    if low < high {
        let p = partition_with_stats(table, low, high, stats, print_proc);
        _quick_sort_with_stats(table, low, p, stats, print_proc);
        _quick_sort_with_stats(table, p + 1, high, stats, print_proc);
    }
}

pub fn merge_sort_with_stats(table: &mut Vec<u64>, print_proc: bool) -> (u32, u32){
    let mut stats: (u32, u32) = (0, 0);
    let mut to_be_sorted = table.clone();
    table.swap_with_slice(&mut _merge_sort_with_stats(&mut to_be_sorted, &mut stats, print_proc)[..]);
    return stats;
}

fn _merge_sort_with_stats(table: &mut Vec<u64>, stats: &mut (u32, u32), print_proc: bool) -> Vec<u64>{
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
        left = _merge_sort_with_stats(&mut left, stats, print_proc);
        right = _merge_sort_with_stats(&mut right, stats, print_proc);
        return merge_with_stats(left, right, stats, print_proc);
    } else {
        return table.clone();
    }
}

fn merge_with_stats(left: Vec<u64>, right: Vec<u64>, stats: &mut (u32, u32), print_proc: bool) -> Vec<u64>{
    let mut merged: VecDeque<u64> = VecDeque::new();
    let mut left = VecDeque::from(left);
    let mut right = VecDeque::from(right);
    while !left.is_empty() && !right.is_empty() {
        stats.1 += 1;
        if left.front().unwrap() <= right.front().unwrap() {
            merged.push_back(left.pop_front().unwrap());
            stats.0 += 1;
        } else {
            merged.push_back(right.pop_front().unwrap());
            stats.0 += 1;
        }
    }
    while !left.is_empty() {
        merged.push_back(left.pop_front().unwrap());
        stats.0 += 1;
    }
    while !right.is_empty() {
        merged.push_back(right.pop_front().unwrap());
        stats.0 += 1;
    }
    let merged: Vec<u64> = Vec::from(merged);
    if print_proc {
        print_as_two_digit(&merged);
    }
    return merged;
}

pub fn dual_pivot_quicksort(table: &mut Vec<u64>){
    _dual_pivot_quicksort(table, 0, table.len() - 1)
}

fn _dual_pivot_quicksort(table: &mut Vec<u64>, left: usize, right: usize){
    if left < right  {
        let p: u64;
        let q: u64;
        if table[right] < table[left] {
            table.swap(right, left);
        }
        p = table[left];
        q = table[right];
        let mut i = left + 1;
        let mut k = right - 1;
        let mut j = i;
        let mut d = 0;
        while j <= k {
            if d >= 0 {
                if table[j] < p {
                    table.swap(i, j);
                    i += 1;
                    j += 1;
                    d += 1;
                } else {
                    if table[j] < q {
                        j += 1;
                    } else {
                      table.swap(j, k);
                      k -= 1;
                      d -= 1; 
                    }
                }
            } else {
                if table[k] > q {
                    k -= 1;
                    d -= 1;
                } else {
                    if table[k] < p {
                        let tmp = table[k];
                        table[k] = table[j];
                        table[j] = table[i];
                        table[i] = tmp;
                        i += 1;
                        d += 1;
                    } else {
                        table.swap(j, k);
                    }
                    j += 1;
                }
            }
        }
        table.swap(left, i - 1);
        table.swap(right, k + 1);
        if (i as isize - 2)  >= 0{
            _dual_pivot_quicksort(table, left, i - 2);
        }
        _dual_pivot_quicksort(table, i, k);
        _dual_pivot_quicksort(table, k + 2, right);
    }
}

pub fn dual_pivot_quicksort_with_stats(table: &mut Vec<u64>, print_proc: bool) -> (u32, u32){
    let mut stats: (u32, u32) = (0, 0);
    _dual_pivot_quicksort_with_stats(table, 0, table.len() - 1, &mut stats, print_proc);
    return stats;
}

fn _dual_pivot_quicksort_with_stats(table: &mut Vec<u64>, left: usize, right: usize, stats: &mut (u32, u32), print_proc: bool){
    if left < right  {
        if print_proc {
            print_as_two_digit(table);
        }
        let p: u64;
        let q: u64;
        stats.1 += 1;
        if table[right] < table[left] {
            stats.0 += 1;
            table.swap(right, left);
        }
        p = table[left];
        q = table[right];
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
            _dual_pivot_quicksort_with_stats(table, left, i - 2, stats, print_proc);
        }
        _dual_pivot_quicksort_with_stats(table, i, k, stats, print_proc);
        _dual_pivot_quicksort_with_stats(table, k + 2, right, stats, print_proc);
    }
}

pub fn dual_pivot_quickinssort(table: &mut Vec<u64>){
    _dual_pivot_quickinssort(table, 0, table.len() - 1)
}

fn inssort_to_combo(table: &mut Vec<u64>, left: usize, right: usize){
    for i in left..=right {
        let mut j = i;
        while j > left && table[j] < table[j - 1] {
            table.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn _dual_pivot_quickinssort(table: &mut Vec<u64>, left: usize, right: usize){
    if left < right  {
        if right - left <= 8{
            inssort_to_combo(table, left, right);
        } else {
            let p: u64;
            let q: u64;
            if table[right] < table[left] {
                table.swap(right, left);
            }
            p = table[left];
            q = table[right];
            let mut i = left + 1;
            let mut k = right - 1;
            let mut j = i;
            let mut d = 0;
            while j <= k {
                if d >= 0 {
                    if table[j] < p {
                        table.swap(i, j);
                        i += 1;
                        j += 1;
                        d += 1;
                    } else {
                        if table[j] < q {
                            j += 1;
                        } else {
                          table.swap(j, k);
                          k -= 1;
                          d -= 1; 
                        }
                    }
                } else {
                    if table[k] > q {
                        k -= 1;
                        d -= 1;
                    } else {
                        if table[k] < p {
                            let tmp = table[k];
                            table[k] = table[j];
                            table[j] = table[i];
                            table[i] = tmp;
                            i += 1;
                            d += 1;
                        } else {
                            table.swap(j, k);
                        }
                        j += 1;
                    }
                }
            }
            table.swap(left, i - 1);
            table.swap(right, k + 1);
            if (i as isize - 2)  >= 0{
                _dual_pivot_quickinssort(table, left, i - 2);
            }
            _dual_pivot_quickinssort(table, i, k);
            _dual_pivot_quickinssort(table, k + 2, right);
        }
    }
}

pub fn dual_pivot_quickinssort_with_stats(table: &mut Vec<u64>, print_proc: bool) -> (u32, u32){
    let mut stats = (0, 0);
    _dual_pivot_quickinssort_with_stats(table, 0, table.len() - 1, &mut stats, print_proc);
    return stats;
}

fn inssort_to_combo_with_stats(table: &mut Vec<u64>, left: usize, right: usize, stats: &mut (u32, u32), print_proc: bool){
    for i in left..=right {
        let mut j = i;
        stats.1 += 1;
        while j > left && table[j] < table[j - 1] {
            stats.1 += 1;
            table.swap(j, j - 1);
            stats.0 += 1;
            j -= 1;
        }
        if print_proc {
            print_as_two_digit(&table);
        }
    }
}

fn _dual_pivot_quickinssort_with_stats(table: &mut Vec<u64>, left: usize, right: usize, stats: &mut (u32, u32), print_proc: bool){
    if left < right  {
        if right - left <= 8{
            inssort_to_combo_with_stats(table, left, right, stats, print_proc);
        } else {
            if print_proc {
                print_as_two_digit(&table);
            }
            let p: u64;
            let q: u64;
            stats.1 += 1;
            if table[right] < table[left] {
                stats.0 += 1;
                table.swap(right, left);
            }
            p = table[left];
            q = table[right];
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
                _dual_pivot_quickinssort_with_stats(table, left, i - 2, stats, print_proc);
            }
            _dual_pivot_quickinssort_with_stats(table, i, k, stats, print_proc);
            _dual_pivot_quickinssort_with_stats(table, k + 2, right, stats, print_proc);
        }
    }
}
