use std::{fs::File, io::Write, thread};

use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use lib::*;

const STACK_SIZE: usize = 16 * 1024 * 1024;

fn main() {
    loop {
        let choices = ["Test BST", "Test RBTree", "Test SplayTree", "Collect Data", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("SELECT MODE")
            .items(&choices)
            .default(0)
            .interact_on_opt(&Term::stderr())
            .expect("failed");
        if selection.is_none() {
            panic!("User did not select anything")
        }
    
        match selection.unwrap() {
            0 => test_bst(),
            1 => test_rb(),
            2 => test_splay(),
            3 => {
                let exp = thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn(collect_data)
                .unwrap();
                exp.join().unwrap();
            },
            _ => break,
        }
    }
}

fn test_bst() {
    println!("BST");
    let items = gen_list(50, Order::Sorted);
    let mut b_tree = BinarySearchTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        b_tree.insert(*item);
        b_tree.print();
        println!("Tree height: {}", b_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        b_tree.delete(*item);
        b_tree.print();
        println!("Tree height: {}", b_tree.height());
    }
    let items = gen_list(50, Order::Random);
    let mut b_tree = BinarySearchTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        b_tree.insert(*item);
        b_tree.print();
        println!("Tree height: {}", b_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        b_tree.delete(*item);
        b_tree.print();
        println!("Tree height: {}", b_tree.height());
    }
}

fn test_rb() {
    println!("RB Tree");
    let items = gen_list(50, Order::Sorted);
    let mut rb_tree = RBTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        rb_tree.insert(*item);
        rb_tree.print_tree();
        println!("Tree height: {}", rb_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        rb_tree.delete(*item);
        rb_tree.print_tree();
        println!("Tree height: {}", rb_tree.height());
    }
    let items = gen_list(50, Order::Random);
    let mut rb_tree = RBTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        rb_tree.insert(*item);
        rb_tree.print_tree();
        println!("Tree height: {}", rb_tree.height());
    }
    let items = gen_list(50, Order::Random);
    for item in &items{
        println!("Deleting: {}", item);
        rb_tree.delete(*item);
        rb_tree.print_tree();
        println!("Tree height: {}", rb_tree.height());
    }
}

fn test_splay() {
    println!("Splay Tree");
    let items = gen_list(30, Order::Sorted);
    let mut splay_tree = SplayTree::new();
    for item in &items{
        println!("Inserting: {}", item);
        splay_tree.insert(*item);
        splay_tree.print_tree();
        println!("Tree height: {}", splay_tree.height());
    }
    let items = gen_list(30, Order::Random);
    println!("Deleting: {}", items[0]);
    splay_tree.delete(items[0]);
    splay_tree.print_tree();
    println!("Tree height: {}", splay_tree.height());


}

fn collect_data() {
    // file setup
    let mut file = File::options().append(true).open("./data/trees.csv").unwrap();
    file.write_all(b"tree;type;n;cmp;rns;h\n").unwrap();
    // tree setup
    let mut bst_tree: Box<BinarySearchTree>;
    let mut rb_tree: RBTree;
    let mut splay_tree: SplayTree;
    //experiment
    let mut inserts: Vec<u32>;
    let mut deletes: Vec<u32>;
    for n in (10_000..=100_000).step_by(10_000) {
        let mut maxcmps: (u32, u32, u32)  = (0, 0, 0);
        let mut maxrns: (u32, u32, u32)  = (0, 0, 0);

        let mut cmpssum: (u32, u32, u32) = (0, 0, 0);
        let mut rnssum: (u32, u32, u32) = (0, 0, 0);

        let mut hsum: (u32, u32, u32) = (0, 0, 0);
        let mut hmax: (u32, u32, u32) = (0, 0, 0);

        let count = 4 * n * 20;

        for _ in 0..20 {
            println!("{n}");
            bst_tree = Box::new(BinarySearchTree::new());
            rb_tree = RBTree::new();
            splay_tree = SplayTree::new();

            inserts = gen_list(n, Order::Sorted);
            for insert in &inserts {
                bst_tree.insert(*insert);
                cmpssum.0 += get_cmps();
                rnssum.0 += get_rns();
                hsum.0 += bst_tree.height();
                maxcmps.0 = maxcmps.0.max(get_cmps());
                maxrns.0 = maxrns.0.max(get_rns());
                hmax.0 = hmax.0.max(bst_tree.height());

                rb_tree.insert(*insert);
                cmpssum.1 += get_cmps();
                rnssum.1 += get_rns();
                hsum.1 += rb_tree.height();
                maxcmps.1 = maxcmps.1.max(get_cmps());
                maxrns.1 = maxrns.1.max(get_rns());
                hmax.1 = hmax.1.max(rb_tree.height());

                splay_tree.insert(*insert);
                cmpssum.2 += get_cmps();
                rnssum.2 += get_rns();
                hsum.2 += splay_tree.height();
                maxcmps.2 = maxcmps.2.max(get_cmps());
                maxrns.2 = maxrns.2.max(get_rns());
                hmax.2 = hmax.2.max(splay_tree.height());
            }
            deletes = gen_list(n,  Order::Random);
            for delete in &deletes {
                bst_tree.delete(*delete);
                cmpssum.0 += get_cmps();
                rnssum.0 += get_rns();
                hsum.0 += bst_tree.height();
                maxcmps.0 = maxcmps.0.max(get_cmps());
                maxrns.0 = maxrns.0.max(get_rns());
                hmax.0 = hmax.0.max(bst_tree.height());

                rb_tree.delete(*delete);
                cmpssum.1 += get_cmps();
                rnssum.1 += get_rns();
                hsum.1 += rb_tree.height();
                maxcmps.1 = maxcmps.1.max(get_cmps());
                maxrns.1 = maxrns.1.max(get_rns());
                hmax.1 = hmax.1.max(rb_tree.height());

                splay_tree.delete(*delete);
                cmpssum.2 += get_cmps();
                rnssum.2 += get_rns();
                hsum.2 += splay_tree.height();
                maxcmps.2 = maxcmps.2.max(get_cmps());
                maxrns.2 = maxrns.2.max(get_rns());
                hmax.2 = hmax.2.max(splay_tree.height());
            };
            bst_tree = Box::new(BinarySearchTree::new());
            rb_tree = RBTree::new();
            splay_tree = SplayTree::new();
            
            inserts = gen_list(n, Order::Random);
            for insert in &inserts {
                bst_tree.insert(*insert);
                cmpssum.0 += get_cmps();
                rnssum.0 += get_rns();
                hsum.0 += bst_tree.height();
                maxcmps.0 = maxcmps.0.max(get_cmps());
                maxrns.0 = maxrns.0.max(get_rns());
                hmax.0 = hmax.0.max(bst_tree.height());

                rb_tree.insert(*insert);
                cmpssum.1 += get_cmps();
                rnssum.1 += get_rns();
                hsum.1 += rb_tree.height();
                maxcmps.1 = maxcmps.1.max(get_cmps());
                maxrns.1 = maxrns.1.max(get_rns());
                hmax.1 = hmax.1.max(rb_tree.height());

                splay_tree.insert(*insert);
                cmpssum.2 += get_cmps();
                rnssum.2 += get_rns();
                hsum.2 += splay_tree.height();
                maxcmps.2 = maxcmps.2.max(get_cmps());
                maxrns.2 = maxrns.2.max(get_rns());
                hmax.2 = hmax.2.max(splay_tree.height());
            }
            deletes = gen_list(n,  Order::Random);
            for delete in &deletes {
                bst_tree.delete(*delete);
                cmpssum.0 += get_cmps();
                rnssum.0 += get_rns();
                hsum.0 += bst_tree.height();
                maxcmps.0 = maxcmps.0.max(get_cmps());
                maxrns.0 = maxrns.0.max(get_rns());
                hmax.0 = hmax.0.max(bst_tree.height());

                rb_tree.delete(*delete);
                cmpssum.1 += get_cmps();
                rnssum.1 += get_rns();
                hsum.1 += rb_tree.height();
                maxcmps.1 = maxcmps.1.max(get_cmps());
                maxrns.1 = maxrns.1.max(get_rns());
                hmax.1 = hmax.1.max(rb_tree.height());

                splay_tree.delete(*delete);
                cmpssum.2 += get_cmps();
                rnssum.2 += get_rns();
                hsum.2 += splay_tree.height();
                maxcmps.2 = maxcmps.2.max(get_cmps());
                maxrns.2 = maxrns.2.max(get_rns());
                hmax.2 = hmax.2.max(splay_tree.height());
            }
        }
        file.write_all(format!("bst;avg;{n};{};{};{}\n", cmpssum.0/count, rnssum.0/count, hsum.0/count).as_bytes()).unwrap();
        file.write_all(format!("rb;avg;{n};{};{};{}\n", cmpssum.1/count, rnssum.1/count, hsum.1/count).as_bytes()).unwrap();
        file.write_all(format!("slay;avg;{n};{};{};{}\n", cmpssum.2/count, rnssum.2/count, hsum.2/count).as_bytes()).unwrap();

        file.write_all(format!("bst;max;{n};{};{};{}\n", maxcmps.0, maxrns.0, hmax.0).as_bytes()).unwrap();
        file.write_all(format!("rb;max;{n};{};{};{}\n", maxcmps.1, maxrns.1, hmax.1).as_bytes()).unwrap();
        file.write_all(format!("slay;max;{n};{};{};{}\n", maxcmps.2, maxrns.2, hmax.2).as_bytes()).unwrap();
    }
}
