use std::{alloc::{self, Layout}, cell::Cell, mem::MaybeUninit, ptr::NonNull, sync::{Arc, Mutex}};

use crate::devices::ram::TernaryPage;

pub struct TernaryPageAllocator {
    head: Cell<Option<NonNull<Slab>>>,
}

unsafe impl Send for TernaryPageAllocator {}

impl TernaryPageAllocator {
    pub const fn new() -> Self {
        Self { head: Cell::new(None) }
    }

    pub fn alloc(&self) -> NonNull<TernaryPage> {
        if self.head.get().is_none() {
            self.head.replace(Some(alloc_slab()));
        }

        let curr = self.head.get().unwrap();

        get_ternpage(curr)
    }
}

fn get_ternpage(mut val: NonNull<Slab>) -> NonNull<TernaryPage> {
    let mut_ref = unsafe {
         val.as_mut()
    };
    if mut_ref.full() {
        match mut_ref.next {
            Some(slab) => become get_ternpage(slab),
            None => {
                mut_ref.next = Some(alloc_slab());
                become get_ternpage(mut_ref.next.unwrap())
            },
        }
    } else {
        mut_ref.alloc_ternpage().unwrap()
    }
}

/// This is not actually a slab allocator so... but it is a preallocated slab that I can
/// allocate chunks of...
#[repr(C)]
struct Slab {
    ternpages: [MaybeUninit<TernaryPage>; TERNPAGE_PER_LARGEPAGE],
    index: usize,
    next: Option<NonNull<Slab>>,
    _reserved: [u8; RES_SIZE],
}

fn alloc_slab() -> NonNull<Slab> {
    unsafe {
        // SAFETY: We allocated a zeroed slab.
        // A slab has 4 fields:
        // - ternpages: `[MaybeUninit<TernaryPage>]` valid when zero allocated
        // - index: an integer, which should start as 0
        // - next: `Option<NonNull<Slab>>` initalized to 0, which is the `None` variant, and therefore
        //   valid and correct.
        let ptr = alloc::alloc_zeroed(
            Layout::from_size_align(LARGEPAGE_BYTES, LARGEPAGE_BYTES)
                .expect("Invalid Layout")
        );
        NonNull::new(ptr.cast()).expect("Could not allocate a large page")
    }
}

impl Slab {
    fn alloc_ternpage(&mut self) -> Option<NonNull<TernaryPage>> {
        println!("ALLOCATING TERNPAGE");
        if self.full() {
            None
        } else {
            let ref_page = &raw mut self.ternpages[self.index];
            self.index += 1;
            unsafe {
                // SAFETY: 0 Initalized is a valid state for [`Ternary`], and therefore
                // an array of them.
                ref_page.write_bytes(0, 1);
                let page_ptr: *mut TernaryPage = ref_page.cast();
                // SAFETY?: Points to internal value.
                // (There... may be aliasing concerns here)
                // Options: Allocated slab pointers instead point
                // to a slab header, which can be cast into a pointer to
                // the full struct when needed? This is ripe for *some kind* of UB
                // A formal model for UB would help here.
                Some(NonNull::new_unchecked(page_ptr))
            }
        }
    }

    fn full(&self) -> bool {
        self.index == TERNPAGE_PER_LARGEPAGE
    }
}

const RES_SIZE: usize = {
    LARGEPAGE_BYTES -
    (TERNPAGE_BYTES * TERNPAGE_PER_LARGEPAGE) -
    size_of::<usize>() -
    size_of::<Option<NonNull<Slab>>>()
};

const _: () = assert!(size_of::<Slab>() == LARGEPAGE_BYTES);

pub struct Ram<const PAGES: usize> {
    pages: ()
}

const LARGEPAGE_BYTES: usize = 2usize.pow(31);
const TERNPAGE_BYTES: usize = size_of::<TernaryPage>();
const TERNPAGE_PER_LARGEPAGE: usize = LARGEPAGE_BYTES / TERNPAGE_BYTES;
const LEFTOVER_BYTES: usize = LARGEPAGE_BYTES % TERNPAGE_BYTES;

#[cfg(test)]
mod tests {
    use super::alloc_slab;

    #[test]
    fn alloc() {
        let nonnull = alloc_slab();
    }

    // TODO: Add some slab allocation tests
}
