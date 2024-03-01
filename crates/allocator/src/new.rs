#![no_std]
use super::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;
use core::slice::from_raw_parts;
use linked_list_allocator::Heap;

pub struct LinkedListAllocator {
    inner: Option<Heap>,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self { inner: None }
    }

    fn inner_mut(&mut self) -> &mut Heap {
        self.inner.as_mut().unwrap()
    }

    fn inner(&self) -> &Heap {
        self.inner.as_ref().unwrap()
    }
}

impl BaseAllocator for LinkedListAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.inner = unsafe { Some(Heap::new(start as *mut u8, size)) };
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        unsafe {
            self.inner_mut().extend(3 * size); //Small extensions are not guaranteed
                                               //to grow the usable size of the heap. In order to grow the Heap most effectively,
                                               //extend by at least 2 * size_of::<usize>,
                                               //keeping the amount a multiple of size_of::<usize>.
        }
        Ok(())
    }
}

impl ByteAllocator for LinkedListAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        self.inner_mut()
            .allocate_first_fit(layout)
            .map_err(|_| AllocError::NoMemory)
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        unsafe { self.inner_mut().deallocate(pos, layout) }
    }

    fn total_bytes(&self) -> usize {
        self.inner().size() as usize
    }

    fn used_bytes(&self) -> usize {
        let val = unsafe { *self.inner().top() - *self.inner().bottom() } as usize;
        val
    }

    fn available_bytes(&self) -> usize {
        self.inner().free()
    }
}
