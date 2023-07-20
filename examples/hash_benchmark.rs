use std::hint::black_box;

use random_string::generate;
use string_table::prelude::*;

pub fn run_my_bench() {
    println!("Generating hashes..");
    let mut hashes = vec![];
    for _ in 0..100000 {
        let charset = "1234567890abcdefghjijlmnopqrstuvwxyzABCDEFGHHJIJKLNOPQRSTUVWXYZ_!;'";
        let generated = generate(1000, charset);
        hashes.push(generated);
    }
    println!("All done!");

    let now = std::time::Instant::now();

    {
        for hash in &hashes {
            black_box({ || string_hash(&hash) }());
        }
    }

    println!("Doing Iter: {:?}", now.elapsed());
    println!("Done Iter: {:?}", now.elapsed());
}

fn main() {
    run_my_bench();
}
