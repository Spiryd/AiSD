use lib::*;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::create("data.csv").unwrap();
}
