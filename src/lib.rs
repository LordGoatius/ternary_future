// I will pay for my hubris one day
#![expect(incomplete_features)]
#![feature(generic_const_exprs,
           const_default,
           derive_const,
           const_trait_impl)]

mod tern_next;
pub mod helper;


use ternary::{tryte::Tryte, word::Word};

pub type Addr = Word;
pub type Memory = &'static [MemEntry];

#[expect(unused)]
pub struct MemEntry {
    dev: &'static dyn MemoryDevice,
    base: Addr,
    size: Word,
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
#[expect(unused)]
pub trait MemoryDevice {
    fn write_tryte(&mut self, offset: Addr, value: Tryte);
    fn write_word(&mut self, offset: Addr, value: Word);

    fn read_tryte(&mut self, offset: Addr, value: Tryte);
    fn read_word(&mut self, offset: Addr, value: Word);

    /// Default impl that calls [`MemoryDevice::write_tryte`] in a loop
    fn write_trytes(&mut self, offset: Addr, value: &[Tryte]) { todo!() }
    /// Default impl that calls [`MemoryDevice::write_word`] in a loop
    fn write_words(&mut self, offset: Addr, value: &[Word]) { todo!() }

    // NOTE: Remove these? We can write in a loop, but when reading we can't
    // assume the internal representation can deref into a slice
    fn read_trytes(&mut self, offset: Addr) -> &[Tryte] { todo!() }
    fn read_words(&mut self, offset: Addr) -> &[Word] { todo!() }
}

pub struct Ram();


#[cfg(test)]
mod tests {
}
