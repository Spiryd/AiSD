use lib::*;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::create("data.csv").unwrap();
    file.write_all(b"type;n;swaps;cmps\n").unwrap();
    let mut stats: (u32, u32);
    let mut line: String;
    let mut table: Vec<u64>;
    for i in 1..=20 {
        for _ in 0..100 {
            table = gen_list(i*10, Order::Random);
            stats = insertion_sort_with_stats(&mut table, false);
            line = format!("insertion;{};{};{}\n", i*10, stats.0, stats.1);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    for i in 1..=20 {
        for _ in 0..100 {
            table = gen_list(i*10, Order::Random);
            stats = merge_sort_with_stats(&mut table, false);
            line = format!("merge;{};{};{}\n", i*10, stats.0, stats.1);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    for i in 1..=20 {
        for _ in 0..100 {
            table = gen_list(i*1000, Order::Random);
            stats = merge_sort_with_stats(&mut table, false);
            line = format!("merge;{};{};{}\n", i*10, stats.0, stats.1);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    for i in 1..=20 {
        for _ in 0..100 {
            table = gen_list(i*10, Order::Random);
            stats = quick_sort_with_stats(&mut table, false);
            line = format!("quick;{};{};{}\n", i*10, stats.0, stats.1);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    for i in 1..=20 {
        for _ in 0..100 {
            table = gen_list(i*1000, Order::Random);
            stats = quick_sort_with_stats(&mut table, false);
            line = format!("quick;{};{};{}\n", i*10, stats.0, stats.1);
            file.write_all(line.as_bytes()).unwrap();
        }
    }
}
