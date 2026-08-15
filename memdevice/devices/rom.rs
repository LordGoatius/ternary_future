use crate::{Addr, MemEntry, MemoryDevice, Tryte, Word};

#[derive(Debug)]
pub struct Rom<const SIZE: usize> {
    data: [Word; SIZE],
}

impl<const SIZE: usize> Rom<SIZE> {
    pub const fn new(data: [Word; SIZE]) -> Self {
        Self { data }
    }

    pub const fn to_mementry(&'static mut self, base: Word) -> MemEntry {
        let Ok(size) = (SIZE as isize * 3).try_into() else {
            panic!()
        };
        MemEntry {
            dev: self,
            base,
            size,
        }
    }
}

impl<const SIZE: usize> MemoryDevice for Rom<SIZE> {
    fn write_tryte(&mut self, _offset: Addr, _value: Tryte) {
        return;
    }

    fn write_word(&mut self, _offset: Addr, _value: Word) {
        return;
    }

    fn read_tryte(&mut self, offset: Addr) -> Tryte {
        let offset: isize = offset.into();
        assert!(offset > 0);
        let data_ind = offset.div_euclid(3) as usize;
        let word_ind = offset.rem_euclid(3) as u32;

        let word = self.data[data_ind];
        return word.slice(word_ind * 9);
    }

    fn read_word(&mut self, offset: Addr) -> Word {
        let offset: isize = offset.into();
        assert!(offset >= 0);
        let data_ind = offset.div_euclid(3) as usize;
        let word_ind = offset.rem_euclid(3) as u32;
        assert!(word_ind == 0);

        self.data[data_ind]
    }
}
