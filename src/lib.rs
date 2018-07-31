//! A simple kernel module in Rust
#![feature(lang_items)]
#![deny(missing_docs, warnings)]
#![no_std]

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
