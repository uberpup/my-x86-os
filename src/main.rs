#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_x86_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use my_x86_os::println;
use my_x86_os::test_panic_handler;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello world!");

    my_x86_os::init();

    //x86_64::instructions::interrupts::int3();   // invoking breakpoint exception

    /*unsafe {
        *(0xdeadbeef as *mut u64) = 1337;   // invoking double-fault
    }*/

    let deadbeef_ptr = 0xdeadbeef as *mut u32;
    unsafe { *deadbeef_ptr = 42; }  // invoking page fault through illegal memory access

    #[cfg(test)]
    test_main();

    my_x86_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    my_x86_os::hlt_loop();
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
