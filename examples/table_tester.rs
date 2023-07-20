use string_table::prelude::*;

fn main() {
    let mut table = StringTable::new();
    table.insert("a", 10i32);
    table.insert("b", 20i32);
    table.insert("c", 30i32);

    assert!(table.get("a").unwrap() == &10);
    assert!(table.get("b").unwrap() == &20);
    assert!(table.get("c").unwrap() == &30);

    // Example of invoking the macro
    let mut second_table = string_table! {
        "hello" => "world",
    };

    assert!(second_table.get("hello") == Some(&"world"));
}