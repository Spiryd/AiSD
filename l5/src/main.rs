
use lib::*;

fn main() {
    let s1 = "AGGTAB".chars().collect();
    let s2 = "GXTXAYB".chars().collect();
    let lcs =  Lcs::new(s1, s2);
    println!("len {}", lcs.len());
    lcs.print_diff();
}
