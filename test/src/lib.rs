#![deny(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![no_std]

use badalloc::BadAlloc;

#[global_allocator]
static ALLOC: BadAlloc = BadAlloc;

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;
    use core::alloc::Layout;
    use core::mem::size_of;

    #[test]
    fn sizes() {
        for (l, s) in [
            (Layout::new::<u8>(), 1),
            (Layout::new::<u32>(), 4),
            (Layout::new::<[u8; 12]>(), 12),
            (
                Layout::new::<Vec<u128>>(),
                size_of::<usize>() + size_of::<u128>(),
            ),
        ] {
            assert_eq!(l.size(), s);
        }
    }

    #[test]
    fn string() {
        let _msg = "hello".to_string();
    }

    #[test]
    fn vec() {
        let mut vec = vec![0_u16, 1];
        // hopefully force the vec to reallocate
        vec.append(&mut vec![2, 3, 4, 5, 6, 7]);
    }
}
