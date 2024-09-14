#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;

mod vga_buffer;

#[allow(clippy::all)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    os::init();

    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    loop {}
}

#[allow(clippy::all)]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
