//! A simple kernel module in Rust
#![no_std]
#![allow(unused_features)]
#![deny(missing_docs, warnings)]
#![feature(lang_items, core_str_ext)]

#[macro_use]
mod print;
pub mod lang_items;

#[no_mangle]
/// Entry point for the kernel module
pub fn init_module() -> i32 {
    println!("Module initialized.");
    return 0;
}

#[no_mangle]
/// Exit point for the kernel module
pub fn cleanup_module() {
    println!("Module exit.");
}
