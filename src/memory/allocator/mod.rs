use x86_64::{structures::paging::{Page, FrameAllocator, Size4KiB, mapper::MapToError, Mapper, PageTableFlags}, VirtAddr};
use super::{HEAP_SIZE, HEAP_START};

pub mod frame_allocator;
pub mod bump_allocator;
pub mod linked_list_allocator;
pub mod fixed_size_block_allocator;
use bootloader::BootInfo;

/**
 * Allocator Types
 */


//      Bump Allocator
use bump_allocator::BumpAllocator;
#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

//      Linked-List Allocator
// use linked_list_allocator::LinkedListAllocator;
// #[global_allocator]
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

//      Fixed-Size-Block Allocator
// use fixed_size_block_allocator::FixedSizeBlockAllocator;
// #[global_allocator]
// static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());


/// Init kernel heap.
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    // Map all kernel heap pages to physical frames
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Init frame and basic allocators.
pub fn init_allocators(boot_info: &'static BootInfo){
    use super::offset_page_table::init_opt;
    use frame_allocator::BootInfoFrameAllocator;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { init_opt(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Basic Allocator
    init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
}