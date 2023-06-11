use std::fmt::Debug;

use rand::prelude::*;
use rand_pcg::Pcg64;

#[derive(Debug)]
pub enum Order{
    Random,
    Sorted,
    Reverse
}

pub fn gen_list(n: u32, order: Order) -> Vec<u32>{
    let mut rng: Pcg64 = Pcg64::from_entropy();
    let mut vector: Vec<u32> = Vec::new();
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

pub struct Lcs<T: Ord + PartialEq + Debug> {
    a: Vec<T>, 
    b: Vec<T>,
    c: Vec<Vec<usize>>
}

impl<T: Ord + PartialEq + Clone + Debug> Lcs<T>{
    pub fn new(a: Vec<T>, b: Vec<T>) -> Self {
        let c = c_gen(a.clone(), b.clone());
        Lcs { a, b, c }
    }

    pub fn len(&self) -> usize{
        self.c[self.a.len()][self.b.len()]
    }

    pub fn print_diff(&self) {
        let mut i = self.a.len() as isize;
        let mut j = self.b.len() as isize;

        while i >= 0 && j >= 0 {
            if i > 0 && j > 0 && self.a[i as usize - 1] == self.b[i as usize - 1] {
                print!("  {:?}", self.a[i as usize - 1]);
                i -= 1;
                j -= 1;
            } else if j > 0 && (i == 0 || self.c[i as usize][j as usize - 1] >= self.c[i as usize - 1][j as usize]) {
                print!("+ {:?}", self.b[i as usize - 1]);
                j -= 1;
            } else if i > 0 && (j == 0 || self.c[i as usize][j as usize - 1] < self.c[i as usize - 1][j as usize]) {
                print!("- {:?}", self.a[i as usize - 1]);
                i -= 1;
            } else {
                break;
            }
        }
    }
}

fn c_gen<T: Ord + PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<Vec<usize>>{
    let m = a.len();
    let n = b.len();
    let mut c: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m{
        for j in 0..=n{
            if i == 0 || j == 0 {
                c[i][j] = 0;
            } else if a[i-1] == b[j-1] {
                c[i][j] = c[i-1][j-1]+1;
            } else {
                c[i][j] = c[i-1][j].max(c[i][j - 1]);
            }
        }
    }
    c
}

pub fn lcs_length<T: Ord + PartialEq>(a: Vec<T>, b: Vec<T>) -> usize {
    let m = a.len();
    let n = b.len();
    let mut c: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m{
        for j in 0..=n{
            if i == 0 || j == 0 {
                c[i][j] = 0;
            } else if a[i-1] == b[j-1] {
                c[i][j] = c[i-1][j-1]+1;
            } else {
                c[i][j] = c[i-1][j].max(c[i][j - 1]);
            }
        }
    }
    c[m][n]
}

#[cfg(test)]
mod tests {
    use crate::lcs_length;

    #[test]
    fn test_lcs_len() {
        let s1 = "AGGTAB".as_bytes().to_vec();
        let s2 = "GXTXAYB".as_bytes().to_vec();
        assert_eq!(lcs_length(s1, s2), 4);
    }
}
