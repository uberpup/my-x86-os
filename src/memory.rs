use x86_64::{
    structures::paging::PageTable,
    structures::paging::OffsetPageTable,
    VirtAddr,
    PhysAddr
};

pub unsafe fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(phys_mem_offset);
    return OffsetPageTable::new(level_4_table, phys_mem_offset);
}

// Returns a mutable reference to the active level 4 table.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
                                   -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;    // level-4 table register

    let (level_4_table_frame, _) = Cr3::read();

    let phys_ = level_4_table_frame.start_address();
    let virtual_ = physical_memory_offset + phys_.as_u64();
    let page_table_ptr: *mut PageTable = virtual_.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

// Translates given virtual address to mapped physical
pub unsafe fn translate_addr(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
    return translate_addr_inner(addr, phys_mem_offset);
}

// Private safe function of 'translate_addr'
fn translate_addr_inner(addr: VirtAddr, phys_mem_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    for &id in &table_indexes {
        // frame -> page table ref
        let virt = phys_mem_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        // updating frame
        let entry = &table[id];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages unsupported")
        };
    }
    return Some(frame.start_address() + u64::from(addr.page_offset()));
}