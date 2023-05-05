use lib::*;
use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::thread;
use std::time::{Instant, Duration};

fn main(){
    let data_thread1 = thread::spawn(|| {
        let mut file = File::create("./data/select_data.csv").unwrap();
        file.write_all(b"type;n;k;cmps;swaps\n").unwrap();
        let mut stats: (u64, u64);
        let mut list: Vec<u64>;
        let mut line: String;
        for n in 1..=100{
            for k in [(0, "first"),((n*100)/4, "n/4"), ((n*200)/4, "2n/4"), ((n*300)/4, "3n/4"), (n*100 - 1, "last")]{
                for _ in 0..1000 {
                    list = gen_list(n*100, Order::Random);
                    stats = select_with_stats(&mut list, (k.0).try_into().unwrap(), 5, false).1;
                    line = format!("select;{};{};{};{}\n", n*100, k.1, stats.0, stats.1);
                    file.write_all(line.as_bytes()).unwrap();
                }
            }
        }
        for n in 1..=100{
            for k in [(0, "first"),((n*100)/4, "n/4"), ((n*200)/4, "2n/4"), ((n*300)/4, "3n/4"), (n*100 - 1, "last")]{
                for _ in 0..1000 {
                    list = gen_list(n*100, Order::Random);
                    stats = randomized_select_with_stats(&mut list, (k.0).try_into().unwrap(), false).1;
                    line = format!("rand_select;{};{};{};{}\n", n*100, k.1, stats.0, stats.1);
                    file.write_all(line.as_bytes()).unwrap();
                }
            }
        }
    });

    let data_thread2 = thread::spawn(|| {
        let mut file = File::create("./data/container_size_data.csv").unwrap();
        file.write_all(b"size;n;k;cmps;swaps\n").unwrap();
        let mut stats: (u64, u64);
        let mut list: Vec<u64>;
        let mut line: String;
        for size in [3, 5, 7, 9] {
            for n in 1..=100{
                for k in [(0, "first"),((n*100)/4, "n/4"), ((n*200)/4, "2n/4"), ((n*300)/4, "3n/4"), (n*100 - 1, "last")]{
                    for _ in 0..1000 {
                        list = gen_list(n*100, Order::Random);
                        stats = select_with_stats(&mut list, (k.0).try_into().unwrap(), size, false).1;
                        line = format!("{size};{};{};{};{}\n", n*100, k.1, stats.0, stats.1);
                        file.write_all(line.as_bytes()).unwrap();
                    }
                }
            }
        }
    });

    let data_thread3 = thread::spawn(|| {
        let mut file = File::create("./data/bin_search_data1.csv").unwrap();
        file.write_all(b"target;n;cmps;time\n").unwrap();
        let mut cmps: u64;
        let mut list: Vec<u64>;
        let mut line: String;
        let mut start: Instant;
        let mut time: Duration;
        for n in (1000..=100000).step_by(1000) {
            list = (0..n).collect();
            for _ in 0..1000{
                start = Instant::now();
                cmps = bin_is_in_with_stats(3, list.clone()).1;
                time = start.elapsed();
                line = format!("3;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
            for _ in 0..1000{
                start = Instant::now();
                cmps = bin_is_in_with_stats(n/12, list.clone()).1;
                time = start.elapsed();
                line = format!("n/12;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
            for _ in 0..1000{
                start = Instant::now();
                cmps = bin_is_in_with_stats((7 * n)/12, list.clone()).1;
                time = start.elapsed();
                line = format!("7n/12;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
            for _ in 0..1000{
                start = Instant::now();
                cmps = bin_is_in_with_stats((11 * n)/12, list.clone()).1;
                time = start.elapsed();
                line = format!("11n/12;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
            for _ in 0..1000{
                start = Instant::now();
                cmps = bin_is_in_with_stats(n-4, list.clone()).1;
                time = start.elapsed();
                line = format!("n-4;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
            for _ in 0..1000{
                start = Instant::now();
                cmps = bin_is_in_with_stats(n, list.clone()).1;
                time = start.elapsed();
                line = format!("n;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
        }
    });

    let data_thread4 = thread::spawn(|| {
        let mut file = File::create("./data/bin_search_data2.csv").unwrap();
        file.write_all(b"n;cmps;time\n").unwrap();
        let mut cmps: u64;
        let mut list: Vec<u64>;
        let mut line: String;
        let mut start: Instant;
        let mut time: Duration;
        let mut rng = Pcg64::from_entropy();
        let mut choosen: u64;
        for n in (1000..=100000).step_by(1000) {
            list = (0..n).collect();
            for _ in 0..1000{
                choosen = rng.gen_range(0..n);
                start = Instant::now();
                cmps = bin_is_in_with_stats(choosen, list.clone()).1;
                time = start.elapsed();
                line = format!("{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
        }
    });

    let data_thread5 = thread::spawn(|| {
        let mut file = File::create("./data/quicksorts.csv").unwrap();
        file.write_all(b"type;n;cmps;time\n").unwrap();
        let mut cmps: u64;
        let mut list: Vec<u64>;
        let mut line: String;
        let mut start: Instant;
        let mut time: Duration;
        for n in (100..=10000).step_by(100) {
            list = (0..n).collect();
            for _ in 0..1000{
                start = Instant::now();
                cmps = quick_sort_with_stats(&mut list.clone(), false);
                time = start.elapsed();
                line = format!("basic;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();

                start = Instant::now();
                cmps = quicksort_select_with_stats(&mut list.clone());
                time = start.elapsed();
                line = format!("select;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
        }
    });

    let data_thread6 = thread::spawn(|| {
        let mut file = File::create("./data/dp_quicksorts.csv").unwrap();
        file.write_all(b"type;n;cmps;time\n").unwrap();
        let mut cmps: u64;
        let mut list: Vec<u64>;
        let mut line: String;
        let mut start: Instant;
        let mut time: Duration;
        for n in (100..=10000).step_by(100) {
            list = (0..n).collect();
            for _ in 0..1000{
                start = Instant::now();
                cmps = dual_pivot_quicksort_with_stats(&mut list.clone());
                time = start.elapsed();
                line = format!("basic;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
                
                start = Instant::now();
                cmps = dp_quicksort_select_with_stats(&mut list.clone());
                time = start.elapsed();
                line = format!("select;{n};{cmps};{}\n", time.as_nanos());
                file.write_all(line.as_bytes()).unwrap();
            }
        }
    });

    data_thread1.join().unwrap();
    data_thread2.join().unwrap();
    data_thread3.join().unwrap();
    data_thread4.join().unwrap();
    data_thread5.join().unwrap();
    data_thread6.join().unwrap();
}
