#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_x86_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use my_x86_os::println;
use my_x86_os::test_panic_handler;
use x86_64::VirtAddr;
use my_x86_os::memory;
use alloc::boxed::Box;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use my_x86_os::memory;
    use x86_64::{structures::paging::{Page, MapperAllSizes}, VirtAddr};
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
    let mut mapper = unsafe { memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
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

    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04)};

    let x = Box::new(42);

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
