#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::{init, println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    loop {}
}

#[test_case]
fn test_println() {
    println!("test println output");
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic(info);
}