use nohash_hasher::NoHashHasher;
use std::{collections::HashMap, hash::BuildHasherDefault};

use crate::hash::string_hash;

#[derive(Clone)]
#[repr(transparent)]
/// A HashMap for Strings implemented with a special hashing algorithm.
/// See: https://doc.rust-lang.org/std/collections/struct.HashMap.html
pub struct StringTable<T> {
    inner: HashMap<u32, T, BuildHasherDefault<NoHashHasher<u32>>>,
}

impl<T> StringTable<T> {
    /// Creates an empty `StringTable`.
    /// 
    /// The table is initially created with a capacity of zero, so it will not initialize with any items.
    /// 
    /// # Example
    /// ```rust
    /// # use string_table::prelude::*;
    /// let table: StringTable<i32> = StringTable::new();
    /// ```
    pub fn new() -> Self {
        StringTable {
            inner: HashMap::with_hasher(BuildHasherDefault::default()),
        }
    }

    /// Creates an empty `StringTable` with a specified capacity.
    /// 
    /// However, it will not initialize with any items.
    /// 
    /// # Example
    /// ```rust
    /// # use string_table::prelude::*;
    /// let table: StringTable<i32> = StringTable::with_capacity(20);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        StringTable {
            inner: HashMap::with_capacity_and_hasher(capacity, BuildHasherDefault::default()),
        }
    }

    /// Inserts a key-value pair into the table.
    /// 
    /// # Example
    /// ```rust
    /// # use string_table::prelude::*;
    /// let mut table: StringTable<i32> = StringTable::with_capacity(20);
    /// table.insert("five", 5);
    /// table.insert("ten", 10);
    /// ```
    pub fn insert(&mut self, key: &str, value: T) {
        self.inner.insert(string_hash(key), value);
    }

    /// Gets a key-value pair from the table.
    /// 
    /// # Example
    /// ```rust
    /// # use string_table::prelude::*;
    /// let mut table: StringTable<i32> = StringTable::with_capacity(1);
    /// table.insert("five", 5);
    /// assert!(table.get("five") == Some(&5))
    /// ```
    pub fn get(&mut self, key: &str) -> Option<&T> {
        self.inner.get(&string_hash(key))
    }

    /// Gets a mutable reference to a key-value pair in the table.
    /// 
    /// # Example
    /// ```rust
    /// # use string_table::prelude::*;
    /// let mut table: StringTable<i32> = StringTable::new();
    /// table.insert("five", 5);
    /// assert!(table.get_mut("five") == Some(&mut 5))
    /// ```
    pub fn get_mut(&mut self, key: &str) -> Option<&mut T> {
        self.inner.get_mut(&string_hash(key))
    }

    /// Gets a key-value pair from the table, without checking if it exists or not.
    /// 
    /// For a safe alternative, see `StringTable::get`.
    /// 
    /// # Safety
    /// Invoking this function with a key that doesn't exist is undefined behavior.
    /// 
    /// # Example
    /// ```rust
    /// # use string_table::prelude::*;
    /// let mut table: StringTable<i32> = StringTable::new();
    /// table.insert("my_string", 10);
    /// unsafe {
    ///     assert!(table.get_unchecked("my_string") == &10);
    /// }
    /// ```
    pub unsafe fn get_unchecked(&mut self, key: &str) -> &T {
        self.inner.get(&string_hash(key)).unwrap_unchecked()
    }
}
