pub mod hash;
pub mod structure;

pub mod prelude {
    pub use crate::hash::string_hash;
    pub use crate::structure::StringTable;
    pub use crate::string_table;
}

/// A helper macro to easily create a StringTable.
/// # Example
/// ```rust
/// # use string_table::prelude::*;
/// let table = string_table! {
///     "hello" => "world!",
/// };
/// ```
#[macro_export]
macro_rules! string_table {
    ($(
        $key:expr => $value:expr
    ),* $(,)?) => {
        {
            let mut table = string_table::prelude::StringTable::new();
            $(
                table.insert($key, $value);
            )*
            table
        }
    }
}