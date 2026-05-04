use core::{
    fmt::Display,
    ops::{BitAnd, BitOr},
};

use binlayout::BinLayout;

#[derive(Debug, PartialEq)]
pub enum SegmentType {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,

    Reserved(u32),
    Os(u32),
    Proc(u32),
}
impl SegmentType {
    pub const NULL: u32 = 0;
    pub const LOAD: u32 = 1;
    pub const DYNAMIC: u32 = 2;
    pub const INTERP: u32 = 3;
    pub const NOTE: u32 = 4;
    pub const SHLIB: u32 = 5;
    pub const PHDR: u32 = 6;
    pub const TLS: u32 = 7;

    pub const LOOS: u32 = 0x6000_0000;

    /* You may add your application's OS-specific types here */

    pub const HIOS: u32 = 0x6fff_ffff;
    pub const LOPROC: u32 = 0x7000_0000;

    /* You may add your application's processor-specific types here */

    pub const HIPROC: u32 = 0x7fff_ffff;
}
impl From<u32> for SegmentType {
    fn from(value: u32) -> Self {
        match value {
            SegmentType::NULL => Self::Null,
            SegmentType::LOAD => Self::Load,
            SegmentType::DYNAMIC => Self::Dynamic,
            SegmentType::INTERP => Self::Interp,
            SegmentType::NOTE => Self::Note,
            SegmentType::SHLIB => Self::Shlib,
            SegmentType::PHDR => Self::Phdr,
            SegmentType::TLS => Self::Tls,

            SegmentType::LOOS..=SegmentType::HIOS => Self::Os(value),
            SegmentType::LOPROC..=SegmentType::HIPROC => Self::Proc(value),

            _ => Self::Reserved(value),
        }
    }
}
impl Display for SegmentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Null => write!(f, "NULL"),
            Self::Load => write!(f, "LOAD"),
            Self::Dynamic => write!(f, "DYNAMIC"),
            Self::Interp => write!(f, "INTERP"),
            Self::Note => write!(f, "NOTE"),
            Self::Shlib => write!(f, "SHLIB"),
            Self::Phdr => write!(f, "PHDR"),
            Self::Tls => write!(f, "TLS"),

            Self::Os(x) => write!(f, "OS specific ({:x})", x),
            Self::Proc(x) => write!(f, "Proc specific ({:x})", x),
            Self::Reserved(x) => write!(f, "Reserved ({:x})", x),
        }
    }
}

pub mod elf_segment_flags {
    pub const X: u32 = 1;
    pub const W: u32 = 2;
    pub const R: u32 = 4;
    pub const MASKOS: u32 = 0x0ff00000;
    pub const MASKPROC: u32 = 0xf0000000;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SegmentFlags(u32);

impl SegmentFlags {
    pub const X: Self = Self(elf_segment_flags::X);
    pub const W: Self = Self(elf_segment_flags::W);
    pub const R: Self = Self(elf_segment_flags::R);

    pub const MASK_OS: Self = Self(elf_segment_flags::MASKOS);
    pub const MASK_PROC: Self = Self(elf_segment_flags::MASKPROC);

    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u32 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}
impl From<u32> for SegmentFlags {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl BitOr for SegmentFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl BitAnd for SegmentFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl Display for SegmentFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}{}{}",
            if self.contains(Self::R) { "R" } else { " " },
            if self.contains(Self::W) { "W" } else { " " },
            if self.contains(Self::X) { "E" } else { " " },
        )
    }
}

#[repr(C)]
#[derive(Debug, BinLayout)]
pub struct Elf32ProHdr {
    pub p_type: u32,   // Type of segment
    pub p_offset: u32, // Segment is stored at <offset> from the beginning of this file
    pub p_vaddr: u32,  // Virtual address of this segment in memory
    pub p_paddr: u32,  // Physical address, only relevant on some systems
    pub p_filesz: u32, // Size of the segment in this file
    pub p_memsz: u32,  // Size of the segment in the memory image
    pub p_flags: u32,
    pub p_align: u32, // Alignment constraints of address fields
}

#[repr(C)]
#[derive(Debug, BinLayout)]
pub struct Elf64ProHdr {
    pub p_type: u32, // Type of segment
    pub p_flags: u32,
    pub p_offset: u64, // Segment is stored at <offset> from the beginning of this file
    pub p_vaddr: u64,  // Virtual address of this segment in memory
    pub p_paddr: u64,  // Physical address, only relevant on some systems
    pub p_filesz: u64, // Size of the segment in this file
    pub p_memsz: u64,  // Size of the segment in the memory image
    pub p_align: u64,  // Alignment constraints of address fields
}

#[derive(Debug)]
pub struct ProgramHeader {
    pub seg_type: SegmentType, // Type of segment
    pub flags: SegmentFlags,
    pub offset: u64, // Segment is stored at <offset> from the beginning of this file
    pub phy_addr: u64, // Physical address, only relevant on some systems
    pub vir_addr: u64, // Virtual address of this segment in memory
    pub file_size: u64, // Size of the segment in this file
    pub mem_size: u64, // Size of the segment in the memory image
    pub alignment: u64, // Alignment constraints of address fields
}

impl From<&Elf32ProHdr> for ProgramHeader {
    fn from(hdr: &Elf32ProHdr) -> Self {
        Self {
            seg_type: hdr.p_type.into(),
            flags: hdr.p_flags.into(),
            offset: hdr.p_offset as u64,
            vir_addr: hdr.p_vaddr as u64,
            phy_addr: hdr.p_paddr as u64,
            file_size: hdr.p_filesz as u64,
            mem_size: hdr.p_memsz as u64,
            alignment: hdr.p_align as u64,
        }
    }
}

impl From<&Elf64ProHdr> for ProgramHeader {
    fn from(hdr: &Elf64ProHdr) -> Self {
        Self {
            seg_type: hdr.p_type.into(),
            flags: hdr.p_flags.into(),
            offset: hdr.p_offset,
            vir_addr: hdr.p_vaddr,
            phy_addr: hdr.p_paddr,
            file_size: hdr.p_filesz,
            mem_size: hdr.p_memsz,
            alignment: hdr.p_align,
        }
    }
}
