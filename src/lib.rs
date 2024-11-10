//! Linked lists that supports arbitrary removal in constant time.
//!
//! It is based on the linked list implementation in [Rust-for-Linux][1].
//!
//! [1]: https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/linked_list.rs

#![no_std]

mod linked_list;
pub mod raw_list;
pub use linked_list::{GetLinksWrapped, List, Wrapper};
pub use raw_list::{Cursor, GetLinks, Links};
