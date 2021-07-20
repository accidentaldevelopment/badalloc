#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![no_std]

extern crate alloc;

use alloc::alloc::handle_alloc_error;
use core::alloc::{GlobalAlloc, Layout};
use libc::c_void;

pub struct BadAlloc;

// Declare `malloc` and `free`. I could have used the definitions in `libc`, but I wanted to write them out.
// c_void is harder to do without libc.
extern "C" {
    fn malloc(size: usize) -> *mut c_void;

    fn free(ptr: *mut c_void);
}

unsafe impl GlobalAlloc for BadAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = malloc(layout.size());
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr.cast()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // TODO: actually free the right memory and not just the first 8 bits.
        free(ptr.cast())
    }
}
