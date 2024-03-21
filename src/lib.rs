//! # key-path
//!
//! `key-path` is a Rust crate providing a type-safe way to represent key
//! paths, allowing access to fields within a struct through these paths.
//! It offers a `KeyPath` struct, a `KeyEnter` trait, and a `key!` macro
//! to facilitate working with key paths.
//!
//! ## Features
//!
//! - Type-safe representation of key paths.
//! - Efficient access to struct fields through key paths.
//! - Support for appending key paths to navigate nested fields.
//!
//! ## Usage
//!
//! You can construct key paths using the `key!` macro and access struct fields
//! using the provided `KeyEnter` trait.
//!
//! ```rust
//! use key_path::*;
//!
//! struct MyStruct {
//!     field1: i32,
//!     field2: f64,
//! }
//!
//! struct MyNestedStruct(MyStruct);
//!
//! fn main() {
//!     let mut instance = MyStruct { field1: 42, field2: 3.14 };
//!     let path = key!(MyStruct[field1]);
//!
//!     *instance.enter_mut(path) -= 20;
//!     assert_eq!(*instance.enter(path), 22);
//!
//!     let nested_instance = MyNestedStruct(instance);
//!     let nested_path = key!(MyNestedStruct[0][field2]);
//!
//!     assert_eq!(*nested_instance.enter(nested_path), 3.14);
//! }
//! ```

mod key_enter;
mod key_path;

#[cfg(test)]
mod tests;

pub use key_enter::KeyEnter;
pub use key_path::KeyPath;
