#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T : Fn(), {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        // print!("{}... ", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
        // println!("[ok!]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuEC {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuEC) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    // println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuEC::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuEC::Failed);
    loop {}
}

// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init(); // new
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn init() {
    interrupts::init_idt();
}