#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;
// mod interrupts;

use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuEC {
    Success = 0x10,
    Failed = 0x11,
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T : Fn(), {
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        print!("{}... ", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
        println!("[ok!]");
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");

    //panic!("Scary spooky panic message");

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuEC::Failed);
    loop {}
}

pub fn exit_qemu(exit_code: QemuEC) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuEC::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
