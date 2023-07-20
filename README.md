![Crates.io](https://img.shields.io/crates/v/string_table)
![License](https://img.shields.io/crates/l/string_table)
![Docs.rs](https://docs.rs/string_table/badge.svg)

# String Table
`string_table` is a crate for Rust that allows for speedy String-based HashMaps.

## Usage
Run the following command:
```sh
$ cargo add string_table
```
Or add the following to your `Cargo.toml`:

```toml
[dependencies]
string_table = "1.0.0"
```

## Example
```rs
let mut table = StringTable::new();
table.insert("a", 10i32);
table.insert("b", 20i32);
table.insert("c", 30i32);

assert!(table.get("a").unwrap() == &10);
assert!(table.get("b").unwrap() == &20);
assert!(table.get("c").unwrap() == &30);
```

## License
This crate is licensed under the `Apache License 2.0`.