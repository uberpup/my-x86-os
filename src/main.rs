#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_x86_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use my_x86_os::println;
use my_x86_os::test_panic_handler;
use x86_64::VirtAddr;
use my_x86_os::memory;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use my_x86_os::memory;
    use x86_64::{structures::paging::MapperAllSizes, VirtAddr};
    println!("Hello world!");

    my_x86_os::init();

    //x86_64::instructions::interrupts::int3();   // invoking breakpoint exception

    /*unsafe {
        *(0xdeadbeef as *mut u64) = 1337;   // invoking double-fault
    }*/

    /*let deadbeef_ptr = 0xdeadbeef as *mut u32;
    unsafe { *deadbeef_ptr = 42; }  // invoking page fault through illegal memory access
    */

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset)};
    let addresses = [
        0xb8000, // the identity-mapped vga buffer page
        0x201008, // some code page
        0x0100_0020_1a10, // some stack page
        boot_info.physical_memory_offset, // virtual address mapped to physical 0
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt) ;
        println!("{:?} -> {:?}", virt, phys);
    }

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
