/*
 * Copyright (c) 2026 Hugo Cebrecos
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! TODO: docs and revise a lot, with the standard

#![allow(non_snake_case)]

pub mod header {
    pub type ElfType = u16;
    pub mod ET {
        use super::ElfType;

        pub const NONE: ElfType = 0; // No type
        pub const REL: ElfType = 1; // relocatable
        pub const EXEC: ElfType = 2; // executable
        pub const DYN: ElfType = 3; // shared object
        pub const CORE: ElfType = 4; // Core file
        pub const LOOS: ElfType = 0xfe00; // start of OS-specific range

        /* You may add your application's OS-specific types here */

        pub const HIOS: ElfType = 0xfeff; //   end of OS-specific range
        pub const LOPROC: ElfType = 0xff00; // start of Processor-specific range

        /* You may add your application's processor-specific types here */

        pub const HIPROC: ElfType = 0xffff; //   end of Processor-specific range
    }

    /// Represents the ELF version of a file.
    ///
    /// This is a 32-bit unsigned integer (`u32`) identifying the version
    /// of the ELF specification used. Most files use `EV::CURRENT`.
    pub type ElfVersion = u32;

    /// Constants for valid [`ElfVersion`] values.
    ///
    /// Use these constants instead of raw numbers for clarity.
    pub mod EV {
        use super::ElfVersion;

        pub const NONE: ElfVersion = 0; // Invalid
        pub const CURRENT: ElfVersion = 1;
    }

    pub type ElfMachine = u16;
    pub mod EM {
        use super::ElfMachine;

        pub const NONE: ElfMachine = 0;

        /* You may add your application-specific machines here */
    }

    // TODO: then extending machines and things is:
    //    use crate::ElfMachine;
    //    pub const EM_ARM: ElfMachine = 40;

    pub type EiClass = u8;
    pub mod ELFCLASS {
        use super::EiClass;

        pub const NONE: EiClass = 0;
        pub const CLASS_32: EiClass = 1;
        pub const CLASS_64: EiClass = 2;
    }

    pub type EiData = u8;
    pub mod ELFDATA {
        use super::EiData;

        pub const NONE: EiData = 0;
        pub const LSB: EiData = 1;
        pub const MSB: EiData = 2;
    }

    pub type ElfABI = u8;
    pub mod ELFOSABI {
        use super::ElfABI;

        pub const NONE: ElfABI = 0; // This is the default value for most linkers

        /* You may add your application-specific ABIs here */
    }

    /// Abstract representation of the ELF header, it does not represent the real layout, instead provides a uniform view into the data.
    /// Abstract representation of the ELF header.
    /// Does not represent the real layout, instead provides a uniform view into the data.
    pub struct ElfHeader {
        /// 32 or 64 bit architecture
        pub ei_class: EiClass,

        /// Endianness of the architecture
        pub ei_data: EiData,

        /// Target platform's ABI
        pub ei_os_abi: ElfABI,

        /// Target ABI version
        pub ei_abi_ver: ElfVersion,

        /// Padding
        pub padding: [u8; 7],

        /// Type of ELF file
        pub elf_type: ElfType,

        /// Architecture
        pub machine: ElfMachine,

        /// Always 1
        pub version: ElfVersion,

        /// Entry point (virtual address)
        pub entry: u64,

        /// Offset of program header table in the file
        pub pro_hdr_off: u64,

        /// Offset of section header table in the file
        pub sec_hdr_off: u64,

        /// Flags
        pub flags: u32,

        /// Size of this header
        pub hdr_size: u16,

        /// Size of one entry in program header table
        pub ph_entry_sz: u16,

        /// Number of entries in program header table
        pub ph_entry_num: u16,

        /// Size of one entry in section header table
        pub sh_entry_sz: u16,

        /// Number of entries in section header table
        pub sh_entry_num: u16,

        /// Index of section name string table
        pub sec_str_idx: u16,
    }

    impl From<(&crate::repr::Elf32Hdr, &crate::repr::ElfInfo)> for ElfHeader {
        fn from(value: (&crate::repr::Elf32Hdr, &crate::repr::ElfInfo)) -> Self {
          let (hdr, info) = value;
            Self {
                ei_class: info.ei_class,
                ei_data: info.ei_data,
                ei_os_abi: info.ei_os_abi,
                ei_abi_ver: info.ei_abi_version as u32,
                padding: info.pad,
                elf_type: hdr.e_type,
                machine: hdr.e_machine,
                version: hdr.e_version,
                entry: hdr.e_entry as u64,
                pro_hdr_off: hdr.e_phoff as u64,
                sec_hdr_off: hdr.e_shoff as u64,
                flags: hdr.e_flags,
                hdr_size: hdr.e_ehsize,
                ph_entry_sz: hdr.e_phentsize,
                ph_entry_num: hdr.e_phnum,
                sh_entry_sz: hdr.e_shentsize,
                sh_entry_num: hdr.e_shnum,
                sec_str_idx: hdr.e_shstrndx,
            }
        }
    }
    impl From<(&crate::repr::Elf64Hdr, &crate::repr::ElfInfo)> for ElfHeader {
        fn from(value: (&crate::repr::Elf64Hdr, &crate::repr::ElfInfo)) -> Self {
              let (hdr, info) = value;
            Self {
                ei_class: info.ei_class,
                ei_data: info.ei_data,
                ei_os_abi: info.ei_os_abi,
                ei_abi_ver: info.ei_abi_version as u32,
                padding: info.pad,
                elf_type: hdr.e_type,
                machine: hdr.e_machine,
                version: hdr.e_version,
                entry: hdr.e_entry,
                pro_hdr_off: hdr.e_phoff,
                sec_hdr_off: hdr.e_shoff,
                flags: hdr.e_flags,
                hdr_size: hdr.e_ehsize,
                ph_entry_sz: hdr.e_phentsize,
                ph_entry_num: hdr.e_phnum,
                sh_entry_sz: hdr.e_shentsize,
                sh_entry_num: hdr.e_shnum,
                sec_str_idx: hdr.e_shstrndx,
            }
        }
    }
}

pub mod section {
    pub type ElfSectionType = u32;
    pub mod SHT {
        use super::ElfSectionType;

        pub const NULL: ElfSectionType = 0;
        pub const PROGBITS: ElfSectionType = 1;
        pub const SYMTAB: ElfSectionType = 2;
        pub const STRTAB: ElfSectionType = 3;
        pub const RELA: ElfSectionType = 4;
        pub const HASH: ElfSectionType = 5;
        pub const DYNAMIC: ElfSectionType = 6;
        pub const NOTE: ElfSectionType = 7;
        pub const NOBITS: ElfSectionType = 8;
        pub const REL: ElfSectionType = 9;
        pub const SHLIB: ElfSectionType = 10;
        pub const DYNSYM: ElfSectionType = 11;
        pub const INIT_ARRAY: ElfSectionType = 14;
        pub const FINI_ARRAY: ElfSectionType = 15;
        pub const PREINIT_ARRAY: ElfSectionType = 16;
        pub const GROUP: ElfSectionType = 17;
        pub const SYMTAB_SHNDX: ElfSectionType = 18;
        pub const RELR: ElfSectionType = 19;
        pub const LOOS: ElfSectionType = 0x60000000;

        /* You may add your application's OS-specific types here */

        pub const HIOS: ElfSectionType = 0x6fffffff;
        pub const LOPROC: ElfSectionType = 0x70000000;

        /* You may add your application's processor-specific types here */

        pub const HIPROC: ElfSectionType = 0x7fffffff;
        pub const LOUSER: ElfSectionType = 0x80000000;
        pub const HIUSER: ElfSectionType = 0xffffffff;
    }

    pub type ElfSectionFlag = u64;
    pub mod SHF {
        use super::ElfSectionFlag;

        pub const WRITE: ElfSectionFlag = 0x1;
        pub const ALLOC: ElfSectionFlag = 0x2;
        pub const EXECINSTR: ElfSectionFlag = 0x4;
        pub const MERGE: ElfSectionFlag = 0x10;
        pub const STRINGS: ElfSectionFlag = 0x20;
        pub const INFO_LINK: ElfSectionFlag = 0x40;
        pub const LINK_ORDER: ElfSectionFlag = 0x80;
        pub const OS_NONCONFORMING: ElfSectionFlag = 0x100;
        pub const GROUP: ElfSectionFlag = 0x200;
        pub const TLS: ElfSectionFlag = 0x400;
        pub const COMPRESSED: ElfSectionFlag = 0x800;
        pub const MASKOS: ElfSectionFlag = 0x0ff00000;

        /* You may add your application's OS-specific flags here */

        pub const MASKPROC: ElfSectionFlag = 0xf0000000;

        /* You may add your application's processor-specific flags here */
    }

    pub type SecGrpFlag = u64;
    pub mod GRP {
        use super::SecGrpFlag;

        pub const COMDAT: SecGrpFlag = 0x1;
        pub const MASKOS: SecGrpFlag = 0x0ff00000;

        /* You may add your application's OS-specific flags here */

        pub const MASKPROC: SecGrpFlag = 0xf0000000;

        /* You may add your application's processor-specific flags here */
    }

    /// Abstract representation of an ELF section header.
    pub struct ElfSecHeader {
        /// Index into the section header string table section.
        pub name_idx: u32,

        /// Type of section (e.g., SHT_PROGBITS, SHT_SYMTAB, etc.).
        pub section_type: ElfSectionType,

        /// Flags describing section properties (e.g., SHF_WRITE, SHF_ALLOC).
        pub flags: u64,

        /// If the section is in the memory image of a process, this is the first address.
        pub address: u64,

        /// Section is stored at `<offset>` from the beginning of this file.
        pub offset: u64,

        /// Size of the section in bytes.
        pub size: u64,

        /// Index in the section header table of an associated section (e.g., for linking).
        pub link: u32,

        /// Additional section information (depends on section type).
        pub info: u32,

        /// Alignment constraints of the address field.
        pub alignment: u64,

        /// If the section is a table of fixed-size entries, this is the size of one entry.
        pub entry_size: u64,
    }
}

pub mod symbol {
    pub type ElfSymbolType = u8;
    pub mod STT {
        use super::ElfSymbolType;

        pub const NOTYPE: ElfSymbolType = 0;
        pub const OBJECT: ElfSymbolType = 1;
        pub const FUNC: ElfSymbolType = 2;
        pub const SECTION: ElfSymbolType = 3;
        pub const FILE: ElfSymbolType = 4;
        pub const COMMON: ElfSymbolType = 5;
        pub const TLS: ElfSymbolType = 6;
        pub const LOOS: ElfSymbolType = 10;
        pub const HIOS: ElfSymbolType = 12;
        pub const LOPROC: ElfSymbolType = 13;
        pub const HIPROC: ElfSymbolType = 15;
    }

    pub type ElfSymbolBind = u8;
    pub mod STB {
        use super::ElfSymbolBind;

        pub const LOCAL: ElfSymbolBind = 0;
        pub const GLOBAL: ElfSymbolBind = 1;
        pub const WEAK: ElfSymbolBind = 2;
        pub const LOOS: ElfSymbolBind = 10;
        pub const HIOS: ElfSymbolBind = 12;
        pub const LOPROC: ElfSymbolBind = 13;
        pub const HIPROC: ElfSymbolBind = 15;
    }

    pub type ElfSymbolVis = u8;
    pub mod STV {
        use super::ElfSymbolVis;

        pub const DEFAULT: ElfSymbolVis = 0;
        pub const INTERNAL: ElfSymbolVis = 1;
        pub const HIDDEN: ElfSymbolVis = 2;
        pub const PROTECTED: ElfSymbolVis = 3;
        pub const EXPORTED: ElfSymbolVis = 4;
        pub const SINGLETON: ElfSymbolVis = 5;
        pub const ELIMINATE: ElfSymbolVis = 6;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ElfSymbolAttr {
        Default,
        Abs,
        Common,
        Undef,
        Unknown,
    }

    pub struct ElfSymTabEntry {
        pub name_idx: u32,
        pub sym_type: ElfSymbolType,
        pub binding: ElfSymbolBind,
        pub visib: ElfSymbolVis,
        pub attr: ElfSymbolAttr,
        pub sec_idx: u64,
        pub value: u64,
        pub size: u64,
    }
}

pub mod segment {
    pub type ElfSegmentType = u32;
    pub mod PT {
        use super::ElfSegmentType;

        pub const NULL: ElfSegmentType = 0;
        pub const LOAD: ElfSegmentType = 1;
        pub const DYNAMIC: ElfSegmentType = 2;
        pub const INTERP: ElfSegmentType = 3;
        pub const NOTE: ElfSegmentType = 4;
        pub const SHLIB: ElfSegmentType = 5;
        pub const PHDR: ElfSegmentType = 6;
        pub const TLS: ElfSegmentType = 7;
        pub const LOOS: ElfSegmentType = 0x60000000;
        pub const HIOS: ElfSegmentType = 0x6fffffff;
        pub const LOPROC: ElfSegmentType = 0x70000000;
        pub const HIPROC: ElfSegmentType = 0x7fffffff;
    }

    pub type ElfSegmentPerms = u32;
    pub mod PF {
        use super::ElfSegmentPerms;

        pub const X: ElfSegmentPerms = 1;
        pub const W: ElfSegmentPerms = 2;
        pub const R: ElfSegmentPerms = 4;
        pub const MASKOS: ElfSegmentPerms = 0x0ff00000;
        pub const MASKPROC: ElfSegmentPerms = 0xf0000000;
    }

    pub struct ElfProHeader {
        pub seg_type: ElfSegmentType,
        pub flags: u32,
        pub offset: u64,
        pub phy_address: u64,
        pub vir_address: u64,
        pub file_size: u64,
        pub mem_size: u64,
        pub alignment: u64,
    }
}

#[derive(Debug)]
pub enum ElfErr {
    Uninit,
    BadMagic,
    BadVersion,
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
    fn read(&mut self, offset: usize, buf: &mut [u8]) -> Result<(), ElfErr>;
}
