//! Certain language items needed with the usage of `#![no_std]`
use core;

#[no_mangle]
#[lang = "eh_personality"]
/// These functions are used by the compiler, but not for a bare-bones hello world. These are
/// normally provided by libstd.
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
#[lang = "eh_unwind_resume"]
/// This function may be needed based on the compilation target.
pub extern "C" fn rust_eh_unwind_resume() {}

#[no_mangle]
#[lang = "panic_impl"]
/// Panic entry function
pub extern "C" fn rust_begin_panic(_info: &core::panic::PanicInfo) -> ! {
    // Try to do a segementation fault
    let _ = *(core::ptr::null::<i32>());
    // Loop forever
    loop {}
}
