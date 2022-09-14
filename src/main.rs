#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust_tutorial::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_in_rust_tutorial::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_in_rust_tutorial::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    os_in_rust_tutorial::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}
