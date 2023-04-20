use lib::*;
use std::fs::File;
use std::io::prelude::*;
use rand::prelude::*;
use rand_pcg::Pcg64;
use std::thread;
use std::time::Instant;

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
        file.write_all(b"n;idx;cmps;time\n").unwrap();
        let mut cmps: u64;
        let mut list: Vec<u64>;
        let mut line: String;
        let mut start: Instant;
        for n in (1000..=100000).step_by(1000) {
            for target_idx in [n/12, (7*n)/12, (11*n)/12]{
                for _ in 0..100{
                    list = gen_list(n, Order::Sorted);
                    start = Instant::now();
                    match rec_bin_search_with_stats(&list, list[target_idx as usize]) {
                        Ok(_) => todo!(),
                        Err(_) => todo!(),
                    }
                }
            }
        }
    });

    data_thread1.join().unwrap();
    data_thread2.join().unwrap();
    data_thread3.join().unwrap();
}
