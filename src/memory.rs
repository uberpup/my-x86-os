use x86_64::{
    structures::paging::PageTable,
    structures::paging::OffsetPageTable,
    structures::paging::{Page, PhysFrame, Mapper, Size4KiB, FrameAllocator},
    VirtAddr,
    PhysAddr
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct EmptyFrameAllocator;

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize
}

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        return None;
    }
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        return BootInfoFrameAllocator {
            memory_map,
            next: 0
        }
    }
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses= addr_ranges.flat_map(|r| r.step_by(4096)); // Current PageSize
        return frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        return frame
    }
}

pub unsafe fn init(phys_mem_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(phys_mem_offset);
    return OffsetPageTable::new(level_4_table, phys_mem_offset);
}

pub fn create_example_mapping(page: Page, mapper: &mut OffsetPageTable,
                              frame_allocator: &mut impl FrameAllocator<Size4KiB>) {
    use x86_64::structures::paging::PageTableFlags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let map_to_result = unsafe {
        // FIXME not safe. only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    return map_to_result.expect("map_to failed").flush();
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