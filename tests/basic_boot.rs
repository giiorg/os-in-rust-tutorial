#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust_tutorial::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_in_rust_tutorial::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_in_rust_tutorial::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
