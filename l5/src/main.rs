use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use lib::*;

fn main() {
    let choices = ["Present", "Collect Data"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your poison")
        .items(&choices)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");
    if selection.is_none() {
        panic!("User did not select anything")
    }

    match selection.unwrap() {
        0 => present(),
        1 => collect_data(),
        _ => panic!("somthing went wrong"),
    }    

}

fn present() {
    let s1 = "XMJYAUZ".chars().collect();
    let s2 = "MZJAWXU".chars().collect();
    println!("s1 = {:?}", s1);
    println!("s2 = {:?}", s2);
    let lcs =  Lcs::new(s1, s2);
    println!("len = {}", lcs.len());
    lcs.print();
    let s1 = "ACADB".chars().collect();
    let s2 = "CBDA".chars().collect();
    println!("s1 = {:?}", s1);
    println!("s2 = {:?}", s2);
    let lcs =  Lcs::new(s1, s2);
    println!("len = {}", lcs.len());
    lcs.print();
}

fn collect_data() {
    let mut file = File::create("./data/lcs_data.csv").unwrap();
    file.write_all(b"n;time\n").unwrap();
    for n in (1000..=5000).step_by(1000) {
        for _ in 0..100 {
            let a = gen_list(n, Order::Random);
            let b = gen_list(n, Order::Random);
            let start = Instant::now();
            lcs_length(a, b);
            let time = start.elapsed();
            file.write_all(format!("{n};{}\n", time.as_millis()).as_bytes()).unwrap();
        }
    }

}
