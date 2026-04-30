pub mod header;

#[derive(Debug)]
pub enum ElfErr {
    Uninit,
    BadMagic,
    BadVersion(u8),
    BadClass,
    BadEndianness,
    BadSize,
    BadHeader,
    BadFormat,
    BadSectionType,
    BadArg,
    BadIndex,
    NotFound,
    BufferOverflow,
    IoEof,
    IoError,
    NoMem,
}
pub trait ElfReader {
    fn read(&mut self, offset: u64, buf: &mut [u8]) -> Result<(), ElfErr>;
}
