use lib::*;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::create("data.csv").unwrap();
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
    file = File::create("data1.csv").unwrap();
    file.write_all(b"size;n;k;cmps;swaps\n").unwrap();
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

}
