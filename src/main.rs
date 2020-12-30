#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Kernel Stated!");
    println!("Kernel written by junho ahn");

    rust_os::init();
    println!("init interrupt table");
    
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic(info);
}

