use x86_64::{ structures::paging::PageTable, VirtAddr, structures::paging::OffsetPageTable };

/// Returns the entry point of the level 4 page-table.
unsafe fn get_page_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr    // unsafe
}

/// Init OffsetPageTable.
pub unsafe fn init_opt(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = get_page_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}