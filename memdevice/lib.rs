// I will pay for my hubris one day
#![expect(incomplete_features)]
#![allow(clippy::doc_lazy_continuation)]
#![feature(generic_const_exprs, const_trait_impl, const_cmp, const_ops, const_convert)]

use std::cmp::Ordering;
use std::fmt::Debug;

use balanced_ternary::Ternary;

pub mod devices;

type Word  = Ternary<27>;
type Tryte = Ternary<9>;

pub type Addr = Word;

unsafe impl Sync for MemEntry {}

#[derive(Debug)]
pub struct MemEntry {
    dev: &'static mut dyn MemoryDevice,
    pub base: Addr,
    pub size: Word,
}

impl MemEntry {
    pub const fn new(dev: &'static mut dyn MemoryDevice, base: Addr, size: Addr) -> Self {
        MemEntry { dev, base, size }
    }

    pub const fn valid_addr(&self, addr: Addr) -> Ordering {
        match addr.cmp(&self.base) {
            Ordering::Less    => Ordering::Greater,
            Ordering::Equal   => Ordering::Equal,
            Ordering::Greater => {
                if addr < (self.base + self.size) {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            },
        }
    }

    pub fn write_tryte(&mut self, offset: Addr, value: Tryte) {
        self.dev.write_tryte(offset, value);
    }

    pub fn write_word(&mut self, offset: Addr, value: Word) {
        self.dev.write_word(offset, value);
    }

    pub fn read_tryte(&self, offset: Addr) -> Tryte {
        self.dev.read_tryte(offset)
    }

    pub fn read_word(&self, offset: Addr) -> Word {
        self.dev.read_word(offset)
    }
}

/// MEMORY_DEVICE_MODEL:
/// - Devices are indexed using OFFSETS
/// - MEMORY is indexed using ADDRESSES
/// - Indexing into a device subtracts the
///   index address from the base address,
///   calculating an offset
/// MMIO:
/// - MMIO requires that reads may have side effects, and therefore
///   reads and writes must take in `&mut self`. That may need to be
///   changed to internal mutability managed by the [`MemoryDevice`]
///   itself.
/// QUESTIONS:
/// - DMA?
/// - Unaligned accesses for devices?
#[expect(unused)]
pub const trait MemoryDevice: Debug {
    fn write_tryte(&mut self, offset: Addr, value: Tryte);
    fn write_word(&mut self, offset: Addr, value: Word);

    fn read_tryte(&self, offset: Addr) -> Tryte;
    fn read_word(&self, offset: Addr) -> Word;

    /// Default impl calls [`MemoryDevice::write_tryte`] in a loop
    fn write_trytes(&mut self, offset: Addr, value: &[Tryte]) {
        todo!()
    }
    /// Default impl calls [`MemoryDevice::write_word`] in a loop
    fn write_words(&mut self, offset: Addr, value: &[Word]) {
        todo!()
    }

    // NOTE: Remove these? We can write in a loop, but when reading we can't
    // assume the internal representation can deref into a slice.
    // In fact it's nearly guaranteed it *cannot* be unless I pull some
    // *really* insane paging tricks somehow.
    fn read_trytes(&mut self, offset: Addr) -> &[Tryte] {
        todo!()
    }
    fn read_words(&mut self, offset: Addr) -> &[Word] {
        todo!()
    }
}

pub struct Ram();

#[cfg(test)]
mod tests {}
