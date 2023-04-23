use lib::*;
use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::thread;
use std::time::{Instant, Duration};

fn main(){
    let data_thread1 = thread::spawn(|| {
        let mut file = File::create("select_data.csv").unwrap();
        file.write_all(b"type;n;k;cmps;swaps\n").unwrap();
        let mut stats: (u64, u64);
        let mut list: Vec<u64>;
        let mut line: String;
        for n in 1..=100{
            for k in [0,(n*100)/4, (n*100)/4, (n*300)/4, (n*100 - 1)]{
                for _ in 0..100 {
                    list = gen_list(n*100, Order::Random);
                    stats = select_with_stats(&mut list, k.try_into().unwrap(), 5, false).1;
                    line = format!("select;{};{k};{};{}\n", n*100, stats.0, stats.1);
                    file.write_all(line.as_bytes()).unwrap();
                }
            }
        }
        for n in 1..=100{
            for k in [0,(n*100)/4, (n*100)/4, (n*300)/4, (n*100 - 1)]{
                for _ in 0..100 {
                    list = gen_list(n*100, Order::Random);
                    stats = randomized_select_with_stats(&mut list, k.try_into().unwrap(), false).1;
                    line = format!("select;{};{k};{};{}\n", n*100, stats.0, stats.1);
                    file.write_all(line.as_bytes()).unwrap();
                }
            }
        }
    });

    let data_thread2 = thread::spawn(|| {
        let mut file = File::create("container_size_data.csv").unwrap();
        file.write_all(b"size;n;k;cmps;swaps\n").unwrap();
        let mut stats: (u64, u64);
        let mut list: Vec<u64>;
        let mut line: String;
        for size in [3, 5, 7, 9] {
            for n in 1..=100{
                for k in [0,(n*100)/4, (n*100)/4, (n*300)/4, (n*100 - 1)]{
                    for _ in 0..100 {
                        list = gen_list(n*100, Order::Random);
                        stats = select_with_stats(&mut list, k.try_into().unwrap(), size, false).1;
                        line = format!("{size};{};{k};{};{}\n", n*100, stats.0, stats.1);
                        file.write_all(line.as_bytes()).unwrap();
                    }
                }
            }
        }
    });

    let data_thread3 = thread::spawn(|| {
        let mut file = File::create("bin_search_data1.csv").unwrap();
        file.write_all(b"target;n;cmps;time\n").unwrap();
        let mut cmps: u64;
        let mut list: Vec<u64>;
        let mut line: String;
        let mut start: Instant;
        let mut time: Duration;
        for n in (1000..=100000).step_by(1000) {
            list = (0..n).collect();
            for target in [3, n/12, (7 * n)/12, (11 * n)/12, n-4]{
                for _ in 0..100{
                    start = Instant::now();
                    cmps = bin_is_in_with_stats(target, list.clone()).1;
                    time = start.elapsed();
                    line = format!("{target};{n};{cmps};{}\n", time.as_nanos());
                    file.write_all(line.as_bytes()).unwrap();
                }
            }
        }
    });

    let data_thread4 = thread::spawn(|| {
        let mut file = File::create("bin_search_data2.csv").unwrap();
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

    data_thread1.join().unwrap();
    data_thread2.join().unwrap();
    data_thread3.join().unwrap();
    data_thread4.join().unwrap();
}
