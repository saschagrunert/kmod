//! A simple kernel module in Rust
#![no_std]
#![feature(lang_items)]
#![deny(missing_docs)]

pub mod lang_items;

#[no_mangle]
/// Entry point for the kernel module
pub fn init_module() -> u8 {
    return 0;
}

#[no_mangle]
/// Exit point for the kernel module
pub fn cleanup_module() {}
