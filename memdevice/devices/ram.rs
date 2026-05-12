use std::{collections::BTreeMap, ptr::NonNull, sync::Mutex};

use balanced_ternary::{Ternary, Trit};

use crate::{Addr, MemoryDevice, Tryte, Word, devices::ram::allocator::TernaryPageAllocator};

mod allocator;

// 729 Words (2187 Trytes) per Page
const PAGE_SIZE: usize = 3usize.pow(6);
const PAGE_SIZE_TRYTES: usize = 3usize.pow(7);

#[derive(Clone, Copy)]
struct TernaryPage([Word; PAGE_SIZE]);

//... This probably isn't ideal but this works.
// TODO: Test extensively with miri.
static ALLOCATOR: Mutex<TernaryPageAllocator> = Mutex::new(TernaryPageAllocator::new());

type Partial = Ternary<20>;
type Index   = Ternary<7>;

// I really don't want to mutex Ram, but if I do multithreading, it may be necessary
#[derive(Debug)]
pub struct Ram {
    memory: BTreeMap<Partial, NonNull<TernaryPage>>,
    base: Word,
}

impl MemoryDevice for Ram {
    fn write_tryte(&mut self, offset: Addr, value: Tryte) {
        self.set_tryte(offset + self.base, value);
    }

    fn write_word(&mut self, offset: Addr, value: Word) {
        self.set_word(offset + self.base, value);
    }

    fn read_tryte(&mut self, offset: Addr) -> Tryte {
        self.get_tryte(offset + self.base)
    }

    fn read_word(&mut self, offset: Addr) -> Word {
        self.get_word(offset + self.base)
    }
}

impl Ram {
    pub fn new(base: Word) -> Ram {
        Ram { memory: BTreeMap::new(), base }        
    }

    fn get_word(&mut self, addr: Addr) -> Word {
        let partial: Partial = dbg!(addr.slice(7));
        let val = dbg!(self.memory.entry(partial)).or_insert_with( ||
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc(),
        );

        let index: Index = addr.slice(0);
        if index.get_trit(0) != Trit::ZERO { panic!("unaligned read") }
        let index: isize = index.into();
        let index: usize = (index + (PAGE_SIZE as isize / 2)) as usize;

        unsafe {
            val.as_mut().0[index]
        }
    }

    fn set_word(&mut self, addr: Addr, word: Word) {
        let partial: Partial = dbg!(addr.slice(7));
        let val = dbg!(self.memory.entry(partial)).or_insert_with( ||
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc(),
        );

        let index: Index = addr.slice(0);
        if index.get_trit(0) != Trit::ZERO { panic!("unaligned read") }
        let index: isize = dbg!(index.into());
        let index: usize = (index + (PAGE_SIZE as isize / 2)) as usize;

        unsafe {
            val.as_mut().0[index] = word;
        }
    }

    fn get_tryte(&mut self, addr: Addr) -> Tryte {
        let partial: Partial = dbg!(addr.slice(7));
        let val = dbg!(self.memory.entry(partial)).or_insert_with( ||
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc(),
        );

        let index: Index = dbg!(addr.slice(0));
        let index: isize = dbg!(index.into());
        let index: usize = dbg!(((index / 3) + (PAGE_SIZE as isize / 2)) as usize);

        let slice_ind = dbg!(index % 3);

        unsafe {
            val.as_mut().0[dbg!(index)].slice(9 * slice_ind as u32)
        }
    }

    fn set_tryte(&mut self, addr: Addr, tryte: Tryte) {
        let partial: Partial = dbg!(addr.slice(7));
        let val = dbg!(self.memory.entry(partial)).or_insert_with( ||
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc(),
        );

        let index: Index = dbg!(addr.slice(0));
        let index: isize = dbg!(index.into());
        let index: usize = dbg!(((index / 3) + (PAGE_SIZE as isize / 2)) as usize);

        let slice_ind = dbg!(index % 3);

        unsafe {
            let val = &mut val.as_mut().0[dbg!(index)];
            *val = val.set(tryte, 9 * slice_ind as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use balanced_ternary::Trit;

    use crate::{MemoryDevice, Tryte, Word, devices::ram::Ram};

    #[test]
    fn test_memory() {
        let mut ram = Ram::new(Word::ZERO);
        ram.write_word(Word::ZERO, Word::MAX);
        assert_eq!(ram.read_word(Word::ZERO), Word::MAX);
    }

    #[test]
    fn test_tryte() {
        let mut ram = Ram::new(Word::ZERO);
        for i in -1093..=1093 {
            let i: Tryte = i.try_into().unwrap();
            ram.write_tryte(i.extend(), i);
            println!();
        }
        println!();
        for i in -1093..=1093 {
            let i: Tryte = i.try_into().unwrap();
            println!("i: {i}");
            assert_eq!(dbg!(ram.read_tryte(i.extend())), dbg!(i));
        }
    }
}
