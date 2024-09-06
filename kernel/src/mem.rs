use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

#[allow(clippy::missing_safety_doc)]
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let lvl_4_table = active_lvl_4_table(physical_memory_offset);
    OffsetPageTable::new(lvl_4_table, physical_memory_offset)
}

// Returns a mutable reference to the active level 4 table.
unsafe fn active_lvl_4_table(ph_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (lvl_4_table_frame, _) = Cr3::read();

    let phys = lvl_4_table_frame.start_address();
    let virt = ph_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe return
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // FIXME: this is not safe
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootInfoFrameAlloc {
    memory_map: &'static MemoryMap,
    next: usize,
}

#[allow(clippy::missing_safety_doc)]
impl BootInfoFrameAlloc {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAlloc { memory_map, next: 0 }
    }
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
