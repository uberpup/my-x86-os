#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_x86_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use my_x86_os::println;
use my_x86_os::test_panic_handler;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    my_x86_os::init();

    //x86_64::instructions::interrupts::int3();   // invoking breakpoint exception

    /*unsafe {
        *(0xdeadbeef as *mut u64) = 1337;   // invoking double-fault
    }*/

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {

    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
