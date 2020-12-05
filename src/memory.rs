use x86_64::{
    structures::paging::PageTable,
    VirtAddr
};

// Returns a mutable reference to the active level 4 table.

// This function is unsafe because the caller must guarantee that the
// complete physical memory is mapped to virtual memory at the passed
// `physical_memory_offset`.
// Also, it must be only called once
// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
                                   -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;    // level-4 table register

    let (level_4_table_frame, _) = Cr3::read();

    let phys_ = level_4_table_frame.start_address();
    let virtual_ = physical_memory_offset + phys_.as_u64();
    let page_table_ptr: *mut PageTable = virtual_.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}