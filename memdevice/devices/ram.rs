use owo_colors::colors::*;
use owo_colors::AnsiColors;
use owo_colors::OwoColorize;
use std::cmp::Ordering;
use std::{collections::BTreeMap, ptr::NonNull, sync::Mutex};

use balanced_ternary::{Ternary, Trit};

use crate::{
    devices::ram::allocator::TernaryPageAllocator, Addr, MemEntry, MemoryDevice, Tryte, Word,
};

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
type Index = Ternary<7>;

// I really don't want to mutex Ram, but if I do multithreading, it may be necessary.
// I am slowly rediscovering cache coherence protocols. I find it likely this becomes
// a bottleneck.
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

fn print_page((start, page): (&Partial, &NonNull<TernaryPage>)) {
    let page = unsafe { &(*page).as_ref().0 };
    let offset: Word = (PAGE_SIZE as isize / 2).try_into().unwrap();
    let mut start: Word = (start.extend() << 7) - offset;
    for i in 0..(PAGE_SIZE / 3) {
        print!("0s{start:b}: ");
        for j in 0..3 {
            let word: Word = page[(i * 3) + j];
            let one: Tryte = word.slice::<9>(0);
            let two: Tryte = word.slice::<9>(9);
            let thr: Tryte = word.slice::<9>(18);
            let one_color = match one.cmp(&Tryte::ZERO) {
                Ordering::Equal => AnsiColors::White,
                Ordering::Greater => AnsiColors::Green,
                Ordering::Less => AnsiColors::Red,
            };
            let two_color = match two.cmp(&Tryte::ZERO) {
                Ordering::Equal => AnsiColors::White,
                Ordering::Greater => AnsiColors::Green,
                Ordering::Less => AnsiColors::Red,
            };
            let thr_color = match thr.cmp(&Tryte::ZERO) {
                Ordering::Equal => AnsiColors::White,
                Ordering::Greater => AnsiColors::Green,
                Ordering::Less => AnsiColors::Red,
            };
            print!(
                "{:X} {:X} {:X}  ",
                one.color(one_color),
                two.color(two_color),
                thr.color(thr_color)
            );
        }
        println!();
        start += Word::THREE;
    }
}

impl Ram {
    pub fn print_ram(&self) {
        self.memory.iter().for_each(print_page);
    }

    pub const fn new(base: Word, size: Word) -> Ram {
        Ram {
            memory: BTreeMap::new(),
            base,
            size,
        }
    }

    pub const fn to_mementry(&'static mut self) -> MemEntry {
        let base = self.base;
        let size = self.size;
        MemEntry {
            dev: self,
            base,
            size,
        }
    }

    fn get_word(&mut self, addr: Addr) -> Word {
        let partial: Partial = addr.slice(7);
        let val = self.memory.entry(partial).or_insert_with(|| {
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc()
        });

        let index: Index = addr.slice(0);
        if index.get_trit(0) != -Trit::ONE {
            panic!("unaligned read")
        }
        let index: isize = (index >> 1).into();
        let index: usize = (index + (PAGE_SIZE as isize / 2)) as usize;

        unsafe { val.as_mut().0[index] }
    }

    fn set_word(&mut self, addr: Addr, word: Word) {
        let partial: Partial = addr.slice(7);
        let val = self.memory.entry(partial).or_insert_with(|| {
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc()
        });

        let index: Index = addr.slice(0);
        if index.get_trit(0) != -Trit::ONE {
            panic!("unaligned read")
        }
        let index: isize = (index >> 1).into();
        let index: usize = (index + (PAGE_SIZE as isize / 2)) as usize;

        unsafe {
            val.as_mut().0[index] = word;
        }
    }

    fn get_tryte(&mut self, addr: Addr) -> Tryte {
        let partial: Partial = addr.slice(7);
        let val = self.memory.entry(partial).or_insert_with(|| {
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc()
        });

        let (index, slice_index) = translate_tryte_index(addr.slice(0));

        unsafe { val.as_mut().0[index].slice(9 * slice_index as u32) }
    }

    fn set_tryte(&mut self, addr: Addr, tryte: Tryte) {
        let partial: Partial = addr.slice(7);
        let val = self.memory.entry(partial).or_insert_with(|| {
            ALLOCATOR
                .lock()
                .expect("Unable to Lock Memory Allocator")
                .alloc()
        });

        let (index, slice_index) = translate_tryte_index(addr.slice(0));

        unsafe {
            let val = &mut val.as_mut().0[index];
            *val = val.set(tryte, 9 * slice_index as u32);
        }
    }
}

fn translate_tryte_index(index: Index) -> (usize, isize) {
    let index1: isize = index.into();
    let index: usize = (((index1 + 1).div_euclid(3)) + (PAGE_SIZE as isize / 2)) as usize;

    let slice_index = (index1 + 1).rem_euclid(3);
    (index, slice_index)
}

#[cfg(test)]
mod tests {
    use crate::{devices::ram::Ram, Addr, MemoryDevice, Tryte, Word};

    #[test]
    fn test_tryte() {
        let mut ram = Ram::new(Word::ZERO, Word::MAX);
        for i in -1093..=1093 {
            let i: Tryte = i.try_into().unwrap();
            ram.write_tryte(i.extend(), i);
        }
        ram.print_ram();
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
            let addr: Addr = (i * 3).try_into().unwrap();
            let addr = addr - Addr::ONE;
            eprintln!("Addr: {addr}");
            ram.write_word(addr, word);
        }
        for i in -364..=364 {
            let word: Word = i.try_into().unwrap();
            let addr: Addr = (i * 3).try_into().unwrap();
            let addr = addr - Addr::ONE;
            assert_eq!(ram.read_word(addr), word);
        }
    }

    #[test]
    fn test_both() {
        let mut ram = Ram::new(Word::ZERO, Word::MAX);
        ram.write_tryte(Word::ZERO, Tryte::MAX);
        assert_eq!(ram.read_word(-Word::ONE), (Tryte::MAX << 9).extend());
        ram.write_tryte(-Word::ONE, Tryte::MAX);
        ram.print_ram();
        assert_eq!(
            ram.read_word(-Word::ONE),
            Tryte::MAX.extend().set(Tryte::MAX, 9)
        );
        ram.write_tryte(Word::ONE, Tryte::MAX);
        assert_eq!(ram.read_word(-Word::ONE), Word::MAX);
    }
}
