use std::collections::HashMap;

use string_table::structure::StringTable;

fn main() {
    for counter in 1..5 {
        println!("Run {counter};");
        println!("Benchmarking HashMap<String, i32>");
        let now = std::time::Instant::now();
        {
            let mut map = HashMap::new();
            for x in 1..1000000 {
                map.insert(format!("{x}"), x);
            }
        }
        let end = now.elapsed();
        println!("Time taken: {end:?}");
        
        println!("Benchmarking StringTable<i32>");
        let now = std::time::Instant::now();
        {
            let mut table = StringTable::new();
            for x in 1..1000000 {
                table.insert(&format!("{x}"), x);
            }
        }
        let end = now.elapsed();
        println!("Time taken: {end:?}");
        println!();
    }
}