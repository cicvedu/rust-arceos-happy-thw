//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator{
    start:usize,
    end:usize,
    next:usize,
    allocation:usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start:0,
            end:0,
            next:0,
            allocation:0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.end = _start + _size;
        self.next = _start;
        self.allocation = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        Err(crate::AllocError::NoMemory)
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = _layout.size();
        let align = _layout.align();
        let align_mask = !(align - 1);

        let start = (self.next + align - 1) & (align_mask);

        if start + size > self.end{
            Err(crate::AllocError::NoMemory)
        } else{
            self.allocation += 1;
            self.next = start + size;
            Ok(NonZeroUsize::new(start).unwrap())
        }
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocation -= 1;
        if self.allocation == 0{
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.next - self.start
    }

    fn available_bytes(&self) -> usize {
        self.end - self.next
    }
}
