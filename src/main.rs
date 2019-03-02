// src/main.rs

#![cfg_attr(test, allow(unused_imports))] // disable compile warnings in test mode
#![cfg_attr(not(test), no_std)] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate core;

extern crate lazy_static;
extern crate spin;
extern crate volatile;

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic
/// only compile when the test flag is not set
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    println!("Hello World{}", "!");

    loop {}
}
