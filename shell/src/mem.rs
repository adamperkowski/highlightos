use x86_64::{structures::paging::PageTable, VirtAddr};

// Returns a mutable reference to the active level 4 table.
pub unsafe fn active_lvl_4_table(ph_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (lvl_4_table_frame, _) = Cr3::read();

    let phys = lvl_4_table_frame.start_address();
    let virt = ph_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe return
}
