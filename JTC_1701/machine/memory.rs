use memdevice::{Addr, MemEntry};

use crate::instr::{Tryte, Word};

#[derive(Default, Debug)]
pub struct Memory {
    /// MemEntries must be sorted
    memspace: &'static mut [MemEntry],
}

impl Memory {
    pub fn from_slice(memspace: &'static mut [MemEntry]) -> Self {
        Self { memspace }
    }

    pub fn from_vec(vec: Vec<MemEntry>) -> Self {
        todo!()
    }

    fn find_device_index(&self, addr: Addr) -> Option<usize> {
        self.memspace
            .binary_search_by(|val| val.valid_addr(addr))
            .ok()
    }

    pub fn find_device_mut(&mut self, addr: Addr) -> Option<&mut MemEntry> {
        self.find_device_index(addr).map(|val| &mut self.memspace[val])
    }

    pub fn find_device(&self, addr: Addr) -> Option<&MemEntry> {
        self.find_device_index(addr).map(|val| &self.memspace[val])
    }

    pub fn write_tryte(&mut self, addr: Addr, value: Tryte) -> Option<()> {
        self.find_device_mut(addr).map(|dev| {
            dev.write_tryte(addr - dev.base, value);
        })
    }

    pub fn write_word(&mut self, addr: Addr, value: Word) -> Option<()> {
        self.find_device_mut(addr).map(|dev| {
            dev.write_word(addr - dev.base, value);
        })
    }

    pub fn read_tryte(&self, addr: Addr) -> Option<Tryte> {
        self.find_device(addr).map(|dev| {
            dev.read_tryte(addr - dev.base)
        })
    }

    pub fn read_word(&self, addr: Addr) -> Option<Word> {
        self.find_device(addr).map(|dev| {
            dev.read_word(addr - dev.base)
        })
    }
}

#[cfg(test)]
pub mod tests {
    use memdevice::{Addr, MemEntry, MemoryDevice};

    use crate::instr::{Tryte, Word};
    use crate::machine::memory::Memory;

    #[derive(Debug)]
    struct Dummy;
    impl MemoryDevice for Dummy {
        fn write_tryte(&mut self, offset: Addr, value: Tryte) {
            println!("write_tryte called: offset {offset}, value {value}");
        }

        fn write_word(&mut self, offset: Addr, value: Word) {
            println!("write_word called: offset {offset}, value {value}");
        }

        fn read_tryte(&self, offset: Addr) -> Tryte {
            println!("read_tryte called: offset {offset}");
            Tryte::ZERO
        }

        fn read_word(&self, offset: Addr) -> Word {
            println!("read_word called: offset {offset}");
            Word::ZERO
        }
    }

    #[test]
    fn test_device_index() {
        let memspace = Box::leak(Box::new([
            MemEntry::new(Box::leak(Box::new(Dummy)), Word::MIN, Word::ONE << 3),
            MemEntry::new(Box::leak(Box::new(Dummy)), Word::MIN + (Word::ONE << 4), 17isize.try_into().unwrap()),
            MemEntry::new(Box::leak(Box::new(Dummy)), Word::ZERO, Word::ONE << 3),
            MemEntry::new(Box::leak(Box::new(Dummy)), Word::ZERO + (Word::ONE << 4), Word::ONE << 3),
            MemEntry::new(Box::leak(Box::new(Dummy)), Word::MAX - (Word::ONE << 4), Word::ONE << 3),
            MemEntry::new(Box::leak(Box::new(Dummy)), Word::MAX, Word::ZERO),
        ]));

        let mem = Memory { memspace } ;

        assert!(Word::MIN < (Word::MIN + Word::ONE));

        assert!(mem.find_device_index(Word::MIN + Word::ONE).is_some());
        assert!(mem.find_device_index(Word::MIN + (Word::ONE << 3)).is_none());
        assert!(mem.find_device_index(Word::MIN + (Word::ONE << 3) - Word::ONE).is_some());
    }
}
