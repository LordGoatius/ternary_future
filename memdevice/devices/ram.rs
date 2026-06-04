use std::{collections::BTreeMap, ptr::NonNull, sync::Mutex};

use balanced_ternary::{Ternary, Trit};

use crate::{Addr, MemEntry, MemoryDevice, Tryte, Word, devices::ram::allocator::TernaryPageAllocator};

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
    size: Word,
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
    pub const fn new(base: Word, size: Word) -> Ram {
        Ram { memory: BTreeMap::new(), base, size }
    }

    pub const fn to_mementry(&'static mut self) -> MemEntry {
        let base = self.base;
        let size = self.size;
        MemEntry { dev: self, base, size }
    }

    fn get_word(&mut self, addr: Addr) -> Word {
        let partial: Partial = addr.slice(7);
        let val = self.memory.entry(partial).or_insert_with( ||
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
        let partial: Partial = addr.slice(7);
        let val = self.memory.entry(partial).or_insert_with( ||
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
            val.as_mut().0[index] = word;
        }
    }

    fn get_tryte(&mut self, addr: Addr) -> Tryte {
        let addr = addr - Word::ONE;
        let partial: Partial = addr.slice(7);
        let val = self.memory.entry(partial).or_insert_with( ||
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc(),
        );

        let index: Index = addr.slice(0);
        let index1: isize = index.into();
        let index: usize = (((index1 + 1).div_floor(3)) + (PAGE_SIZE as isize / 2)) as usize;

        let slice_ind = (index1 + 1).rem_euclid(3);

        unsafe {
            val.as_mut().0[index].slice(9 * slice_ind as u32)
        }
    }

    fn set_tryte(&mut self, addr: Addr, tryte: Tryte) {
        let addr = addr - Word::ONE;
        let partial: Partial = addr.slice(7);
        let val = self.memory.entry(partial).or_insert_with( ||
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc(),
        );

        let index: Index = addr.slice(0);
        let index1: isize = index.into();
        let index: usize = (((index1 + 1).div_floor(3)) + (PAGE_SIZE as isize / 2)) as usize;

        let slice_ind = (index1 + 1).rem_euclid(3);

        unsafe {
            let val = &mut val.as_mut().0[index];
            *val = val.set(tryte, 9 * slice_ind as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MemoryDevice, Tryte, Word, devices::ram::Ram};

    #[test]
    fn test_memory() {
        let mut ram = Ram::new(Word::ZERO, Word::MAX);
        ram.write_word(Word::ZERO, Word::MAX);
        assert_eq!(ram.read_word(Word::ZERO), Word::MAX);
    }

    #[test]
    fn test_tryte() {
        let mut ram = Ram::new(Word::ZERO, Word::MAX);
        for i in -1093..=1093 {
            let i: Tryte = i.try_into().unwrap();
            ram.write_tryte(i.extend(), i);
        }
        println!();
        for i in -1093..=1093 {
            let i: Tryte = i.try_into().unwrap();
            assert_eq!(ram.read_tryte(i.extend()), i);
        }
    }

    #[test]
    fn test_word() {
        let mut ram = Ram::new(Word::ZERO, Word::MAX);
        for i in -364..=364 {
            let word: Word = i.try_into().unwrap();
            ram.write_word(word, (i * 3).try_into().unwrap());
        }
        for i in -364..=364 {
            let word: Word = i.try_into().unwrap();
            assert_eq!(ram.read_word((i * 3).try_into().unwrap()), word);
        }
    }

    #[test]
    fn test_both() {
        let mut ram = Ram::new(Word::ZERO, Word::MAX);
        ram.write_tryte(Word::ZERO, Tryte::MAX);
        assert_eq!(ram.read_word(Word::ZERO), Tryte::MAX.extend());
        ram.write_tryte(Word::ONE, Tryte::MAX);
        assert_eq!(ram.read_word(Word::ZERO), Tryte::MAX.extend().set(Tryte::MAX, 9));
        ram.write_tryte(Word::TWO, Tryte::MAX);
        assert_eq!(ram.read_word(Word::ZERO), Word::MAX);
    }
}
